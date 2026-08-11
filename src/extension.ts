// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
import * as vscode from "vscode";

// This method is called when your extension is activated
// Your extension is activated the very first time the command is executed
export function activate(context: vscode.ExtensionContext) {
  context.subscriptions.push(
    vscode.window.registerWebviewViewProvider(
      "apaSidebarView",
      new ApaWebviewViewProvider(context),
    ),
  );

  // Now provide the implementation of the command with registerCommand
  const disposable = vscode.commands.registerCommand(
    "autonomous-path-architect.reload",
    () => {
      vscode.window.showInformationMessage(
        "This hasn't been implemented yet. Please check back later.",
      );
    },
  );

  context.subscriptions.push(disposable);
}

class ApaWebviewViewProvider implements vscode.WebviewViewProvider {
  constructor(private readonly context: vscode.ExtensionContext) {}

  resolveWebviewView(webviewView: vscode.WebviewView) {
    webviewView.webview.options = {
      enableScripts: true,
    };

    webviewView.webview.html = `
      <!DOCTYPE html>
      <html lang="en">
      <body>
        <h1 id="grammatik">Grammatik</h1>
<li>Durchgängig <strong>Imperfekt</strong> (Präteritum) nutzen.</li>
<li>Weitestgehend im <strong>Passiv</strong> berichten.</li>
<li><strong>Präzise</strong> Begriffe und Beschreibungen anstreben.</li>
<li>Mit <strong>Stichpunkten</strong> strukturieren, Fließtext meiden.</li>
      </body>
      </html>
    `;
  }
}

// This method is called when your extension is deactivated
export function deactivate() {}
