#!/usr/bin/env node

/**
 * @usepaperpilot/mcp
 * Zero-install MCP runner for PaperPilot.
 *
 * Automatically detects OS/architecture, retrieves or caches the precompiled
 * paperpilot-mcp binary in ~/.paperpilot/bin, and transparently forwards
 * standard I/O (stdin, stdout, stderr) for MCP JSON-RPC protocol compliance.
 */

const { spawn } = require('child_process');
const fs = require('fs');
const path = require('path');
const os = require('os');
const https = require('https');
const http = require('http');
const zlib = require('zlib');

// Platform and Architecture Detection
function getPlatformInfo() {
  const platform = os.platform(); // 'linux', 'darwin', 'win32'
  const arch = os.arch();         // 'x64', 'arm64', 'ia32', etc.

  let osTarget = '';
  let archTarget = '';
  let ext = '';
  let isZip = false;

  if (platform === 'linux') {
    osTarget = 'linux';
    if (arch === 'x64') {
      archTarget = 'x86_64';
    } else if (arch === 'arm64' || arch === 'aarch64') {
      archTarget = 'aarch64';
    }
  } else if (platform === 'darwin') {
    osTarget = 'macos';
    if (arch === 'arm64' || arch === 'aarch64') {
      archTarget = 'aarch64';
    } else if (arch === 'x64') {
      archTarget = 'x86_64';
    }
  } else if (platform === 'win32') {
    osTarget = 'windows';
    ext = '.exe';
    isZip = true;
    if (arch === 'x64') {
      archTarget = 'x86_64';
    }
  }

  if (!osTarget || !archTarget) {
    throw new Error(
      `Unsupported platform/architecture: ${platform} (${arch}). PaperPilot MCP supports Linux (x64, arm64), macOS (Apple Silicon arm64, Intel x64), and Windows (x64).`
    );
  }

  return { osTarget, archTarget, ext, isZip, platform };
}

// Determine writeable cache directory
function getCacheDir() {
  if (process.env.PAPERPILOT_CACHE_DIR) {
    return process.env.PAPERPILOT_CACHE_DIR;
  }
  const homeDir = os.homedir();
  const primaryCache = path.join(homeDir, '.paperpilot', 'bin');
  try {
    fs.mkdirSync(primaryCache, { recursive: true });
    return primaryCache;
  } catch (_) {
    // If ~/.paperpilot is read-only (e.g. sandbox or restricted environment), fallback to temp directory
    const fallbackCache = path.join(os.tmpdir(), '.paperpilot', 'bin');
    try {
      fs.mkdirSync(fallbackCache, { recursive: true });
    } catch (_) {}
    return fallbackCache;
  }
}

// Locate local or cached binary
function getBinaryPath(info) {
  const binaryName = `paperpilot-mcp${info.ext}`;

  // 1. Check custom override env var
  if (process.env.PAPERPILOT_MCP_PATH && fs.existsSync(process.env.PAPERPILOT_MCP_PATH)) {
    return process.env.PAPERPILOT_MCP_PATH;
  }

  // 2. Check user cache directory (~/.paperpilot/bin or temp fallback)
  const homeDir = os.homedir();
  const primaryCachedBinary = path.join(homeDir, '.paperpilot', 'bin', binaryName);
  if (fs.existsSync(primaryCachedBinary)) {
    return primaryCachedBinary;
  }

  const fallbackCachedBinary = path.join(os.tmpdir(), '.paperpilot', 'bin', binaryName);
  if (fs.existsSync(fallbackCachedBinary)) {
    return fallbackCachedBinary;
  }

  // 3. Check workspace / target builds if running inside repo or nearby
  const repoRoot = path.resolve(__dirname, '..', '..', '..');
  const possiblePaths = [
    path.join(repoRoot, 'target', 'release', binaryName),
    path.join(repoRoot, 'target', 'debug', binaryName),
    path.join(__dirname, '..', binaryName),
  ];
  for (const p of possiblePaths) {
    if (fs.existsSync(p)) {
      return p;
    }
  }

  // Default target path for download
  const cacheDir = getCacheDir();
  return path.join(cacheDir, binaryName);
}

