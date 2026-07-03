use std::path::Path;

pub fn install_pre_commit_hook() -> anyhow::Result<()> {
    let git_dir = Path::new(".git");
    if !git_dir.exists() {
        anyhow::bail!("No `.git` directory found — are you in a git repository?");
    }

    let hooks_dir = git_dir.join("hooks");
    if !hooks_dir.exists() {
        std::fs::create_dir_all(&hooks_dir)?;
    }

    let hook_path = hooks_dir.join("pre-commit");

    if hook_path.exists() {
        let existing = std::fs::read_to_string(&hook_path)?;
        if existing.contains("react-auditor") {
            eprintln!(
                "react-auditor pre-commit hook already installed at {}",
                hook_path.display()
            );
            return Ok(());
        }
        eprintln!(
            "A pre-commit hook already exists at {}. Appending react-auditor check.",
            hook_path.display()
        );
    }

    let hook_content = r#"#!/bin/sh
# react-auditor pre-commit hook — scan staged JS/TS/React files
FILES=$(git diff --cached --name-only --diff-filter=ACM -- '*.js' '*.jsx' '*.ts' '*.tsx')
if [ -n "$FILES" ]; then
    echo "$FILES" | xargs react-auditor --max-warnings 0
    if [ $? -ne 0 ]; then
        echo "react-auditor: fix violations before committing"
        exit 1
    fi
fi
"#;

    std::fs::write(&hook_path, hook_content)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&hook_path, std::fs::Permissions::from_mode(0o755))?;
    }

    eprintln!(
        "installed react-auditor pre-commit hook at {}",
        hook_path.display()
    );
    Ok(())
}
