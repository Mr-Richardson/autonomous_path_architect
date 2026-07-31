// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
import * as vscode from "vscode";

// This method is called when your extension is activated
// Your extension is activated the very first time the command is executed
export function activate(context: vscode.ExtensionContext) {
  console.log(
    'Congratulations, the extension "autonomous-path-architect" is now active!',
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

// This method is called when your extension is deactivated
export function deactivate() {}
