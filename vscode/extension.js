const vscode = require('vscode');
const { execFile } = require('child_process');
const { promisify } = require('util');
const fs = require('fs');
const path = require('path');

const execFileAsync = promisify(execFile);

const SEVERITY_MAP = {
  error: vscode.DiagnosticSeverity.Error,
  warning: vscode.DiagnosticSeverity.Warning,
};

let statusBarItem;
let diagnosticCollection;
let decorationTypes = {};

function activate(context) {
  diagnosticCollection = vscode.languages.createDiagnosticCollection('react-auditor');
  context.subscriptions.push(diagnosticCollection);

  statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left, 100);
  statusBarItem.text = '$(check) React Auditor';
  statusBarItem.command = 'react-auditor.run';
  statusBarItem.tooltip = 'Click to run React Auditor on current file';
  statusBarItem.show();
  context.subscriptions.push(statusBarItem);

  let debounceTimer;

  function debouncedRun(doc) {
    if (doc.languageId !== 'javascript' && doc.languageId !== 'javascriptreact' &&
        doc.languageId !== 'typescript' && doc.languageId !== 'typescriptreact') {
      return;
    }
    clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => runAuditor(doc), 500);
  }

  const onChange = vscode.workspace.onDidChangeTextDocument((e) => {
    const config = vscode.workspace.getConfiguration('reactAuditor');
    if (config.get('runOnChange', false)) {
      debouncedRun(e.document);
    }
  });
  context.subscriptions.push(onChange);

  const onSave = vscode.workspace.onDidSaveTextDocument((doc) => {
    const config = vscode.workspace.getConfiguration('reactAuditor');
    if (config.get('runOnSave', true)) {
      debouncedRun(doc);
    }
  });
  context.subscriptions.push(onSave);

  const onOpen = vscode.workspace.onDidOpenTextDocument((doc) => {
    const config = vscode.workspace.getConfiguration('reactAuditor');
    if (config.get('runOnOpen', false)) {
      debouncedRun(doc);
    }
  });
  context.subscriptions.push(onOpen);

  context.subscriptions.push(vscode.commands.registerCommand('react-auditor.run', async () => {
    const editor = vscode.window.activeTextEditor;
    if (editor) {
      await runAuditor(editor.document);
    }
  }));

  context.subscriptions.push(vscode.commands.registerCommand('react-auditor.runWorkspace', async () => {
    const files = await vscode.workspace.findFiles(
      '**/*.{js,jsx,ts,tsx}',
      '**/node_modules/**'
    );

    await vscode.window.withProgress({
      location: vscode.ProgressLocation.Notification,
      title: 'React Auditor: scanning workspace...',
      cancellable: true,
    }, async (progress, token) => {
      const total = Math.min(files.length, 50);
      for (let i = 0; i < total; i++) {
        if (token.isCancellationRequested) break;
        progress.report({ message: `${i + 1}/${total}`, increment: 100 / total });
        const doc = await vscode.workspace.openTextDocument(files[i]);
        await runAuditor(doc);
      }
    });

    vscode.window.showInformationMessage(`React Auditor: scanned ${Math.min(files.length, 50)} files`);
  }));

  context.subscriptions.push(vscode.commands.registerCommand('react-auditor.clear', () => {
    diagnosticCollection.clear();
    clearDecorations();
    statusBarItem.text = '$(check) React Auditor';
  }));

  context.subscriptions.push(vscode.commands.registerCommand('react-auditor.configure', () => {
    ConfigWebview.createOrShow(context.extensionUri);
  }));

  context.subscriptions.push(vscode.commands.registerCommand('react-auditor.disableRule', (ruleId) => {
    const config = vscode.workspace.getConfiguration('reactAuditor');
    const wsFolders = vscode.workspace.workspaceFolders;
    if (!wsFolders) return;
    const rcPath = path.join(wsFolders[0].uri.fsPath, '.rauditrc.toml');
    if (!fs.existsSync(rcPath)) {
      fs.writeFileSync(rcPath, '# React Auditor Configuration\n');
    }
    fs.appendFileSync(rcPath, `\n"${ruleId}" = "off"\n`);
    vscode.window.showInformationMessage(`Rule "${ruleId}" disabled in .rauditrc.toml`);
  }));

  context.subscriptions.push(vscode.commands.registerCommand('react-auditor.fixFile', async (uri) => {
    const filePath = uri ? uri.fsPath : vscode.window.activeTextEditor?.document.uri.fsPath;
    if (!filePath) return;

    const config = vscode.workspace.getConfiguration('reactAuditor');
    const binaryPath = config.get('binaryPath', 'react-auditor');

    statusBarItem.text = '$(sync~spin) React Auditor: fixing...';
    try {
      const { stdout } = await execFileAsync(binaryPath, [
        filePath, '--fix', '--format', 'json',
      ], { timeout: 30000, maxBuffer: 10 * 1024 * 1024 });
      const msg = (stdout || '').trim();
      if (msg) {
        vscode.window.showInformationMessage(`React Auditor: ${msg}`);
      }
      // Re-scan after fix
      if (vscode.window.activeTextEditor?.document.uri.fsPath === filePath) {
        await runAuditor(vscode.window.activeTextEditor.document);
      }
    } catch (err) {
      // The fix command may still have applied fixes before exiting non-zero.
      // Try to show its output before surfacing the error.
      const msg = (err.stdout || err.stderr || '').toString().trim() || 'Fixes applied. Re-scan to verify.';
      vscode.window.showInformationMessage(`React Auditor: ${msg}`);
      if (vscode.window.activeTextEditor?.document.uri.fsPath === filePath) {
        await runAuditor(vscode.window.activeTextEditor.document);
      }
    } finally {
      statusBarItem.text = '$(check) React Auditor';
    }
  }));

  // Register code action provider for quick fixes
  context.subscriptions.push(
    vscode.languages.registerCodeActionsProvider(
      ['javascript', 'javascriptreact', 'typescript', 'typescriptreact'],
      new ReactAuditorFixProvider(),
      { providedCodeActionKinds: [vscode.CodeActionKind.QuickFix] }
    )
  );

  // Update decorations when active editor changes
  context.subscriptions.push(
    vscode.window.onDidChangeActiveTextEditor((editor) => {
      if (editor && diagnosticCollection.has(editor.document.uri)) {
        updateDecorations(editor);
      } else {
        clearDecorations();
      }
    })
  );

  // Update decorations when diagnostics change
  diagnosticCollection.onDidChangeDecorations?.(() => {
    applyDecorations();
  });
}

