// @ts-check
// Stages raw native binaries in the platform workspaces without packing them.
// Usage: node npm/build.mjs PLATFORM=PATH...
import { chmod, copyFile, mkdir, readFile, stat } from 'node:fs/promises';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const PLATFORMS = [
  'darwin-arm64',
  'darwin-x64',
  'linux-x64',
  'linux-arm64',
  'win32-x64',
];
const usage = 'Usage: node npm/build.mjs PLATFORM=PATH...';
const sources = process.argv.slice(2);
if (sources.length === 0) {
  throw new Error(usage);
}

const npmDirectory = dirname(fileURLToPath(import.meta.url));
const repositoryDirectory = resolve(npmDirectory, '..');
const npmManifest = JSON.parse(await readFile(join(npmDirectory, 'package.json'), 'utf8'));
if (npmManifest.name !== 'wotlk-database-mcp' || typeof npmManifest.version !== 'string') {
  throw new Error('npm/package.json must define the wotlk-database-mcp package name and version');
}

const cargoToml = await readFile(join(repositoryDirectory, 'Cargo.toml'), 'utf8');
const cargoPackage = cargoToml.split(/^\[package\]\s*$/m)[1]?.split(/^\[/m, 1)[0] ?? '';
const cargoVersion = /^\s*version\s*=\s*"([^"]+)"\s*$/m.exec(cargoPackage)?.[1];
if (!cargoVersion) {
  throw new Error('Could not read [package].version from Cargo.toml');
}
if (npmManifest.version !== cargoVersion) {
  throw new Error(`npm package version ${npmManifest.version} does not match Cargo version ${cargoVersion}`);
}

const stagedSources = [];
const seenPlatforms = new Set();
for (const source of sources) {
  const separator = source.indexOf('=');
  if (separator < 1 || separator === source.length - 1) {
    throw new Error(`Expected PLATFORM=PATH, got ${source}`);
  }

  const platform = source.slice(0, separator);
  const sourcePath = resolve(source.slice(separator + 1));
  if (!PLATFORMS.includes(platform)) {
    throw new Error(`Unknown platform ${platform}. Supported platforms: ${PLATFORMS.join(', ')}`);
  }
  if (seenPlatforms.has(platform)) {
    throw new Error(`Platform ${platform} was provided more than once`);
  }
  seenPlatforms.add(platform);

  const fileInfo = await stat(sourcePath);
  if (!fileInfo.isFile()) {
    throw new Error(`Input binary is not a file: ${sourcePath}`);
  }

  stagedSources.push({
    platform,
    sourcePath,
    binaryName: platform === 'win32-x64' ? 'wotlk-database-mcp.exe' : 'wotlk-database-mcp',
  });
}

const licensePath = join(repositoryDirectory, 'LICENSE');
await copyFile(licensePath, join(npmDirectory, 'LICENSE'));

for (const { platform, sourcePath, binaryName } of stagedSources) {
  const packageDirectory = join(npmDirectory, 'platforms', platform);
  const binDirectory = join(packageDirectory, 'bin');
  const binaryPath = join(binDirectory, binaryName);
  await mkdir(binDirectory, { recursive: true });
  await copyFile(sourcePath, binaryPath);
  await chmod(binaryPath, 0o755);
  await copyFile(licensePath, join(packageDirectory, 'LICENSE'));
  console.log(`Staged ${platform}: ${binaryPath}`);
}
