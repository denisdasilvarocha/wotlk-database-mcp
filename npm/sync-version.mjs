// @ts-check
import { readFile, writeFile } from 'node:fs/promises';

const npmManifestPath = new URL('./package.json', import.meta.url);
const cargoManifestPath = new URL('../Cargo.toml', import.meta.url);
const cargoLockPath = new URL('../Cargo.lock', import.meta.url);

/**
 * Updates the selected Cargo package version in a TOML manifest.
 *
 * @param {string} content - The Cargo manifest text.
 * @param {string} version - The package version to set.
 * @returns {string} The updated Cargo manifest text.
 * @throws When the package version field is missing.
 */
const updateCargoManifestVersion = (content, version) => {
  let currentSection = '';
  let foundVersion = false;
  const lines = content.split(/\r?\n/).map((line) => {
    const section = /^\[([^\]]+)\]$/.exec(line);
    if (section) {
      currentSection = section[1];
      return line;
    }

    if (currentSection !== 'package' || !/^version\s*=\s*"[^"]+"\s*$/.test(line)) {
      return line;
    }

    foundVersion = true;
    return `version = "${version}"`;
  });

  if (!foundVersion) {
    throw new Error('Could not find [package].version in Cargo.toml');
  }

  return lines.join('\n');
};

/**
 * Updates the root server package version in Cargo.lock.
 *
 * @param {string} content - The Cargo lockfile text.
 * @param {string} version - The package version to set.
 * @returns {string} The updated Cargo lockfile text.
 * @throws When the root server package version is missing.
 */
const updateCargoLockVersion = (content, version) => {
  const lines = content.split(/\r?\n/);
  let isServerPackage = false;
  let foundVersion = false;

  for (const [index, line] of lines.entries()) {
    if (line === '[[package]]') {
      isServerPackage = lines[index + 1] === 'name = "wotlk-database-mcp"';
      continue;
    }

    if (isServerPackage && /^version\s*=\s*"[^"]+"$/.test(line)) {
      lines[index] = `version = "${version}"`;
      foundVersion = true;
      isServerPackage = false;
    }
  }

  if (!foundVersion) {
    throw new Error('Could not find the wotlk-database-mcp package version in Cargo.lock');
  }

  return lines.join('\n');
};

const npmManifest = JSON.parse(await readFile(npmManifestPath, 'utf8'));
if (npmManifest.name !== 'wotlk-database-mcp' || typeof npmManifest.version !== 'string') {
  throw new Error('npm/package.json must define the wotlk-database-mcp package name and version');
}

const [cargoManifest, cargoLock] = await Promise.all([
  readFile(cargoManifestPath, 'utf8'),
  readFile(cargoLockPath, 'utf8'),
]);
const updatedCargoManifest = updateCargoManifestVersion(cargoManifest, npmManifest.version);
const updatedCargoLock = updateCargoLockVersion(cargoLock, npmManifest.version);

await Promise.all([
  writeFile(cargoManifestPath, updatedCargoManifest),
  writeFile(cargoLockPath, updatedCargoLock),
]);