class ReactAuditorFixProvider {
  provideCodeActions(document, range, context) {
    const actions = [];
    for (const diagnostic of context.diagnostics) {
      if (diagnostic.source === 'react-auditor') {
        const fixAction = new vscode.CodeAction(
          `Fix: ${diagnostic.code || 'this issue'}`,
          vscode.CodeActionKind.QuickFix
        );
        fixAction.command = {
          command: 'react-auditor.fixFile',
          title: 'Fix with react-auditor',
          arguments: [document.uri],
        };
        fixAction.diagnostics = [diagnostic];
        fixAction.isPreferred = true;
        actions.push(fixAction);

        const disableAction = new vscode.CodeAction(
          `Disable rule: ${diagnostic.code}`,
          vscode.CodeActionKind.QuickFix
        );
        disableAction.command = {
          command: 'react-auditor.disableRule',
          title: 'Disable rule',
          arguments: [diagnostic.code],
        };
        disableAction.diagnostics = [diagnostic];
        actions.push(disableAction);
      }
    }
    return actions;
  }
}

// ── Decorations ──

function getDecorationType(severity) {
  if (!decorationTypes[severity]) {
    const color = severity === vscode.DiagnosticSeverity.Error
      ? { gutterIconPath: undefined, dark: { gutterIconColor: '#f14c4c' }, light: { gutterIconColor: '#f14c4c' } }
      : { gutterIconPath: undefined, dark: { gutterIconColor: '#cca700' }, light: { gutterIconColor: '#cca700' } };

    decorationTypes[severity] = vscode.window.createTextEditorDecorationType({
      isWholeLine: true,
      gutterIconSize: 'contain',
      overviewRulerColor: severity === vscode.DiagnosticSeverity.Error
        ? new vscode.ThemeColor('editorError.foreground')
        : new vscode.ThemeColor('editorWarning.foreground'),
      overviewRulerLane: vscode.OverviewRulerLane.Right,
      dark: {
        backgroundColor: severity === vscode.DiagnosticSeverity.Error
          ? 'rgba(241,76,76,0.05)' : 'rgba(204,167,0,0.05)',
      },
      light: {
        backgroundColor: severity === vscode.DiagnosticSeverity.Error
          ? 'rgba(241,76,76,0.05)' : 'rgba(204,167,0,0.05)',
      },
    });
  }
  return decorationTypes[severity];
}

function applyDecorations() {
  const editor = vscode.window.activeTextEditor;
  if (editor) {
    updateDecorations(editor);
  }
}

function updateDecorations(editor) {
  clearDecorations();
  const diagnostics = diagnosticCollection.get(editor.document.uri);
  if (!diagnostics || diagnostics.length === 0) return;

  const errorLines = [];
  const warningLines = [];
  for (const d of diagnostics) {
    const line = d.range.start.line;
    if (d.severity === vscode.DiagnosticSeverity.Error) {
      errorLines.push(new vscode.Range(line, 0, line, 0));
    } else {
      warningLines.push(new vscode.Range(line, 0, line, 0));
    }
  }
  if (errorLines.length > 0) {
    editor.setDecorations(getDecorationType(vscode.DiagnosticSeverity.Error), errorLines);
  }
  if (warningLines.length > 0) {
    editor.setDecorations(getDecorationType(vscode.DiagnosticSeverity.Warning), warningLines);
  }
}

