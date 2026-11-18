class Paperpilot < Formula
  desc "100% Private, Local-First, Pure-Rust PDF & Document Workstation"
  homepage "https://usepaperpilot.com"
  version "0.7.0"
  license "Apache-2.0"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/KrushnaVardhanReddy/PaperPilot/releases/download/v#{version}/paperpilot-cli-macos-latest-aarch64-apple-darwin.tar.gz"
      sha256 "6e80e292b3ee2aad97820b063fca7446deccc19c7d1049788dc17464089f13aa"
    else
      url "https://github.com/KrushnaVardhanReddy/PaperPilot/releases/download/v#{version}/paperpilot-cli-macos-latest-x86_64-apple-darwin.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256" # Pending x86_64-apple-darwin release asset
    end
  end

  on_linux do
    if Hardware::CPU.intel?
      url "https://github.com/KrushnaVardhanReddy/PaperPilot/releases/download/v#{version}/paperpilot-cli-ubuntu-22.04-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "4454fc3a247a1146f88d1c1056f5f0a9bbc623767155ee21ac2d1200117d117d"
    end
  end

  def install
    bin.install "paperpilot"
  end

  test do
    assert_match "paperpilot", shell_output("#{bin}/paperpilot --version")
  end
end
