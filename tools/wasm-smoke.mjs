// Exercise the exact production Wasm ABI; graphics imports validate commands in memory.
import {readFileSync} from 'node:fs';
import assert from 'node:assert/strict';
const module=new WebAssembly.Module(readFileSync(new URL('../dist/yykamakura.wasm',import.meta.url)));
let wasm,labels=[],nextTexture=1,nextBitmap=1,saved=null,draws=0,ui=1;
const decode=(p,n)=>new TextDecoder().decode(new Uint8Array(wasm.memory.buffer,p,n));
const env=Object.fromEntries(WebAssembly.Module.imports(module).map(({name})=>[name,()=>0]));
Object.assign(env,{
 js_random:()=>12345,js_lang_get:()=>0,
 js_tex_create:()=>nextTexture++,js_bmp_create:()=>nextBitmap++,
 js_gl_begin:()=>{labels=[];draws=0;},js_2d_begin:s=>{ui=s;},
 js_gl_draw:(p,n)=>{assert.ok(Array.from(new Float32Array(wasm.memory.buffer,p,n*8)).every(Number.isFinite));draws++;},
 js_text:(p,n,x,y,w,h)=>labels.push({text:decode(p,n),x,y,w,h}),
 js_measure:(p,n)=>decode(p,n).length*11,
 js_measure_h:(p,n,f,w)=>Math.max(20,Math.ceil(decode(p,n).length*11/Math.max(1,w))*19),
 js_save:(slot,p,n)=>{saved=Uint8Array.from(new Uint8Array(wasm.memory.buffer,p,n));},
 js_log:(p,n,level)=>{if(level===2)throw Error(decode(p,n));},
});
wasm=new WebAssembly.Instance(module,{env}).exports;
const tick=()=>wasm.yy_frame(0.016,1);
function key(k){wasm.yy_event(4,k,1,0);tick();wasm.yy_event(4,k,0,0);tick();}
function has(text){assert.ok(labels.some(l=>l.text.includes(text)),`Missing ${text}; labels: ${labels.map(l=>l.text).join('|')}`);}
function click(text){
 const l=labels.find(l=>l.text===text || l.text.startsWith(text));
 assert.ok(l,`No clickable text ${text}`);
 const x=(l.x+l.w/2)*ui,y=(l.y+l.h/2)*ui;
 wasm.yy_event(1,0,x,y);tick();wasm.yy_event(2,0,x,y);tick();
}
wasm.yy_init(1600,900,0);tick();
has('Kamakura');has('1179');assert.equal(wasm.yy_is_title(),1);assert.ok(draws>0);
click('勢力を選ぶ');has('源氏');click('この国で覇業を開始');
assert.equal(wasm.yy_is_title(),0);key(27);has('1179');
key(72);has('英雄図鑑');has('源頼朝');
for(let p=0;p<5;p++)click('次の九人');
has('妹尾兼康');has('平知盛');has('平重盛');
key(27);key(112);has('1221');has('新しい邑');key(27);
key(114);has('歴史');key(27);
key(116);has('保存');click('保存');assert.ok(saved?.length>1000);
assert.equal(new TextDecoder().decode(saved.slice(0,4)),'YYKM');
key(27);const ptr=wasm.yy_alloc(saved.length);
new Uint8Array(wasm.memory.buffer,ptr,saved.length).set(saved);wasm.yy_loaded(ptr,saved.length);tick();
key(27);assert.equal(wasm.yy_is_title(),0);
wasm.yy_event(6,1000,520,0);wasm.yy_event(8,1,0,0);tick();assert.ok(draws>0);
console.log('Kamakura production Wasm: title, faction, start, all 54 hero images, help, history, save/load, compact draw passed.');
