"use strict";(self.webpackChunk=self.webpackChunk||[]).push([[414,745,578],{745:(e,n,t)=>{t.r(n),t.d(n,{CosmoSim:()=>k,default:()=>U,get_scale_length:()=>g,initSync:()=>R,initThreadPool:()=>v,wbg_rayon_PoolBuilder:()=>T,wbg_rayon_start_worker:()=>A});var r=t(578);let o;e=t.hmd(e);const _=new Array(128).fill(void 0);function i(e){return _[e]}_.push(void 0,null,!0,!1);let c=_.length;function u(e){const n=i(e);return function(e){e<132||(_[e]=c,c=e)}(e),n}const s="undefined"!=typeof TextDecoder?new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw Error("TextDecoder not available")}};"undefined"!=typeof TextDecoder&&s.decode();let a=null;function w(){return null!==a&&a.buffer===o.memory.buffer||(a=new Uint8Array(o.memory.buffer)),a}function b(e,n){return e>>>=0,s.decode(w().slice(e,e+n))}function f(e){c===_.length&&_.push(_.length+1);const n=c;return c=_[n],_[n]=e,n}function g(){return o.get_scale_length()}function l(e,n){try{return e.apply(this,n)}catch(e){o.__wbindgen_exn_store(f(e))}}let d=0;const y="undefined"!=typeof TextEncoder?new TextEncoder("utf-8"):{encode:()=>{throw Error("TextEncoder not available")}},m=function(e,n){const t=y.encode(e);return n.set(t),{read:e.length,written:t.length}};let p=null;function h(){return null!==p&&p.buffer===o.memory.buffer||(p=new Int32Array(o.memory.buffer)),p}function v(e){return u(o.initThreadPool(e))}function A(e){o.wbg_rayon_start_worker(e)}class k{static __wrap(e){e>>>=0;const n=Object.create(k.prototype);return n.__wbg_ptr=e,n}__destroy_into_raw(){const e=this.__wbg_ptr;return this.__wbg_ptr=0,e}free(){const e=this.__destroy_into_raw();o.__wbg_cosmosim_free(e)}constructor(e,n,t){const r=o.cosmosim_new(e,n,t);return k.__wrap(r)}simulate(e){o.cosmosim_simulate(this.__wbg_ptr,e)}get_position(){return u(o.cosmosim_get_position(this.__wbg_ptr))}get_velocity(){return u(o.cosmosim_get_velocity(this.__wbg_ptr))}}class T{static __wrap(e){e>>>=0;const n=Object.create(T.prototype);return n.__wbg_ptr=e,n}__destroy_into_raw(){const e=this.__wbg_ptr;return this.__wbg_ptr=0,e}free(){const e=this.__destroy_into_raw();o.__wbg_wbg_rayon_poolbuilder_free(e)}numThreads(){return o.wbg_rayon_poolbuilder_numThreads(this.__wbg_ptr)>>>0}receiver(){return o.wbg_rayon_poolbuilder_receiver(this.__wbg_ptr)}build(){o.wbg_rayon_poolbuilder_build(this.__wbg_ptr)}}function W(){const n={wbg:{}};return n.wbg.__wbindgen_object_drop_ref=function(e){u(e)},n.wbg.__wbg_crypto_c48a774b022d20ac=function(e){return f(i(e).crypto)},n.wbg.__wbindgen_is_object=function(e){const n=i(e);return"object"==typeof n&&null!==n},n.wbg.__wbg_process_298734cf255a885d=function(e){return f(i(e).process)},n.wbg.__wbg_versions_e2e78e134e3e5d01=function(e){return f(i(e).versions)},n.wbg.__wbg_node_1cd7a5d853dbea79=function(e){return f(i(e).node)},n.wbg.__wbindgen_is_string=function(e){return"string"==typeof i(e)},n.wbg.__wbg_msCrypto_bcb970640f50a1e8=function(e){return f(i(e).msCrypto)},n.wbg.__wbg_require_8f08ceecec0f4fee=function(){return l((function(){return f(e.require)}),arguments)},n.wbg.__wbindgen_is_function=function(e){return"function"==typeof i(e)},n.wbg.__wbindgen_string_new=function(e,n){return f(b(e,n))},n.wbg.__wbg_getRandomValues_37fa2ca9e4e07fab=function(){return l((function(e,n){i(e).getRandomValues(i(n))}),arguments)},n.wbg.__wbg_randomFillSync_dc1e9a60c158336d=function(){return l((function(e,n){i(e).randomFillSync(u(n))}),arguments)},n.wbg.__wbg_newnoargs_581967eacc0e2604=function(e,n){return f(new Function(b(e,n)))},n.wbg.__wbg_call_cb65541d95d71282=function(){return l((function(e,n){return f(i(e).call(i(n)))}),arguments)},n.wbg.__wbindgen_object_clone_ref=function(e){return f(i(e))},n.wbg.__wbg_self_1ff1d729e9aae938=function(){return l((function(){return f(self.self)}),arguments)},n.wbg.__wbg_window_5f4faef6c12b79ec=function(){return l((function(){return f(window.window)}),arguments)},n.wbg.__wbg_globalThis_1d39714405582d3c=function(){return l((function(){return f(globalThis.globalThis)}),arguments)},n.wbg.__wbg_global_651f05c6a0944d1c=function(){return l((function(){return f(t.g.global)}),arguments)},n.wbg.__wbindgen_is_undefined=function(e){return void 0===i(e)},n.wbg.__wbg_call_01734de55d61e11d=function(){return l((function(e,n,t){return f(i(e).call(i(n),i(t)))}),arguments)},n.wbg.__wbg_buffer_085ec1f694018c4f=function(e){return f(i(e).buffer)},n.wbg.__wbg_newwithbyteoffsetandlength_6da8e527659b86aa=function(e,n,t){return f(new Uint8Array(i(e),n>>>0,t>>>0))},n.wbg.__wbg_new_8125e318e6245eed=function(e){return f(new Uint8Array(i(e)))},n.wbg.__wbg_set_5cf90238115182c3=function(e,n,t){i(e).set(i(n),t>>>0)},n.wbg.__wbg_newwithbyteoffsetandlength_69193e31c844b792=function(e,n,t){return f(new Float32Array(i(e),n>>>0,t>>>0))},n.wbg.__wbg_new_d086a66d1c264b3f=function(e){return f(new Float32Array(i(e)))},n.wbg.__wbg_newwithlength_e5d69174d6984cd7=function(e){return f(new Uint8Array(e>>>0))},n.wbg.__wbg_subarray_13db269f57aa838d=function(e,n,t){return f(i(e).subarray(n>>>0,t>>>0))},n.wbg.__wbg_new_abda76e883ba8a5f=function(){return f(new Error)},n.wbg.__wbg_stack_658279fe44541cf6=function(e,n){const t=function(e,n,t){if(void 0===t){const t=y.encode(e),r=n(t.length,1)>>>0;return w().subarray(r,r+t.length).set(t),d=t.length,r}let r=e.length,o=n(r,1)>>>0;const _=w();let i=0;for(;i<r;i++){const n=e.charCodeAt(i);if(n>127)break;_[o+i]=n}if(i!==r){0!==i&&(e=e.slice(i)),o=t(o,r,r=i+3*e.length,1)>>>0;const n=w().subarray(o+i,o+r);i+=m(e,n).written}return d=i,o}(i(n).stack,o.__wbindgen_malloc,o.__wbindgen_realloc),r=d;h()[e/4+1]=r,h()[e/4+0]=t},n.wbg.__wbg_error_f851667af71bcfc6=function(e,n){let t,r;try{t=e,r=n,console.error(b(e,n))}finally{o.__wbindgen_free(t,r,1)}},n.wbg.__wbindgen_throw=function(e,n){throw new Error(b(e,n))},n.wbg.__wbindgen_module=function(){return f(M.__wbindgen_wasm_module)},n.wbg.__wbindgen_memory=function(){return f(o.memory)},n.wbg.__wbg_startWorkers_6fd3af285ea11136=function(e,n,t){return f((0,r.Q)(u(e),u(n),T.__wrap(t)))},n}function x(e,n){e.wbg.memory=n||new WebAssembly.Memory({initial:18,maximum:16384,shared:!0})}function E(e,n){return o=e.exports,M.__wbindgen_wasm_module=n,p=null,a=null,o.__wbindgen_start(),o}function R(e,n){if(void 0!==o)return o;const t=W();return x(t,n),e instanceof WebAssembly.Module||(e=new WebAssembly.Module(e)),E(new WebAssembly.Instance(e,t),e)}async function M(e,n){if(void 0!==o)return o;void 0===e&&(e=new URL(t(647),t.b));const r=W();("string"==typeof e||"function"==typeof Request&&e instanceof Request||"function"==typeof URL&&e instanceof URL)&&(e=fetch(e)),x(r,n);const{instance:_,module:i}=await async function(e,n){if("function"==typeof Response&&e instanceof Response){if("function"==typeof WebAssembly.instantiateStreaming)try{return await WebAssembly.instantiateStreaming(e,n)}catch(n){if("application/wasm"==e.headers.get("Content-Type"))throw n;console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",n)}const t=await e.arrayBuffer();return await WebAssembly.instantiate(t,n)}{const t=await WebAssembly.instantiate(e,n);return t instanceof WebAssembly.Instance?{instance:t,module:e}:t}}(await e,r);return E(_,i)}const U=M},578:(e,n,t)=>{function r(e,n){return new Promise((t=>{e.addEventListener("message",(function r({data:o}){null!=o&&o.type===n&&(e.removeEventListener("message",r),t(o))}))}))}let o;async function _(e,n,_){const i={type:"wasm_bindgen_worker_init",module:e,memory:n,receiver:_.receiver()};o=await Promise.all(Array.from({length:_.numThreads()},(async()=>{const e=new Worker(new URL(t.p+t.u(578),t.b),{type:void 0});return e.postMessage(i),await r(e,"wasm_bindgen_worker_ready"),e}))),_.build()}t.d(n,{Q:()=>_}),r(self,"wasm_bindgen_worker_init").then((async e=>{const n=await t.e(745).then(t.bind(t,745));await n.default(e.module,e.memory),postMessage({type:"wasm_bindgen_worker_ready"}),n.wbg_rayon_start_worker(e.receiver)}))},647:(e,n,t)=>{e.exports=t.p+"32972cc4ae17a592d46e.wasm"}}]);