function clearDecorations() {
  for (const key of Object.keys(decorationTypes)) {
    decorationTypes[key]?.dispose();
  }
  decorationTypes = {};
}

// ── Configuration Webview ──

class ConfigWebview {
  static instance;
  static panel;

  static createOrShow(extensionUri) {
    if (ConfigWebview.panel) {
      ConfigWebview.panel.reveal(vscode.ViewColumn.One);
      return;
    }

    const panel = vscode.window.createWebviewPanel(
      'reactAuditorConfig',
      'React Auditor Configuration',
      vscode.ViewColumn.One,
      { enableScripts: true }
    );

    ConfigWebview.panel = panel;
    ConfigWebview.panel.webview.html = getConfigWebviewHtml();

    panel.webview.onDidReceiveMessage(async (message) => {
      switch (message.type) {
        case 'save': {
          const configContent = generateConfigToml(message.config);
          const wsEdit = new vscode.WorkspaceEdit();
          const configPath = vscode.workspace.workspaceFolders?.[0]?.uri;
          if (!configPath) {
            vscode.window.showErrorMessage('No workspace folder open');
            return;
          }
          const fileUri = vscode.Uri.joinPath(configPath, '.rauditrc.toml');
          wsEdit.createFile(fileUri, { overwrite: true });
          wsEdit.set(fileUri, [
            vscode.TextEdit.insert(new vscode.Position(0, 0), configContent),
          ]);
          await vscode.workspace.applyEdit(wsEdit);
          const doc = await vscode.workspace.openTextDocument(fileUri);
          await doc.save();
          vscode.window.showInformationMessage('.rauditrc.toml created');
          ConfigWebview.panel.dispose();
          break;
        }
        case 'cancel': {
          ConfigWebview.panel.dispose();
          break;
        }
      }
    });

    panel.onDidDispose(() => {
      ConfigWebview.panel = undefined;
    });
  }
}

function generateConfigToml(config) {
  const lines = ['# React Auditor Configuration\n'];
  for (const [key, val] of Object.entries(config.rules || {})) {
    lines.push(`"${key}" = "${val}"`);
  }
  return lines.join('\n');
}

function getConfigWebviewHtml() {
  return `<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>React Auditor Config</title>
<style>
body { font-family: var(--vscode-font-family); padding: 20px; color: var(--vscode-editor-foreground); }
h2 { margin-bottom: 16px; }
label { display: block; margin: 8px 0 4px; font-weight: 600; }
select, input { width: 100%; padding: 6px; margin-bottom: 4px; background: var(--vscode-input-background); color: var(--vscode-input-foreground); border: 1px solid var(--vscode-input-border); }
.row { display: flex; gap: 8px; align-items: center; }
.row select { flex: 1; }
.row button { flex-shrink: 0; }
.actions { margin-top: 16px; display: flex; gap: 8px; }
button { padding: 8px 16px; cursor: pointer; border: none; border-radius: 2px; }
.btn-primary { background: var(--vscode-button-background); color: var(--vscode-button-foreground); }
.btn-secondary { background: var(--vscode-button-secondaryBackground); color: var(--vscode-button-secondaryForeground); }
</style>
</head>
<body>
<h2>React Auditor — Rule Configuration</h2>
<p>Set severity overrides for each rule. Rules not listed use their default severity.</p>
<div id="rules"></div>
<div class="actions">
<button class="btn-primary" onclick="save()">Save</button>
<button class="btn-secondary" onclick="cancel()">Cancel</button>
</div>
<script>
const RULES = [
  { id: 'no-console', cat: 'quality', default: 'warning' },
  { id: 'no-var', cat: 'quality', default: 'warning' },
  { id: 'no-empty-blocks', cat: 'quality', default: 'warning' },
  { id: 'no-explicit-any', cat: 'typescript', default: 'warning' },
  { id: 'no-non-null-assertion', cat: 'typescript', default: 'warning' },
  { id: 'no-eval', cat: 'security', default: 'error' },
  { id: 'no-dangerously-set-innerhtml', cat: 'security', default: 'error' },
  { id: 'no-missing-key', cat: 'react', default: 'error' },
  { id: 'no-img-element', cat: 'nextjs', default: 'warning' },
  { id: 'no-large-libraries', cat: 'performance', default: 'warning' },
];
const container = document.getElementById('rules');
const overrides = {};
RULES.forEach(r => {
  const div = document.createElement('div');
  div.className = 'row';
  div.innerHTML = '<span style="flex:1;min-width:200px">' + r.id + ' <span style="opacity:0.6">(' + r.cat + ', default: ' + r.default + ')</span></span>' +
    '<select id="sel-' + r.id + '"><option value="">default</option><option value="error">error</option><option value="warn">warn</option><option value="off">off</option></select>';
  container.appendChild(div);
});
function collect() {
  const config = { rules: {} };
  RULES.forEach(r => {
    const sel = document.getElementById('sel-' + r.id);
    if (sel.value) config.rules[r.id] = sel.value;
  });
  return config;
}
function save() {
  const vscode = acquireVsCodeApi();
  vscode.postMessage({ type: 'save', config: collect() });
}
function cancel() {
  const vscode = acquireVsCodeApi();
  vscode.postMessage({ type: 'cancel' });
}
</script>
</body>
</html>`;
}

