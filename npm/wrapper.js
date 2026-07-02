#!/usr/bin/env node
const { execFileSync, execSync } = require("child_process");
const { join } = require("path");
const { existsSync } = require("fs");

function findBinary() {
  const local = join(__dirname, "react-auditor");
  if (existsSync(local)) return local;
  try {
    return execSync("which react-auditor", { encoding: "utf8" }).trim();
  } catch {
    return null;
  }
}

const bin = findBinary();

if (!bin) {
  console.error(
    "react-auditor binary not found.\n" +
      "Install it first:\n" +
      "  cargo install react-auditor\n" +
      "  brew install react-auditor\n" +
      "  npm install -g react-auditor"
  );
  process.exit(1);
}

try {
  const status = execFileSync(bin, process.argv.slice(2), { stdio: "inherit" });
  process.exit(status.status ?? 0);
} catch (e) {
  process.exit(e.status ?? 1);
}
