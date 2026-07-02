class ReactAuditor < Formula
  desc "Fast Rust-based linter for React, TypeScript, and web security"
  homepage "https://github.com/chesteralan/react-auditor"
  license "MIT"
  version "VERSION_PLACEHOLDER"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/chesteralan/react-auditor/releases/download/vVERSION_PLACEHOLDER/react-auditor-aarch64-apple-darwin-vVERSION_PLACEHOLDER.tar.gz"
      sha256 "MACOS_ARM_SHA256"
    else
      url "https://github.com/chesteralan/react-auditor/releases/download/vVERSION_PLACEHOLDER/react-auditor-x86_64-apple-darwin-vVERSION_PLACEHOLDER.tar.gz"
      sha256 "MACOS_X64_SHA256"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/chesteralan/react-auditor/releases/download/vVERSION_PLACEHOLDER/react-auditor-aarch64-unknown-linux-gnu-vVERSION_PLACEHOLDER.tar.gz"
      sha256 "LINUX_ARM_SHA256"
    else
      url "https://github.com/chesteralan/react-auditor/releases/download/vVERSION_PLACEHOLDER/react-auditor-x86_64-unknown-linux-gnu-vVERSION_PLACEHOLDER.tar.gz"
      sha256 "LINUX_X64_SHA256"
    end
  end

  def install
    bin.install "react-auditor"
  end

  test do
    system "#{bin}/react-auditor", "--version"
  end
end