// Download helper with redirect handling
function downloadFile(url, dest, onProgress) {
  return new Promise((resolve, reject) => {
    const file = fs.createWriteStream(dest);

    const makeRequest = (currentUrl, redirects = 0) => {
      if (redirects > 8) {
        return reject(new Error('Too many redirects while downloading binary.'));
      }

      const client = currentUrl.startsWith('https://') ? https : http;
      const req = client.get(
        currentUrl,
        {
          headers: {
            'User-Agent': '@usepaperpilot/mcp-runner',
            'Accept': 'application/octet-stream, */*',
          },
        },
        (res) => {
          if (res.statusCode >= 300 && res.statusCode < 400 && res.headers.location) {
            const redirectUrl = new URL(res.headers.location, currentUrl).href;
            return makeRequest(redirectUrl, redirects + 1);
          }

          if (res.statusCode !== 200) {
            return reject(
              new Error(`Failed to download binary: HTTP ${res.statusCode} ${res.statusMessage} from ${currentUrl}`)
            );
          }

          const totalBytes = parseInt(res.headers['content-length'] || '0', 10);
          let downloadedBytes = 0;

          res.on('data', (chunk) => {
            downloadedBytes += chunk.length;
            if (onProgress && totalBytes > 0) {
              onProgress(downloadedBytes, totalBytes);
            }
          });

          res.pipe(file);

          file.on('finish', () => {
            file.close(() => resolve());
          });
        }
      );

      req.on('error', (err) => {
        fs.unlink(dest, () => {});
        reject(err);
      });
    };

    makeRequest(url);
  });
}

// Simple tar.gz extractor in pure Node without external deps
function extractTarGz(tarGzPath, targetDir, binaryFilename) {
  return new Promise((resolve, reject) => {
    try {
      const gunzip = zlib.createGunzip();
      const readStream = fs.createReadStream(tarGzPath);

      let buffer = Buffer.alloc(0);

      gunzip.on('data', (chunk) => {
        buffer = Buffer.concat([buffer, chunk]);
      });

      gunzip.on('end', () => {
        // Parse TAR blocks (512 bytes)
        let offset = 0;
        let extracted = false;
        const outPath = path.join(targetDir, binaryFilename);

        while (offset + 512 <= buffer.length) {
          const header = buffer.subarray(offset, offset + 512);
          // Check if empty block
          if (header.every((b) => b === 0)) {
            break;
          }

          // File name is at offset 0, 100 bytes null-terminated
          let fileName = header.subarray(0, 100).toString('utf8').replace(/\0.*$/, '').trim();
          // Strip prefix if any
          fileName = path.basename(fileName);

          // File size at offset 124, 12 bytes octal
          const sizeStr = header.subarray(124, 136).toString('utf8').replace(/\0.*$/, '').trim();
          const fileSize = parseInt(sizeStr, 8) || 0;

          offset += 512;

          if (fileName === binaryFilename) {
            const fileData = buffer.subarray(offset, offset + fileSize);
            fs.writeFileSync(outPath, fileData);
            extracted = true;
          }

          // Round up to next 512 byte boundary
          const remainder = fileSize % 512;
          const padding = remainder === 0 ? 0 : 512 - remainder;
          offset += fileSize + padding;
        }

        if (extracted) {
          resolve(outPath);
        } else {
          reject(new Error(`Binary ${binaryFilename} not found inside tar archive.`));
        }
      });

      gunzip.on('error', reject);
      readStream.on('error', reject);
      readStream.pipe(gunzip);
    } catch (err) {
      reject(err);
    }
  });
}

// Extract zip on Windows using PowerShell Expand-Archive (built-in) or direct stream fallback
function extractZip(zipPath, targetDir, binaryFilename) {
  return new Promise((resolve, reject) => {
    const child = spawn('powershell.exe', [
      '-NoProfile',
      '-NonInteractive',
      '-Command',
      `Expand-Archive -Path "${zipPath}" -DestinationPath "${targetDir}" -Force`,
    ], { stdio: 'inherit' });

    child.on('close', (code) => {
      const outPath = path.join(targetDir, binaryFilename);
      if (code === 0 && fs.existsSync(outPath)) {
        resolve(outPath);
      } else {
        reject(new Error(`Failed to extract ${zipPath} via PowerShell (code ${code}).`));
      }
    });

    child.on('error', reject);
  });
}

