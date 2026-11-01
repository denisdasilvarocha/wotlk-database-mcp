// Stages and packs the npm packages: one per native binary, plus the launcher.
// Usage: node npm/build.mjs VERSION OUTPUT_DIR PLATFORM=PATH...
// PATH is the server binary, or the dist release archive (.tar.xz or .zip) that holds it.
import { execFileSync, execSync } from 'node:child_process';
import { chmod, copyFile, mkdir, mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { basename, join, resolve } from 'node:path';

const platforms = ['darwin-arm64', 'darwin-x64', 'linux-x64', 'linux-arm64', 'win32-x64'];
const repository = { type: 'git', url: 'git+https://github.com/denisdasilvarocha/wotlk-database-mcp.git' };
// Git Bash puts GNU tar first on PATH; the Windows bsdtar reads zip and drive-letter paths.
const tar = process.platform === 'win32' ? join(process.env.SystemRoot ?? 'C:\\Windows', 'System32', 'tar.exe') : 'tar';

const [version, outputArg, ...sources] = process.argv.slice(2);
if (!version || !outputArg || sources.length === 0) {
  throw new Error('Usage: node npm/build.mjs VERSION OUTPUT_DIR PLATFORM=PATH...');
}
const output = resolve(outputArg);
// Fails when the directory exists, so stale packages are never packed again.
await mkdir(output);

const writeManifest = (directory, manifest) =>
  writeFile(join(directory, 'package.json'), `${JSON.stringify(manifest, null, 2)}\n`);

// dist archives hold `<archive name>/<binary>` (.tar.xz) or `<binary>` (.zip).
const extract = (archive, binary, directory) => {
  if (archive.endsWith('.zip')) {
    // GNU tar on Linux cannot read zip.
    if (process.platform === 'linux') execFileSync('unzip', ['-q', archive, '-d', directory]);
    else execFileSync(tar, ['-xf', archive, '-C', directory]);
    return join(directory, binary);
  }
  execFileSync(tar, ['-xf', archive, '-C', directory]);
  return join(directory, basename(archive, '.tar.xz'), binary);
};

const packages = [];
for (const source of sources) {
  const [, platform, path] = /^([^=]+)=(.+)$/.exec(source) ?? [];
  if (!platforms.includes(platform)) {
    throw new Error(`Expected PLATFORM=PATH with PLATFORM in ${platforms.join(', ')}, got ${source}`);
  }
  const [os, cpu] = platform.split('-');
  const binary = os === 'win32' ? 'wotlk-database-mcp.exe' : 'wotlk-database-mcp';
  const name = `wotlk-database-mcp-${platform}`;
  const bin = join(output, name, 'bin');
  // Non-recursive: a repeated platform fails here.
  await mkdir(join(output, name));
  await mkdir(bin);
  if (/\.(tar\.xz|zip)$/.test(path)) {
    const scratch = await mkdtemp(join(tmpdir(), 'wotlk-npm-'));
    try {
      await copyFile(extract(resolve(path), binary, scratch), join(bin, binary));
    } finally {
      await rm(scratch, { recursive: true, force: true });
    }
  } else {
    await copyFile(path, join(bin, binary));
  }
  await chmod(join(bin, binary), 0o755);
  await writeManifest(join(output, name), {
    name, version, description: `Native ${platform} binary for wotlk-database-mcp`, repository,
    os: [os], cpu: [cpu], ...(os === 'linux' && { libc: ['glibc'] }), preferUnplugged: true, files: ['bin/'],
  });
  packages.push(name);
}

const launcher = JSON.parse(await readFile(new URL('package.json', import.meta.url), 'utf8'));
const launcherDir = join(output, launcher.name);
await mkdir(join(launcherDir, 'bin'), { recursive: true });
await copyFile(new URL('launcher.cjs', import.meta.url), join(launcherDir, 'bin', 'launcher.cjs'));
await chmod(join(launcherDir, 'bin', 'launcher.cjs'), 0o755);
await writeManifest(launcherDir, {
  ...launcher, version,
  optionalDependencies: Object.fromEntries(platforms.map(platform => [`wotlk-database-mcp-${platform}`, version])),
});
packages.push(launcher.name);

// A shell lets Windows find npm.cmd. Package names are fixed, so nothing needs quoting.
execSync(`npm pack ${packages.map(name => `./${name}`).join(' ')}`, { cwd: output, stdio: 'inherit' });
