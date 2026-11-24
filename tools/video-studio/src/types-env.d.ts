declare namespace NodeJS {
  interface ProcessEnv {
    [key: string]: string | undefined;
  }
  interface Process {
    env: ProcessEnv;
    argv: string[];
    exit(code?: number): never;
  }
}
declare var process: NodeJS.Process;
declare module 'path';
declare module 'fs';
declare module 'url';
