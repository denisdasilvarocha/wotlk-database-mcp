import { spawn } from 'node:child_process';
import { mkdtemp, rm, access } from 'node:fs/promises';
import { join, resolve, isAbsolute } from 'node:path';
import { tmpdir } from 'node:os';
import { createInterface } from 'node:readline';
const [installArg, flag, database, ...extra] = process.argv.slice(2);
if (!installArg || (flag && (flag !== '--db' || !database || !isAbsolute(database))) || extra.length) {
  throw new Error('Usage: node npm/smoke.mjs INSTALL_DIR [--db ABS_PATH]');
}
const launcher = join(resolve(installArg), 'node_modules/wotlk-database-mcp/bin/launcher.cjs');
await access(launcher);
const isolated = await mkdtemp(join(tmpdir(), 'wotlk-mcp-smoke-'));
const env = { ...process.env, HOME: isolated, USERPROFILE: isolated, XDG_CACHE_HOME: join(isolated, 'cache'), LOCALAPPDATA: join(isolated, 'local'), APPDATA: join(isolated, 'appdata') };
delete env.WOTLK_DB;
const child = spawn(process.execPath, [launcher, ...(database ? ['--db', database] : [])], { env, stdio: ['pipe', 'pipe', 'pipe'] });
let stderr = '';
child.stderr.on('data', chunk => { stderr += chunk; });
const pending = new Map();
let nextId = 1;
let failure;
const lines = createInterface({ input: child.stdout });
function fail(error) { failure = error; for (const waiter of pending.values()) waiter.reject(error); pending.clear(); }
lines.on('line', line => {
  let message;
  try { message = JSON.parse(line); } catch { fail(new Error(`Non-JSON MCP stdout: ${line}`)); return; }
  const waiter = pending.get(message.id);
  if (waiter) {
    pending.delete(message.id);
    if (message.error) waiter.reject(new Error(JSON.stringify(message.error)));
    else waiter.resolve(message.result);
  }
});
child.on('error', fail);
child.on('exit', (code, signal) => fail(new Error(`Launcher exited (${code ?? signal}): ${stderr}`)));
child.stdin.on('error', fail);
function send(method, params, notification = false) {
  if (failure) return Promise.reject(failure);
  const id = notification ? undefined : nextId++;
  const promise = notification ? Promise.resolve() : new Promise((resolve, reject) => pending.set(id, { resolve, reject }));
  child.stdin.write(JSON.stringify({ jsonrpc: '2.0', ...(notification ? {} : { id }), method, params }) + '\n');
  return promise;
}
const timeout = setTimeout(() => fail(new Error(`MCP smoke timed out: ${stderr}`)), 30000);
try {
  const initialized = await send('initialize', { protocolVersion: '2024-11-05', capabilities: {}, clientInfo: { name: 'npm-release-smoke', version: '1.0.0' } });
  if (!initialized?.serverInfo) throw new Error('initialize did not return serverInfo');
  await send('notifications/initialized', {}, true);
  const result = await send('tools/list', {});
  if (!result?.tools?.some(tool => tool.name === 'search')) throw new Error('tools/list does not contain search');
  console.log(`MCP smoke passed (${database ? 'explicit database' : 'empty isolated cache'}): ${result.tools.length} tools`);
} finally {
  clearTimeout(timeout);
  child.kill();
  if (child.exitCode === null && child.signalCode === null) await new Promise(resolve => child.once('exit', resolve));
  lines.close();
  await rm(isolated, { recursive: true, force: true });
}
