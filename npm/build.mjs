import { mkdir, readFile, writeFile, copyFile, chmod, readdir, mkdtemp, rm, access } from 'node:fs/promises';
import { join, resolve, basename } from 'node:path';
import { tmpdir } from 'node:os';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';
import { spawnNpm } from './npm-command.mjs';

const here = fileURLToPath(new URL('.', import.meta.url));
const targets = {
  'darwin-arm64': 'aarch64-apple-darwin',
  'darwin-x64': 'x86_64-apple-darwin',
  'linux-x64': 'x86_64-unknown-linux-gnu',
  'linux-arm64': 'aarch64-unknown-linux-gnu',
  'win32-x64': 'x86_64-pc-windows-msvc',
};
const [version, inputArg, outputArg, platform, ...extra] = process.argv.slice(2);
if (!version || !inputArg || !outputArg || extra.length || !/^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?$/.test(version) || (platform && !targets[platform])) {
  throw new Error('Usage: node npm/build.mjs VERSION INPUT_DIR OUTPUT_DIR [PLATFORM]');
}
const input = resolve(inputArg);
const output = resolve(outputArg);
await mkdir(output, { recursive: true });
function run(command, args, cwd) {
  const options = { cwd, encoding: 'utf8' };
  const result = command === 'npm' ? spawnNpm(args, options) : spawnSync(command, args, options);
  if (result.error) throw result.error;
  if (result.status !== 0) throw new Error(`${command} failed: ${result.stderr || result.stdout}`);
  return result.stdout;
}
async function exists(path) { try { await access(path); return true; } catch { return false; } }
async function findBinary(directory, name) {
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    const path = join(directory, entry.name);
    if (entry.isFile() && entry.name === name) return path;
    if (entry.isDirectory()) { const found = await findBinary(path, name); if (found) return found; }
  }
}
async function pack(directory) {
  const info = JSON.parse(run('npm', ['pack', '--ignore-scripts', '--json', '--pack-destination', output], directory));
  console.log(join(output, Object.values(info)[0].filename));
}
for (const [key, target] of Object.entries(targets)) {
  if (platform && key !== platform) continue;
  const [os, cpu] = key.split('-');
  const name = `wotlk-database-mcp-${key}`;
  const binary = `wotlk-database-mcp${os === 'win32' ? '.exe' : ''}`;
  const directory = join(output, name);
  if (await exists(directory)) throw new Error(`Output package already exists: ${directory}`);
  await mkdir(join(directory, 'bin'), { recursive: true });
  let source;
  const hostCandidates = key === `${process.platform}-${process.arch}` ? [join(input, binary), join(input, 'release', binary), join(input, 'target', 'release', binary)] : [];
  for (const candidate of [join(input, target, 'release', binary), join(input, 'target', target, 'release', binary), join(input, `wotlk-database-mcp-${target}`, binary), ...hostCandidates]) {
    if (await exists(candidate)) { source = candidate; break; }
  }
  const temporary = await mkdtemp(join(tmpdir(), 'wotlk-npm-extract-'));
  try {
    if (!source) {
      const archive = (await readdir(input)).find(file => file.includes(target) && /\.(tar\.xz|zip)$/.test(file));
      if (!archive) throw new Error(`Missing binary or dist archive for ${target} in ${input}`);
      if (archive.endsWith('.zip') && process.platform !== 'win32') run('unzip', ['-q', join(input, archive), '-d', temporary]);
      // Git Bash puts GNU tar first on PATH, which reads `D:\...` as a remote host; Windows' bsdtar also reads zip.
      else run(process.platform === 'win32' ? join(process.env.SystemRoot ?? 'C:\\Windows', 'System32', 'tar.exe') : 'tar', ['-xf', join(input, archive), '-C', temporary]);
      source = await findBinary(temporary, binary);
      if (!source) throw new Error(`Missing ${binary} in ${basename(archive)}`);
    }
    await copyFile(source, join(directory, 'bin', binary));
    await chmod(join(directory, 'bin', binary), 0o755);
  } finally { await rm(temporary, { recursive: true, force: true }); }
  await writeFile(join(directory, 'package.json'), JSON.stringify({ name, version, description: `Native ${key} binary for wotlk-database-mcp`, repository: { type: 'git', url: 'git+https://github.com/denisdasilvarocha/wotlk-database-mcp.git' }, os: [os], cpu: [cpu], ...(os === 'linux' ? { libc: ['glibc'] } : {}), preferUnplugged: true, files: ['bin/'] }, null, 2) + '\n');
  await pack(directory);
}
const launcherDir = join(output, 'wotlk-database-mcp');
if (await exists(launcherDir)) throw new Error(`Output package already exists: ${launcherDir}`);
await mkdir(join(launcherDir, 'bin'), { recursive: true });
const manifest = JSON.parse(await readFile(join(here, 'package.json'), 'utf8'));
manifest.version = version;
manifest.optionalDependencies = Object.fromEntries(Object.keys(targets).map(key => [`wotlk-database-mcp-${key}`, version]));
await writeFile(join(launcherDir, 'package.json'), JSON.stringify(manifest, null, 2) + '\n');
await copyFile(join(here, 'launcher.cjs'), join(launcherDir, 'bin/launcher.cjs'));
await chmod(join(launcherDir, 'bin/launcher.cjs'), 0o755);
await pack(launcherDir);
