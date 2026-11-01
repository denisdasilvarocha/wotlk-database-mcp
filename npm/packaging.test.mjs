import { test } from 'node:test';
import assert from 'node:assert/strict';
import { mkdtemp, mkdir, writeFile, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { spawnSync } from 'node:child_process';

const build = resolve('npm/build.mjs');
const targets = {
  'darwin-arm64': 'aarch64-apple-darwin',
  'darwin-x64': 'x86_64-apple-darwin',
  'linux-x64': 'x86_64-unknown-linux-gnu',
  'linux-arm64': 'aarch64-unknown-linux-gnu',
  'win32-x64': 'x86_64-pc-windows-msvc',
};
function run(command, args, options = {}) {
  const result = spawnSync(command, args, { encoding: 'utf8', ...options });
  assert.equal(result.status, 0, result.stderr || result.stdout);
  return result;
}
test('packs binaries and dist archives into packages that install without scripts', async () => {
  const root = await mkdtemp(join(tmpdir(), 'wotlk-pack-test-'));
  try {
    const output = join(root, 'packed');
    const host = `${process.platform}-${process.arch}`;
    const sources = [];
    for (const [platform, target] of Object.entries(targets)) {
      const stem = `wotlk-database-mcp-${target}`;
      const directory = join(root, stem);
      const binary = platform === 'win32-x64' ? 'wotlk-database-mcp.exe' : 'wotlk-database-mcp';
      await mkdir(directory);
      const path = join(directory, binary);
      await writeFile(path, '#!/bin/sh\nprintf "%s\\n" "$@"\nexit 23\n', { mode: 0o755 });
      let source = path;
      if (platform === host) {
        source = join(root, `${stem}.tar.xz`);
        run('tar', ['-cJf', source, stem], { cwd: root });
      } else if (platform === 'win32-x64') {
        source = join(root, `${stem}.zip`);
        run('zip', ['-qj', source, path]);
      }
      sources.push(`${platform}=${source}`);
    }
    run(process.execPath, [build, '1.2.3', output, ...sources]);
    for (const platform of Object.keys(targets)) {
      const binary = platform === 'win32-x64' ? 'wotlk-database-mcp.exe' : 'wotlk-database-mcp';
      const tarball = join(output, `wotlk-database-mcp-${platform}-1.2.3.tgz`);
      const contents = run('tar', ['-tzf', tarball]).stdout.trim().split('\n').filter(line => !line.endsWith('/')).sort();
      assert.deepEqual(contents, [`package/bin/${binary}`, 'package/package.json']);
    }
    const launcher = join(output, 'wotlk-database-mcp-1.2.3.tgz');
    const contents = run('tar', ['-tzf', launcher]).stdout.trim().split('\n').filter(line => !line.endsWith('/')).sort();
    assert.deepEqual(contents, ['package/bin/launcher.cjs', 'package/package.json']);
    const install = join(root, 'install');
    await mkdir(install);
    await writeFile(join(install, 'package.json'), '{"private":true}\n');
    run('npm', ['install', '--ignore-scripts', '--no-audit', '--no-fund', '--offline', join(output, `wotlk-database-mcp-${host}-1.2.3.tgz`), launcher], { cwd: install });
    const child = spawnSync(process.execPath, [join(install, 'node_modules/wotlk-database-mcp/bin/launcher.cjs'), '--db', '/path with spaces/db'], { encoding: 'utf8' });
    assert.equal(child.status, 23, child.stderr);
    assert.equal(child.stdout, '--db\n/path with spaces/db\n');
  } finally {
    await rm(root, { recursive: true, force: true });
  }
});
