import {test} from 'node:test';
import assert from 'node:assert/strict';
import {prepareRust, wasmTarget} from './rust-toolchain.mjs';

const ok = (stdout = '') => ({status: 0, stdout});
const missing = () => ({error: Object.assign(new Error('not found'), {code: 'ENOENT'})});
function fixture(handler, download) {
  const calls = [];
  return {
    calls,
    options: {
      env: {PATH: '/usr/bin', CARGO_HOME: '/cache/cargo'}, platform: 'linux',
      log() {},
      spawn(command, args, options) {
        calls.push({command, args, options});
        return handler(command, args, options);
      },
      download: download || (() => { throw new Error('Unexpected download'); }),
    },
  };
}

test('existing Cargo and Wasm target need no installation and expose Cargo bin', async () => {
  const f = fixture(command => ok(command === 'cargo' ? 'cargo 1.92\n' : wasmTarget));
  const env = await prepareRust(f.options);
  assert.equal(f.calls.length, 2);
  assert.match(env.PATH, /cargo[\\/]bin:\/usr\/bin$/);
  assert.equal(f.options.env.PATH, '/usr/bin');
});

test('missing target is added before building', async () => {
  const f = fixture((command, args) => ok(command === 'cargo' ? 'cargo 1.92' : 'x86_64-unknown-linux-gnu'));
  await prepareRust(f.options);
  assert.deepEqual(f.calls.at(-1).args, ['target', 'add', wasmTarget]);
});

test('Cargo absent in Node-only environment downloads and runs official installer', async () => {
  let installed = false;
  const f = fixture(command => {
    if (command === 'sh') { installed = true; return ok(); }
    if (command === 'cargo') return installed ? ok('cargo 1.92') : missing();
    return ok(wasmTarget);
  }, async url => {
    assert.equal(url, 'https://sh.rustup.rs');
    return {ok: true, text: async () => '# official installer'};
  });
  await prepareRust(f.options);
  const installer = f.calls.find(call => call.command === 'sh');
  assert.equal(installer.options.input, '# official installer');
  assert.ok(installer.args.includes('--no-modify-path'));
  assert.deepEqual(installer.args.slice(-2), ['--target', wasmTarget]);
  assert.equal(installed, true);
});

test('download failure stops setup', async () => {
  const f = fixture(missing, async () => ({ok: false, status: 503}));
  await assert.rejects(prepareRust(f.options), /HTTP 503/);
  assert.equal(f.calls.length, 1);
});

test('installer failure stops setup', async () => {
  const f = fixture(command => command === 'sh' ? {status: 1} : missing(),
    async () => ({ok: true, text: async () => ''}));
  await assert.rejects(prepareRust(f.options), /sh failed \(exit 1\)/);
});

test('broken existing Cargo is reported without installing over it', async () => {
  const f = fixture(() => ({status: 1, stderr: 'toolchain broken'}));
  await assert.rejects(prepareRust(f.options), /toolchain broken/);
});

test('target installation errors are not ignored', async () => {
  const f = fixture((command, args) => args.includes('add') ? {status: 1} : ok('cargo 1.92'));
  await assert.rejects(prepareRust(f.options), /rustup failed/);
});

test('Windows missing Cargo gives installation instructions', async () => {
  const f = fixture(missing);
  await assert.rejects(prepareRust({...f.options, platform: 'win32'}), /Install Rust/);
});
