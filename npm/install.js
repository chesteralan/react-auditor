#!/usr/bin/env node

const { existsSync, chmodSync, createWriteStream, unlinkSync, readFileSync } = require("fs");
const { get } = require("https");
const { join } = require("path");
const { platform, arch } = require("os");
const { execSync, execFileSync } = require("child_process");

const PKG_VERSION = "0.1.8";
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

const isWin = platform() === "win32";
const binaryName = isWin ? "react-auditor.exe" : "react-auditor";
const archiveExt = isWin ? ".zip" : ".tar.gz";
const archiveName = `react-auditor-${target}-v${PKG_VERSION}${archiveExt}`;
const url = `https://github.com/${REPO}/releases/download/v${PKG_VERSION}/${archiveName}`;
const dest = join(__dirname, binaryName);

function getInstalledVersion(binPath) {
  try {
    const output = execFileSync(binPath, ["--version"], {
      encoding: "utf8",
      stdio: ["ignore", "pipe", "pipe"],
    }).trim();
    const match = output.match(/\d+\.\d+\.\d+(?:[-+][0-9A-Za-z.-]+)?/);
    return match ? match[0] : null;
  } catch (_) {
    return null;
  }
}

if (existsSync(dest)) {
  const installedVersion = getInstalledVersion(dest);
  if (installedVersion === PKG_VERSION) {
    console.log(`react-auditor v${PKG_VERSION} is already installed`);
    process.exit(0);
  }
  console.log(
    `Found existing react-auditor ${installedVersion ? `v${installedVersion}` : "(unknown version)"}; installing v${PKG_VERSION}...`
  );
}

console.log(`Downloading react-auditor v${PKG_VERSION} for ${target}...`);

const tmpFile = join(__dirname, `_tmp_${archiveName}`);

function cleanup() {
  try { unlinkSync(tmpFile); } catch (_) {}
}

process.on("exit", cleanup);

const tmp = createWriteStream(tmpFile);
get(url, (response) => {
  if (response.statusCode !== 200) {
    console.error(`Download failed (HTTP ${response.statusCode})`);
    console.error(`  ${url}`);
    console.error("Please install via cargo: cargo install react-auditor");
    process.exit(1);
  }
  response.pipe(tmp);
  tmp.on("finish", () => {
    tmp.close();
    try {
      if (isWin) {
        execSync(`powershell -NoProfile "Expand-Archive -Path '${tmpFile}' -DestinationPath '${__dirname}' -Force"`, { stdio: "pipe" });
      } else {
        execSync(`tar xzf "${tmpFile}" -C "${__dirname}"`, { stdio: "pipe" });
      }
    } catch (e) {
      console.error("Extraction failed:", e.message);
      process.exit(1);
    }
    if (existsSync(dest)) {
      if (!isWin) chmodSync(dest, 0o755);
      console.log("react-auditor installed successfully");
    } else {
      console.error(`Binary not found after extraction: ${dest}`);
      process.exit(1);
    }
  });
}).on("error", (err) => {
  console.error("Download failed:", err.message);
  process.exit(1);
});
