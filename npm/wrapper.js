#!/usr/bin/env node
const { execFileSync } = require("child_process");
const { join } = require("path");
const { existsSync } = require("fs");

const BINARIES = {
  "darwin-arm64": "react-auditor-darwin-arm64",
  "darwin-x64": "react-auditor-darwin-x64",
  "linux-arm64": "react-auditor-linux-arm64",
  "linux-x64": "react-auditor-linux-x64",
  "win32-x64": "react-auditor-win32-x64.exe",
};

function findBinary() {
  const key = `${process.platform}-${process.arch}`;
  const name = BINARIES[key];
  if (!name) return null;
  const local = join(__dirname, name);
  if (existsSync(local)) return local;
  return null;
}

const bin = findBinary();

if (!bin) {
  console.error(
    "react-auditor binary not found for your platform.\n" +
      "Supported: macOS (x64/arm64), Linux (x64/arm64), Windows (x64).\n" +
      "Install via: cargo install react-auditor  or  brew install react-auditor"
  );
  process.exit(1);
}

try {
  execFileSync(bin, process.argv.slice(2), { stdio: "inherit" });
} catch (e) {
  process.exit(e.status ?? 1);
}
