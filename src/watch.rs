use std::collections::HashSet;
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

use anyhow::Result;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};

use crate::formatters;
use crate::scanner::Scanner;

const DEBOUNCE_MS: u64 = 200;

pub fn watch(scanner: &Scanner) -> Result<()> {
    let dirs = resolve_watch_dirs(&scanner.files);
    if dirs.is_empty() {
        eprintln!("Nothing to watch. Specify a directory or file pattern.");
        return Ok(());
    }

    let (tx, rx) = mpsc::channel::<Vec<String>>();

    let mut watcher = RecommendedWatcher::new(
        move |res: Result<Event, notify::Error>| {
            if let Ok(event) = res {
                let paths: Vec<String> = event
                    .paths
                    .iter()
                    .filter_map(|p| {
                        let ext = p.extension().and_then(|e| e.to_str())?;
                        if matches!(ext, "js" | "jsx" | "ts" | "tsx") {
                            Some(p.to_string_lossy().to_string())
                        } else {
                            None
                        }
                    })
                    .collect();
                if !paths.is_empty() {
                    let _ = tx.send(paths);
                }
            }
        },
        Config::default(),
    )?;

    for dir in &dirs {
        watcher.watch(dir, RecursiveMode::Recursive)?;
    }

    eprintln!(
        "Watching {} director(ies) for changes. Ctrl+C to stop.",
        dirs.len()
    );

    run_scan_loop(scanner, &rx)
}

fn run_scan_loop(scanner: &Scanner, rx: &mpsc::Receiver<Vec<String>>) -> Result<()> {
    loop {
        let mut changed = HashSet::new();

        match rx.recv() {
            Ok(paths) => {
                for p in paths {
                    changed.insert(p);
                }
            }
            Err(_) => break,
        }

        loop {
            match rx.recv_timeout(Duration::from_millis(DEBOUNCE_MS)) {
                Ok(paths) => {
                    for p in paths {
                        changed.insert(p);
                    }
                }
                Err(mpsc::RecvTimeoutError::Timeout) => break,
                Err(mpsc::RecvTimeoutError::Disconnected) => return Ok(()),
            }
        }

        let changed: Vec<String> = changed.into_iter().collect();

        if changed.is_empty() {
            continue;
        }

        eprint!("\r[{} file(s) changed] Scanning...", changed.len());

        match scanner.scan_paths(&changed) {
            Ok(results) => {
                let formatter = formatters::get_formatter("stylish");
                let output = formatter.format(&results, false);
                print!("{output}");
            }
            Err(e) => {
                eprintln!("\rScan error: {e}");
            }
        }
    }

    Ok(())
}

fn resolve_watch_dirs(patterns: &[String]) -> Vec<Box<Path>> {
    let mut dirs = Vec::new();
    for pattern in patterns {
        let path = Path::new(pattern);
        if path.is_dir() {
            dirs.push(path.into());
        } else if path.is_file() {
            if let Some(parent) = path.parent() {
                dirs.push(parent.into());
            }
        } else {
            if let Some(parent) = path.parent() {
                if parent.to_string_lossy().is_empty() || parent == Path::new(".") {
                    if let Ok(cwd) = std::env::current_dir() {
                        dirs.push(cwd.into());
                    }
                } else if parent.is_dir() {
                    dirs.push(parent.into());
                }
            }
        }
    }
    dirs.sort();
    dirs.dedup();
    dirs
}
