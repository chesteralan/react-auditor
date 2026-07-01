const vscode = require('vscode');
const { execFile } = require('child_process');
const { promisify } = require('util');

const execFileAsync = promisify(execFile);

const SEVERITY_MAP = {
  error: vscode.DiagnosticSeverity.Error,
  warning: vscode.DiagnosticSeverity.Warning,
};

/**
 * @param {vscode.ExtensionContext} context
 */
function activate(context) {
  const diagnosticCollection = vscode.languages.createDiagnosticCollection('react-auditor');
  context.subscriptions.push(diagnosticCollection);

  async function runAuditor(document) {
    const config = vscode.workspace.getConfiguration('reactAuditor');
    const binaryPath = config.get('binaryPath', 'react-auditor');

    try {
      const { stdout } = await execFileAsync(binaryPath, [
        document.uri.fsPath,
        '--format', 'json',
      ], { timeout: 30000, maxBuffer: 10 * 1024 * 1024 });

      const results = JSON.parse(stdout);
      const diagnostics = [];

      for (const fileResult of results) {
        for (const v of fileResult.violations) {
          const range = new vscode.Range(
            v.line - 1, (v.column || 1) - 1,
            v.line - 1, (v.column || 1) + 20
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
    } catch (err) {
      if (err.code === 'ENOENT') {
        diagnosticCollection.set(document.uri, [new vscode.Diagnostic(
          new vscode.Range(0, 0, 0, 0),
          'react-auditor binary not found. Install: cargo install react-auditor',
          vscode.DiagnosticSeverity.Warning
        )]);
      }
    }
  }

  const runOnSave = vscode.workspace.onDidSaveTextDocument((doc) => {
    const config = vscode.workspace.getConfiguration('reactAuditor');
    if (config.get('runOnSave', true)) {
      runAuditor(doc);
    }
  });
  context.subscriptions.push(runOnSave);

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
    for (const file of files.slice(0, 50)) {
      const doc = await vscode.workspace.openTextDocument(file);
      await runAuditor(doc);
    }
    vscode.window.showInformationMessage('React Auditor: scanned workspace');
  }));
}

function deactivate() {}

module.exports = { activate, deactivate };
