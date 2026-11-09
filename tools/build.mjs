import {spawnSync} from 'node:child_process';
import {mkdirSync, copyFileSync, writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {prepareRust} from './rust-toolchain.mjs';
const root=fileURLToPath(new URL('../',import.meta.url));
const env=await prepareRust();
const build=spawnSync('cargo',['build','--release','--locked','--lib','--target','wasm32-unknown-unknown','--config','profile.release.debug=false'],{cwd:root,env,stdio:'inherit'});
if(build.error) throw build.error;
if(build.status) process.exit(build.status);
mkdirSync(new URL('../dist/',import.meta.url),{recursive:true});
for(const file of ['index.html','main.js','browser-recovery.js','manifest.webmanifest','icon-192.png','icon-512.png','apple-touch-icon.png']) {
  copyFileSync(new URL('../web/'+file,import.meta.url),new URL('../dist/'+file,import.meta.url));
}
copyFileSync(new URL('../assets/app_icon.ico',import.meta.url),new URL('../dist/favicon.ico',import.meta.url));
copyFileSync(new URL('../target/wasm32-unknown-unknown/release/yykamakura.wasm',import.meta.url),new URL('../dist/yykamakura.wasm',import.meta.url));
writeFileSync(new URL('../dist/_headers',import.meta.url),'/\n  Cache-Control: no-cache\n/index.html\n  Cache-Control: no-cache\n/main.js\n  Cache-Control: no-cache\n/browser-recovery.js\n  Cache-Control: no-cache\n/yykamakura.wasm\n  Content-Type: application/wasm\n  Cache-Control: public, max-age=3600\n');
console.log('Kamakura Web build: dist/');
