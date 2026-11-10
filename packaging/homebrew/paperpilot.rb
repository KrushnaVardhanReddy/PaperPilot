class Paperpilot < Formula
  desc "100% Private, Local-First, Pure-Rust PDF & Document Workstation"
  homepage "https://paperpilot.app"
  version "1.0.0"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/KrushnaVardhanReddy/PaperPilot/releases/download/v#{version}/paperpilot-aarch64-apple-darwin.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256"
    else
      url "https://github.com/KrushnaVardhanReddy/PaperPilot/releases/download/v#{version}/paperpilot-x86_64-apple-darwin.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256"
    end
  end

  on_linux do
    if Hardware::CPU.intel?
      url "https://github.com/KrushnaVardhanReddy/PaperPilot/releases/download/v#{version}/paperpilot-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256"
    end
  end

  def install
    bin.install "paperpilot"
  end

  test do
    assert_match "paperpilot", shell_output("#{bin}/paperpilot --version")
  end
end
