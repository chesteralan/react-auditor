const vscode = require('vscode');
const { execFile } = require('child_process');
const { promisify } = require('util');

const execFileAsync = promisify(execFile);

const SEVERITY_MAP = {
  error: vscode.DiagnosticSeverity.Error,
  warning: vscode.DiagnosticSeverity.Warning,
};

let statusBarItem;

function activate(context) {
  const diagnosticCollection = vscode.languages.createDiagnosticCollection('react-auditor');
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
    statusBarItem.text = '$(sync~spin) React Auditor: scanning...';
    const files = await vscode.workspace.findFiles(
      '**/*.{js,jsx,ts,tsx}',
      '**/node_modules/**'
    );
    for (const file of files.slice(0, 50)) {
      const doc = await vscode.workspace.openTextDocument(file);
      await runAuditor(doc);
    }
    statusBarItem.text = '$(check) React Auditor';
    vscode.window.showInformationMessage(`React Auditor: scanned ${Math.min(files.length, 50)} files`);
  }));

  context.subscriptions.push(vscode.commands.registerCommand('react-auditor.clear', () => {
    diagnosticCollection.clear();
    statusBarItem.text = '$(check) React Auditor';
  }));
}

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
          `[${v.ruleId}] ${v.message}`,
          severity
        );
        diagnostic.source = 'react-auditor';
        diagnostics.push(diagnostic);
      }
    }

    diagnosticCollection.set(document.uri, diagnostics);

    const count = diagnostics.length;
    statusBarItem.text = count === 0
      ? '$(pass) React Auditor'
      : `$(warning) React Auditor: ${count}`;
    statusBarItem.tooltip = count === 0
      ? 'No issues found'
      : `${count} issue${count > 1 ? 's' : ''} found`;
  } catch (err) {
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
