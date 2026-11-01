import { test } from 'node:test';
import assert from 'node:assert/strict';
import { mkdtemp, mkdir, writeFile, readFile, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { spawnSync } from 'node:child_process';
import vm from 'node:vm';

const build = resolve('npm/build.mjs');
const platforms = {
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
test('packaging CLI stamps all packages and installs the host with scripts disabled', async () => {
  const root = await mkdtemp(join(tmpdir(), 'wotlk-pack-test-'));
  try {
    const input = join(root, 'input');
    const output = join(root, 'output');
    for (const [platform, target] of Object.entries(platforms)) {
      const dir = join(input, target, 'release');
      await mkdir(dir, { recursive: true });
      await writeFile(join(dir, `wotlk-database-mcp${platform.startsWith('win32') ? '.exe' : ''}`), '#!/bin/sh\nprintf "%s\\n" "$@"\nexit 23\n', { mode: 0o755 });
    }
    run(process.execPath, [build, '1.2.3', input, output]);
    const launcher = JSON.parse(await readFile(join(output, 'wotlk-database-mcp/package.json')));
    assert.deepEqual(Object.values(launcher.optionalDependencies), Array(5).fill('1.2.3'));
    assert.deepEqual(launcher.repository, { type: 'git', url: 'git+https://github.com/denisdasilvarocha/wotlk-database-mcp.git' });
    for (const platform of Object.keys(platforms)) {
      const manifest = JSON.parse(await readFile(join(output, `wotlk-database-mcp-${platform}/package.json`)));
      assert.equal(manifest.preferUnplugged, true);
      assert.deepEqual(manifest.repository, { type: 'git', url: 'git+https://github.com/denisdasilvarocha/wotlk-database-mcp.git' });
      assert.deepEqual(manifest.os, [platform.split('-')[0]]);
      assert.deepEqual(manifest.cpu, [platform.split('-')[1]]);
      if (platform.startsWith('linux')) assert.deepEqual(manifest.libc, ['glibc']);
    }
    run(process.execPath, [resolve('npm/check-packages.mjs'), output]);
    const host = `${process.platform}-${process.arch}`;
    const install = join(root, 'install');
    await mkdir(install);
    await writeFile(join(install, 'package.json'), '{"private":true}\n');
    run('npm', ['install', '--ignore-scripts', '--no-audit', '--no-fund', '--offline', join(output, `wotlk-database-mcp-${host}-1.2.3.tgz`), join(output, 'wotlk-database-mcp-1.2.3.tgz')], { cwd: install });
    const child = spawnSync(process.execPath, [join(install, 'node_modules/wotlk-database-mcp/bin/launcher.cjs'), '--db', '/path with spaces/db'], { encoding: 'utf8' });
    assert.equal(child.status, 23, child.stderr);
    assert.equal(child.stdout, '--db\n/path with spaces/db\n');
    const hostOutput = join(root, 'host-output');
    run(process.execPath, [build, '1.2.3', input, hostOutput, host]);
    await readFile(join(hostOutput, `wotlk-database-mcp-${host}-1.2.3.tgz`));
    const archives = join(root, 'archives');
    await mkdir(archives);
    for (const [key, target] of Object.entries(platforms)) {
      const dir = join(input, target, 'release');
      if (key === 'win32-x64') run('zip', ['-q', join(archives, `wotlk-database-mcp-${target}.zip`), 'wotlk-database-mcp.exe'], { cwd: dir });
      else run('tar', ['-cJf', join(archives, `wotlk-database-mcp-${target}.tar.xz`), '-C', dir, 'wotlk-database-mcp']);
    }
    run(process.execPath, [build, '1.2.3', archives, join(root, 'archive-output')]);
    const bad = join(root, 'bad');
    await mkdir(join(bad, 'package'), { recursive: true });
    await writeFile(join(bad, 'package/package.json'), JSON.stringify({ name: 'sqlite-content-test', version: '1.0.0' }));
    await writeFile(join(bad, 'package/wotlk.sqlite'), 'not shipped');
    const rejected = spawnSync(process.execPath, [resolve('npm/check-packages.mjs'), bad], { encoding: 'utf8' });
    assert.notEqual(rejected.status, 0);
    assert.match(rejected.stderr, /contains SQLite/);
  } finally { await rm(root, { recursive: true, force: true }); }
});
test('unsupported platforms explain supported choices without resolving a binary', async () => {
  const source = await readFile(resolve('npm/launcher.cjs'), 'utf8');
  let error = '';
  let status;
  assert.throws(() => vm.runInNewContext(source, {
    require() { throw new Error('must not resolve unsupported binary'); },
    process: { platform: 'freebsd', arch: 'riscv64', exit(code) { status = code; throw new Error('exit'); } },
    console: { error(message) { error = message; } },
  }, { filename: 'launcher.cjs' }), /exit/);
  assert.equal(status, 1);
  assert.match(error, /freebsd-riscv64/);
  assert.match(error, /darwin-arm64.*darwin-x64.*linux-x64.*linux-arm64.*win32-x64/);
});
