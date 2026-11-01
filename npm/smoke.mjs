// Starts the installed launcher with an empty, isolated cache and checks that
// the MCP handshake and tools/list succeed without a database.
// Usage: node npm/smoke.mjs INSTALL_DIR
import { spawn } from 'node:child_process';
import { once } from 'node:events';
import { mkdir, mkdtemp, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { createInterface } from 'node:readline';

const [installDir, ...extra] = process.argv.slice(2);
if (!installDir || extra.length) throw new Error('Usage: node npm/smoke.mjs INSTALL_DIR');
const launcher = join(resolve(installDir), 'node_modules/wotlk-database-mcp/bin/launcher.cjs');
const home = await mkdtemp(join(tmpdir(), 'wotlk-mcp-smoke-'));
// Windows resolves the cache through SHGetKnownFolderPath, which fails for a missing folder.
const localAppData = join(home, 'AppData', 'Local');
await mkdir(localAppData, { recursive: true });
const env = { ...process.env, HOME: home, USERPROFILE: home, XDG_CACHE_HOME: join(home, 'cache'), LOCALAPPDATA: localAppData, APPDATA: join(home, 'AppData', 'Roaming') };
delete env.WOTLK_DB;

const child = spawn(process.execPath, [launcher], { env, stdio: ['pipe', 'pipe', 'inherit'] });
const send = message => child.stdin.write(`${JSON.stringify({ jsonrpc: '2.0', ...message })}\n`);
const lines = createInterface({ input: child.stdout });
let timedOut = false;
const timer = setTimeout(() => {
  timedOut = true;
  lines.close();
  // The native child can keep stdout open after its launcher exits.
  child.stdout.destroy();
  child.kill();
}, 30_000);
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
if (!tools?.some(tool => tool.name === 'search')) {
  throw new Error(`tools/list did not return the search tool: ${JSON.stringify(tools)}`);
}
console.log(`MCP smoke passed: ${tools.length} tools`);
