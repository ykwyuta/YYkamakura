import {spawnSync} from 'node:child_process';
import {homedir} from 'node:os';
import {join} from 'node:path';

export const wasmTarget = 'wasm32-unknown-unknown';

// Node build images may lack Rust or may not source ~/.cargo/env.
export async function prepareRust({
  env = process.env, platform = process.platform, home = homedir(),
  spawn = spawnSync, download = fetch, log = console.log,
} = {}) {
  const buildEnv = {...env};
  const pathKey = Object.keys(buildEnv).find(key => key.toLowerCase() === 'path') || 'PATH';
  buildEnv[pathKey] = [join(env.CARGO_HOME || join(home, '.cargo'), 'bin'), buildEnv[pathKey]]
    .filter(Boolean).join(platform === 'win32' ? ';' : ':');
  const probe = (command, args) => spawn(command, args, {env: buildEnv, encoding: 'utf8'});
  const run = (command, args, options = {}) => {
    const result = spawn(command, args, {env: buildEnv, stdio: 'inherit', ...options});
    if (result.error) throw new Error(`Cannot run ${command}: ${result.error.message}`);
    if (result.status !== 0) throw new Error(`${command} failed (exit ${result.status ?? result.signal}).`);
  };

  let cargo = probe('cargo', ['--version']);
  if (cargo.error?.code === 'ENOENT') {
    if (!['linux', 'darwin'].includes(platform)) {
      throw new Error('Rust is missing. Install Rust from https://rust-lang.org/tools/install/ and retry npm run build.');
    }
    log('Cargo not found; installing Rust with the official rustup installer.');
    const response = await download('https://sh.rustup.rs', {signal: AbortSignal.timeout(60_000)});
    if (!response.ok) throw new Error(`Rust installer download failed: HTTP ${response.status}`);
    run('sh', ['-s', '--', '-y', '--profile', 'minimal', '--default-toolchain', 'stable', '--no-modify-path', '--target', wasmTarget], {
      input: await response.text(), stdio: ['pipe', 'inherit', 'inherit'],
    });
    cargo = probe('cargo', ['--version']);
  }
  if (cargo.error || cargo.status !== 0) {
    throw new Error(`Cargo is unavailable: ${cargo.error?.message || cargo.stderr || cargo.status}`);
  }
  log(cargo.stdout.trim());
  const targets = probe('rustup', ['target', 'list', '--installed']);
  if (targets.error || targets.status !== 0) {
    throw new Error('rustup is required to prepare the WebAssembly target. Install Rust from https://rust-lang.org/tools/install/.');
  }
  if (!targets.stdout.split(/\s+/).includes(wasmTarget)) run('rustup', ['target', 'add', wasmTarget]);
  return buildEnv;
}
