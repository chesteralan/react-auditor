#!/usr/bin/env node

const { execSync } = require("child_process");
const { existsSync, chmodSync, createWriteStream, mkdirSync } = require("fs");
const { get } = require("https");
const { join } = require("path");
const { platform, arch } = require("os");

const PKG_VERSION = "0.1.5";
const REPO = "chesteralan/react-auditor";

const PLATFORM_MAP = {
  darwin: { x64: "x86_64-apple-darwin", arm64: "aarch64-apple-darwin" },
  linux: { x64: "x86_64-unknown-linux-gnu", arm64: "aarch64-unknown-linux-gnu" },
  win32: { x64: "x86_64-pc-windows-msvc" },
};

const target = PLATFORM_MAP[platform()]?.[arch()];

if (!target) {
  console.error(`Unsupported platform: ${platform()} ${arch()}`);
  console.error("Please install via cargo: cargo install react-auditor");
  process.exit(1);
}

const binaryName = platform() === "win32" ? "react-auditor.exe" : "react-auditor";
const url = `https://github.com/${REPO}/releases/download/v${PKG_VERSION}/${binaryName}-${target}`;
const dest = join(__dirname, binaryName);

if (existsSync(dest)) {
  console.log("react-auditor binary already installed");
  process.exit(0);
}

console.log(`Downloading react-auditor v${PKG_VERSION} for ${target}...`);

const file = createWriteStream(dest);
get(url, (response) => {
  if (response.statusCode !== 200) {
    console.error(`Download failed (HTTP ${response.statusCode})`);
    console.error(`  ${url}`);
    console.error("Please install via cargo: cargo install react-auditor");
    process.exit(1);
  }
  response.pipe(file);
  file.on("finish", () => {
    file.close();
    chmodSync(dest, 0o755);
    console.log("react-auditor installed successfully");
  });
}).on("error", (err) => {
  console.error("Download failed:", err.message);
  process.exit(1);
});
