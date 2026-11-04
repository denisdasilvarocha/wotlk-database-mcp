// Starts the installed npm command with an empty cache and checks that the
// MCP handshake and tools/list succeed without a database.
// Usage: node npm/smoke.mjs INSTALL_DIR
// @ts-check
import { spawn } from 'node:child_process';
import { once } from 'node:events';
import { mkdir, mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { createInterface } from 'node:readline';

const [installDir, ...extra] = process.argv.slice(2);
if (!installDir || extra.length) throw new Error('Usage: node npm/smoke.mjs INSTALL_DIR');
const installDirectory = resolve(installDir);
await mkdir(installDirectory, { recursive: true });
const packageJsonPath = join(installDirectory, 'package.json');
try {
  const packageManifest = JSON.parse(await readFile(packageJsonPath, 'utf8'));
  if (packageManifest.private !== true) {
    throw new Error('INSTALL_DIR/package.json must set private to true');
  }
} catch (error) {
  if (error && typeof error === 'object' && 'code' in error && error.code === 'ENOENT') {
    await writeFile(packageJsonPath, `${JSON.stringify({ private: true })}\n`);
  } else {
    throw error;
  }
}

const home = await mkdtemp(join(tmpdir(), 'wotlk-mcp-smoke-'));
// Windows resolves the cache through SHGetKnownFolderPath, which fails for a missing folder.
const localAppData = join(home, 'AppData', 'Local');
await mkdir(localAppData, { recursive: true });
const env = {
  ...process.env,
  HOME: home,
  USERPROFILE: home,
  XDG_CACHE_HOME: join(home, 'cache'),
  LOCALAPPDATA: localAppData,
  APPDATA: join(home, 'AppData', 'Roaming'),
};
delete env.WOTLK_DB;

const isWindows = process.platform === 'win32';
const command = isWindows ? 'npx.cmd --offline --yes wotlk-database-mcp' : 'npx';
const args = isWindows ? [] : ['--offline', '--yes', 'wotlk-database-mcp'];
const child = spawn(command, args, {
  cwd: installDirectory,
  env,
  stdio: ['pipe', 'pipe', 'inherit'],
  ...(isWindows && { shell: true }),
});
/** @param {{ id?: number; method: string; params?: Record<string, unknown> }} message */
const send = (message) => child.stdin.write(`${JSON.stringify({ jsonrpc: '2.0', ...message })}\n`);
const lines = createInterface({ input: child.stdout });
let timedOut = false;
const timer = setTimeout(() => {
  timedOut = true;
  lines.close();
  // The native child can keep stdout open after its launcher exits.
  child.stdout.destroy();
  child.kill();
}, 30_000);
/** @type {Array<{ name?: string }> | undefined} */
let tools;
try {
  send({ id: 1, method: 'initialize', params: { protocolVersion: '2024-11-05', capabilities: {}, clientInfo: { name: 'npm-smoke', version: '1.0.0' } } });
  for await (const line of lines) {
    const message = JSON.parse(line);
    if (message.id === 1) {
      send({ method: 'notifications/initialized' });
      send({ id: 2, method: 'tools/list' });
    } else if (message.id === 2) {
      tools = message.result?.tools ?? [];
      break;
    }
  }
} finally {
  child.stdin.end();
  if (child.exitCode === null && child.signalCode === null) await once(child, 'exit');
  clearTimeout(timer);
  await rm(home, { recursive: true, force: true });
}
if (timedOut) throw new Error('MCP smoke timed out after 30 seconds');
if (!tools?.some((tool) => tool.name === 'search')) {
  throw new Error(`tools/list did not return the search tool: ${JSON.stringify(tools)}`);
}
console.log(`MCP smoke passed: ${tools.length} tools`);
