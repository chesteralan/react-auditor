class ReactAuditor < Formula
  desc "Fast Rust-based linter for React, TypeScript, and web security"
  homepage "https://github.com/chesteralan/react-auditor"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/chesteralan/react-auditor/releases/download/v0.1.4/react-auditor-aarch64-apple-darwin-v0.1.4.tar.gz"
      sha256 "0000000000000000000000000000000000000000000000000000000000000000" # placeholder
    else
      url "https://github.com/chesteralan/react-auditor/releases/download/v0.1.4/react-auditor-x86_64-apple-darwin-v0.1.4.tar.gz"
      sha256 "0000000000000000000000000000000000000000000000000000000000000000" # placeholder
    end
  end

  on_linux do
    url "https://github.com/chesteralan/react-auditor/releases/download/v0.1.4/react-auditor-x86_64-unknown-linux-gnu-v0.1.4.tar.gz"
    sha256 "0000000000000000000000000000000000000000000000000000000000000000" # placeholder
  end

  def install
    bin.install "react-auditor"
  end

  test do
    system "#{bin}/react-auditor", "--version"
  end
end
