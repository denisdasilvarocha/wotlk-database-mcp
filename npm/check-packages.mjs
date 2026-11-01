import { readdir, readFile } from 'node:fs/promises';
import { resolve, join } from 'node:path';
import { spawnNpm } from './npm-command.mjs';
const [argument, ...extra] = process.argv.slice(2);
if (!argument || extra.length) throw new Error('Usage: node npm/check-packages.mjs DIRECTORY');
const directory = resolve(argument);
let checked = 0;
for (const entry of await readdir(directory, { withFileTypes: true })) {
  if (!entry.isDirectory()) continue;
  const packageDir = join(directory, entry.name);
  let manifest;
  try { manifest = JSON.parse(await readFile(join(packageDir, 'package.json'), 'utf8')); }
  catch (error) { if (error.code === 'ENOENT') continue; throw error; }
  const result = spawnNpm(['pack', '--dry-run', '--ignore-scripts', '--json'], { cwd: packageDir, encoding: 'utf8' });
  if (result.error) throw result.error;
  if (result.status !== 0) throw new Error(result.stderr);
  const files = Object.values(JSON.parse(result.stdout))[0].files;
  const forbidden = files.filter(file => /\.sqlite(?:$|[.-])/i.test(file.path));
  if (forbidden.length) throw new Error(`${manifest.name} contains SQLite files: ${forbidden.map(file => file.path).join(', ')}`);
  console.log(`${manifest.name}: ${files.length} files, no SQLite database`);
  checked++;
}
if (!checked) throw new Error(`No package directories found in ${directory}`);
