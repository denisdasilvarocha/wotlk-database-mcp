// @ts-check
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { copyFile, mkdir, mkdtemp, readFile, rm, stat, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { test } from 'node:test';

const BUILD_SCRIPT = 'npm/build.mjs';

/**
 * Creates an isolated staging workspace.
 *
 * @param {string} root - The temporary workspace path.
 * @returns {Promise<string>} The source package version.
 */
const createWorkspace = async (root) => {
  const launcherManifest = JSON.parse(await readFile(resolve('npm/package.json'), 'utf8'));
  const version = launcherManifest.version;
  await writeFile(join(root, 'Cargo.toml'), `[package]\nname = "wotlk-database-mcp"\nversion = "${version}"\n`);
  await copyFile(resolve('LICENSE'), join(root, 'LICENSE'));

  const npmDirectory = join(root, 'npm');
  await mkdir(npmDirectory);
  await copyFile(resolve('npm/package.json'), join(npmDirectory, 'package.json'));
  await copyFile(resolve(BUILD_SCRIPT), join(npmDirectory, 'build.mjs'));

  return version;
};

/**
 * Requires the staging command to reject the input without changing packages.
 *
 * @param {string} workspace - The temporary workspace path.
 * @param {string[]} args - The command arguments.
 * @returns {Promise<void>} A promise that resolves after the failure checks.
 */
const expectBuildFailure = async (workspace, args) => {
  const result = spawnSync(process.execPath, [join(workspace, BUILD_SCRIPT), ...args], {
    cwd: workspace,
    encoding: 'utf8',
  });
  assert.equal(result.status, 1, result.stderr || result.stdout);
  await assert.rejects(stat(join(workspace, 'npm', 'LICENSE')), { code: 'ENOENT' });
};

test('rejects invalid release inputs before it stages any package', async () => {
  const root = await mkdtemp(join(tmpdir(), 'wotlk-pack-invalid-'));
  try {
    const version = await createWorkspace(root);
    const binary = join(root, 'fake-native-binary');
    await writeFile(binary, 'not executed');

    await expectBuildFailure(root, [`darwin-arm64=${binary}`, `darwin-arm64=${binary}`]);
    await expectBuildFailure(root, [`darwin-arm64=${binary}`, 'solaris-x64=/missing']);
    await expectBuildFailure(root, [`darwin-arm64=${binary}`, `linux-x64=${join(root, 'missing')}`]);

    const mismatchedVersion = version === '999.0.0' ? '999.0.1' : '999.0.0';
    await writeFile(join(root, 'Cargo.toml'), `[package]\nname = "wotlk-database-mcp"\nversion = "${mismatchedVersion}"\n`);
    await expectBuildFailure(root, [`darwin-arm64=${binary}`]);
  } finally {
    await rm(root, { recursive: true, force: true });
  }
});
