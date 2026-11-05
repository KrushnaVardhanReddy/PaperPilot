import * as fs from 'fs';
import * as path from 'path';
import * as crypto from 'crypto';

/**
 * Manages unique, isolated filesystem workspaces per test.
 * Completely eliminates state cross-contamination and manual cleanup.
 */
export class SandboxManager {
  private baseDir: string;
  private activeSandboxes: Set<string> = new Set();

  constructor(baseDir?: string) {
    this.baseDir = baseDir || path.resolve(process.cwd(), '../../tests/e2e_fixtures/out/sandboxes');
    if (!fs.existsSync(this.baseDir)) {
      fs.mkdirSync(this.baseDir, { recursive: true });
    }
  }

  /**
   * Allocates a new ephemeral sandbox folder with a unique UUID.
   */
  public allocate(prefix: string = 'test'): string {
    const uuid = crypto.randomUUID();
    const sandboxPath = path.resolve(this.baseDir, `${prefix}_${uuid}`);
    fs.mkdirSync(sandboxPath, { recursive: true });
    this.activeSandboxes.add(sandboxPath);
    return sandboxPath;
  }

  /**
   * Cleans up a specific sandbox directory safely.
   */
  public cleanup(sandboxPath: string): void {
    if (this.activeSandboxes.has(sandboxPath) && fs.existsSync(sandboxPath)) {
      // Ensure path is strictly within baseDir to avoid traversal
      if (sandboxPath.startsWith(this.baseDir)) {
        fs.rmSync(sandboxPath, { recursive: true, force: true });
        this.activeSandboxes.delete(sandboxPath);
      }
    }
  }

  /**
   * Sweeps all active sandboxes allocated by this instance.
   */
  public sweep(): void {
    for (const sandboxPath of Array.from(this.activeSandboxes)) {
      this.cleanup(sandboxPath);
    }
  }

  /**
   * Prunes abandoned sandboxes older than maxAgeMs (default: 1 hour).
   */
  public pruneStale(maxAgeMs: number = 3600000): void {
    if (!fs.existsSync(this.baseDir)) return;
    const now = Date.now();
    for (const entry of fs.readdirSync(this.baseDir)) {
      const fullPath = path.resolve(this.baseDir, entry);
      try {
        const stat = fs.statSync(fullPath);
        if (stat.isDirectory() && now - stat.mtimeMs > maxAgeMs) {
          fs.rmSync(fullPath, { recursive: true, force: true });
        }
      } catch (e) {}
    }
  }
}
