import { spawnSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { dirname, join, delimiter } from 'node:path';

// Execute npm's JavaScript CLI directly on Windows: npm.cmd is not an executable,
// and using a shell would reinterpret release paths containing shell characters.
export function spawnNpm(args, options) {
  if (process.platform !== 'win32') return spawnSync('npm', args, options);
  const directories = [...(process.env.PATH ?? '').split(delimiter), dirname(process.execPath)];
  const cli = directories.map(directory => join(directory, 'node_modules', 'npm', 'bin', 'npm-cli.js')).find(existsSync);
  if (!cli) throw new Error('Cannot locate npm/node_modules/npm/bin/npm-cli.js on PATH or beside node.exe. Install Node.js with npm.');
  return spawnSync(process.execPath, [cli, ...args], options);
}