// ── Helpers ──

function parseResults(document, results) {
  const diagnostics = [];
  for (const fileResult of results) {
    for (const v of fileResult.violations) {
      const startCol = Math.max(0, (v.column || 1) - 1);
      const range = new vscode.Range(
        v.line - 1, startCol,
        v.line - 1, startCol + 40
      );
      const severity = SEVERITY_MAP[v.severity] || vscode.DiagnosticSeverity.Warning;
      const diagnostic = new vscode.Diagnostic(
        range,
        `[${v.category}/${v.ruleId}] ${v.message}`,
        severity
      );
      diagnostic.source = 'react-auditor';
      diagnostic.code = v.ruleId;
      diagnostic.relatedInformation = [
        new vscode.DiagnosticRelatedInformation(
          new vscode.Location(document.uri, range),
          `Category: ${v.category}`
        ),
      ];
      diagnostics.push(diagnostic);
    }
  }
  return diagnostics;
}

// ── Main auditor runner ──

async function runAuditor(document) {
  const config = vscode.workspace.getConfiguration('reactAuditor');
  const binaryPath = config.get('binaryPath', 'react-auditor');
    const filePath = document.uri.fsPath;

  if (!filePath) return;

  try {
    statusBarItem.text = '$(sync~spin) React Auditor: running...';

    const { stdout } = await execFileAsync(binaryPath, [
      filePath, '--format', 'json',
    ], { timeout: 30000, maxBuffer: 10 * 1024 * 1024 });

    const results = JSON.parse(stdout);
    const diagnostics = parseResults(document, results);

    diagnosticCollection.set(document.uri, diagnostics);

    const count = diagnostics.length;
    statusBarItem.text = count === 0
      ? '$(pass) React Auditor'
      : `$(warning) React Auditor: ${count}`;
    statusBarItem.tooltip = count === 0
      ? 'No issues found'
      : `${count} issue${count > 1 ? 's' : ''} found`;

    // Update decorations for active editor
    const editor = vscode.window.activeTextEditor;
    if (editor && editor.document.uri.fsPath === filePath) {
      updateDecorations(editor);
    }
  } catch (err) {
    // The binary exits with code 1 when violations are found, but still
    // writes valid JSON to stdout. Try to parse it before falling back
    // to error display.
    if (err.stdout) {
      try {
        const results = JSON.parse(err.stdout.toString());
        if (Array.isArray(results) && results.some(r => r.file || r.violations?.length)) {
          diagnosticCollection.set(document.uri, parseResults(document, results));
          const count = diagnosticCollection.get(document.uri)?.length || 0;
          statusBarItem.text = count === 0
            ? '$(pass) React Auditor'
            : `$(warning) React Auditor: ${count}`;
          return;
        }
      } catch (_) {
        // stdout wasn't valid JSON — fall through to error
      }
    }

    if (err.code === 'ENOENT') {
      statusBarItem.text = '$(alert) React Auditor: not found';
      statusBarItem.tooltip = 'binary not found — install with cargo or npm';
      diagnosticCollection.set(document.uri, [new vscode.Diagnostic(
        new vscode.Range(0, 0, 0, 0),
        'react-auditor binary not found. Install: cargo install react-auditor or npm install -g react-auditor',
        vscode.DiagnosticSeverity.Warning
      )]);
    } else {
      const msg = err.stderr ? err.stderr.toString().trim() : err.message;
      statusBarItem.text = '$(alert) React Auditor: error';
      statusBarItem.tooltip = msg;
      diagnosticCollection.set(document.uri, [new vscode.Diagnostic(
        new vscode.Range(0, 0, 0, 0),
        `React Auditor error: ${msg}`,
        vscode.DiagnosticSeverity.Error
      )]);
    }
  }
}

function deactivate() {}

module.exports = { activate, deactivate };
