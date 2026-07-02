#!/usr/bin/env node
const { execFileSync } = require("child_process");
const { join } = require("path");

const bin = join(__dirname, "react-auditor");

try {
  const status = execFileSync(bin, process.argv.slice(2), { stdio: "inherit" });
  process.exit(status.status ?? 0);
} catch (e) {
  process.exit(e.status ?? 1);
}