// Ensure precompiled binary exists or download it
async function ensureBinary(info) {
  const binaryPath = getBinaryPath(info);
  const binaryName = `paperpilot-mcp${info.ext}`;

  if (fs.existsSync(binaryPath)) {
    return binaryPath;
  }

  const cacheDir = path.dirname(binaryPath);
  if (!fs.existsSync(cacheDir)) {
    fs.mkdirSync(cacheDir, { recursive: true });
  }

  // Construct download URL
  // GitHub Release asset patterns:
  // Tar: paperpilot-mcp-<os>-<target>.tar.gz or direct binary
  // Release fallback asset name candidates
  const archiveExt = info.isZip ? '.zip' : '.tar.gz';
  const assetCandidates = [
    `paperpilot-mcp-${info.osTarget}-${info.archTarget}${info.ext}`,
    `paperpilot-mcp-${info.osTarget}-${info.archTarget}${archiveExt}`,
    `paperpilot-mcp${info.ext}`,
  ];

  process.stderr.write(`[paperpilot-mcp] PaperPilot MCP binary not found locally.\n`);
  process.stderr.write(`[paperpilot-mcp] Downloading precompiled binary for ${info.osTarget}-${info.archTarget}...\n`);

  let downloaded = false;
  let lastError = null;

  for (const asset of assetCandidates) {
    const downloadUrl = `https://github.com/KrushnaVardhanReddy/PaperPilot/releases/latest/download/${asset}`;
    const tmpDest = path.join(cacheDir, `download-${Date.now()}-${asset}`);

    try {
      process.stderr.write(`[paperpilot-mcp] Trying: ${downloadUrl}\n`);
      await downloadFile(downloadUrl, tmpDest, (curr, total) => {
        const pct = Math.floor((curr / total) * 100);
        process.stderr.write(`\r[paperpilot-mcp] Downloading: ${pct}% (${(curr / 1024 / 1024).toFixed(1)}MB / ${(total / 1024 / 1024).toFixed(1)}MB)`);
      });
      process.stderr.write('\n');

      // Check if it's an archive or direct binary
      if (asset.endsWith('.tar.gz')) {
        await extractTarGz(tmpDest, cacheDir, binaryName);
        fs.unlinkSync(tmpDest);
      } else if (asset.endsWith('.zip')) {
        await extractZip(tmpDest, cacheDir, binaryName);
        fs.unlinkSync(tmpDest);
      } else {
        // Direct binary
        fs.renameSync(tmpDest, binaryPath);
      }

      if (fs.existsSync(binaryPath)) {
        if (info.platform !== 'win32') {
          fs.chmodSync(binaryPath, 0o755);
        }
        process.stderr.write(`[paperpilot-mcp] Successfully installed PaperPilot MCP binary to ${binaryPath}\n`);
        downloaded = true;
        break;
      }
    } catch (err) {
      lastError = err;
      if (fs.existsSync(tmpDest)) {
        try { fs.unlinkSync(tmpDest); } catch (_) {}
      }
    }
  }

  if (!downloaded) {
    throw new Error(
      `Could not automatically download PaperPilot MCP binary.\n` +
      `You can build it from source with:\n` +
      `  cargo build --release -p paperpilot-mcp\n` +
      `And copy target/release/${binaryName} to ${binaryPath}\n` +
      `Original error: ${lastError ? lastError.message : 'Unknown'}`
    );
  }

  return binaryPath;
}

// Main execution routine
async function main() {
  const args = process.argv.slice(2);

  let info;
  try {
    info = getPlatformInfo();
  } catch (err) {
    process.stderr.write(`[paperpilot-mcp] Error: ${err.message}\n`);
    process.exit(1);
  }

  let binaryPath;
  try {
    binaryPath = await ensureBinary(info);
  } catch (err) {
    process.stderr.write(`[paperpilot-mcp] Initialization error: ${err.message}\n`);
    process.exit(1);
  }

  // Ensure execution permissions on Unix
  if (info.platform !== 'win32') {
    try {
      fs.chmodSync(binaryPath, 0o755);
    } catch (_) {}
  }

  // Spawn the native Rust MCP server forwarding stdin, stdout, and stderr
  const child = spawn(binaryPath, args, {
    stdio: ['inherit', 'inherit', 'inherit'],
    env: process.env,
  });

  child.on('error', (err) => {
    process.stderr.write(`[paperpilot-mcp] Failed to launch binary: ${err.message}\n`);
    process.exit(1);
  });

  child.on('close', (code, signal) => {
    if (signal) {
      process.kill(process.pid, signal);
    } else {
      process.exit(code !== null ? code : 0);
    }
  });

  // Handle process signals for graceful child teardown
  ['SIGINT', 'SIGTERM', 'SIGHUP'].forEach((signal) => {
    process.on(signal, () => {
      if (!child.killed) {
        child.kill(signal);
      }
    });
  });
}

main().catch((err) => {
  process.stderr.write(`[paperpilot-mcp] Fatal: ${err.message}\n`);
  process.exit(1);
});
