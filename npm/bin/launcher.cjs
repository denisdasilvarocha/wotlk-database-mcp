#!/usr/bin/env node
'use strict';
const supported = ['darwin-arm64', 'darwin-x64', 'linux-x64', 'linux-arm64', 'win32-x64'];
const platform = `${process.platform}-${process.arch}`;
if (!supported.includes(platform)) {
  console.error(`Unsupported platform ${platform}. Supported platforms: ${supported.join(', ')} (Linux requires glibc).`);
  process.exit(1);
}
const { spawnSync } = require('node:child_process');
const binary = `wotlk-database-mcp${process.platform === 'win32' ? '.exe' : ''}`;
let executable;
try {
  executable = require.resolve(`wotlk-database-mcp-${platform}/bin/${binary}`);
} catch (error) {
  console.error(`Cannot find the native binary for ${platform}. Reinstall wotlk-database-mcp with optional dependencies enabled${process.platform === 'linux' ? ' on a glibc-based distribution' : ''}. ${error.message}`);
  process.exit(1);
}
const child = spawnSync(executable, process.argv.slice(2), { stdio: 'inherit' });
if (child.error) {
  console.error(`Cannot start ${executable}: ${child.error.message}`);
  process.exit(1);
}
if (child.signal) {
  process.kill(process.pid, child.signal);
} else {
  process.exit(child.status ?? 1);
}
