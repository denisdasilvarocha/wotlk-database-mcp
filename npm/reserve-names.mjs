import { mkdtemp, mkdir, writeFile, rm } from 'node:fs/promises';
import { join } from 'node:path';
import { tmpdir } from 'node:os';
import { spawnNpm } from './npm-command.mjs';
const names = ['wotlk-database-mcp', ...['darwin-arm64', 'darwin-x64', 'linux-x64', 'linux-arm64', 'win32-x64'].map(platform => `wotlk-database-mcp-${platform}`)];
if (process.argv.length !== 3 || process.argv[2] !== '--publish-reservations') {
  console.error('Explicit opt-in required: node npm/reserve-names.mjs --publish-reservations\nPublishes public 0.0.0 name reservations to npm using your authenticated account.');
  process.exit(1);
}
const root = await mkdtemp(join(tmpdir(), 'wotlk-npm-reserve-'));
try {
  for (const name of names) {
    const directory = join(root, name);
    await mkdir(directory);
    await writeFile(join(directory, 'package.json'), JSON.stringify({ name, version: '0.0.0', description: 'Reserved for the World of Warcraft 3.3.5a database MCP server', files: [] }, null, 2) + '\n');
    const result = spawnNpm(['publish', '--access', 'public', '--tag', 'reserved', '--ignore-scripts'], { cwd: directory, stdio: 'inherit' });
    if (result.error) throw result.error;
    if (result.status !== 0) throw new Error(`Reservation failed for ${name}`);
  }
} finally { await rm(root, { recursive: true, force: true }); }
