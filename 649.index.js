"use strict";(self.webpackChunk=self.webpackChunk||[]).push([[649],{649:(n,e,t)=>{t.r(e),t.d(e,{CosmoSim:()=>T,default:()=>S,get_scale_length:()=>g,initSync:()=>E,initThreadPool:()=>A,wbg_rayon_PoolBuilder:()=>W,wbg_rayon_start_worker:()=>v});var r=t(363);let o;n=t.hmd(n);const _=new Array(128).fill(void 0);function i(n){return _[n]}_.push(void 0,null,!0,!1);let c=_.length;function u(n){const e=i(n);return function(n){n<132||(_[n]=c,c=n)}(n),e}const s="undefined"!=typeof TextDecoder?new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw Error("TextDecoder not available")}};"undefined"!=typeof TextDecoder&&s.decode();let a=null;function b(){return null!==a&&a.buffer===o.memory.buffer||(a=new Uint8Array(o.memory.buffer)),a}function w(n,e){return n>>>=0,s.decode(b().slice(n,n+e))}function f(n){c===_.length&&_.push(_.length+1);const e=c;return c=_[e],_[e]=n,e}function g(){return o.get_scale_length()}function l(n,e){try{return n.apply(this,e)}catch(n){o.__wbindgen_exn_store(f(n))}}let d=0;const y="undefined"!=typeof TextEncoder?new TextEncoder("utf-8"):{encode:()=>{throw Error("TextEncoder not available")}},m=function(n,e){const t=y.encode(n);return e.set(t),{read:n.length,written:t.length}};let p=null;function h(){return null!==p&&p.buffer===o.memory.buffer||(p=new Int32Array(o.memory.buffer)),p}function A(n){return u(o.initThreadPool(n))}function v(n){o.wbg_rayon_start_worker(n)}class T{static __wrap(n){n>>>=0;const e=Object.create(T.prototype);return e.__wbg_ptr=n,e}__destroy_into_raw(){const n=this.__wbg_ptr;return this.__wbg_ptr=0,n}free(){const n=this.__destroy_into_raw();o.__wbg_cosmosim_free(n)}constructor(n,e,t,r,_,i){const c=o.cosmosim_new(n,e,t,r,_,i);return T.__wrap(c)}simulate(n){o.cosmosim_simulate(this.__wbg_ptr,n)}get_position(){return u(o.cosmosim_get_position(this.__wbg_ptr))}get_velocity(){return u(o.cosmosim_get_velocity(this.__wbg_ptr))}}class W{static __wrap(n){n>>>=0;const e=Object.create(W.prototype);return e.__wbg_ptr=n,e}__destroy_into_raw(){const n=this.__wbg_ptr;return this.__wbg_ptr=0,n}free(){const n=this.__destroy_into_raw();o.__wbg_wbg_rayon_poolbuilder_free(n)}numThreads(){return o.wbg_rayon_poolbuilder_numThreads(this.__wbg_ptr)>>>0}receiver(){return o.wbg_rayon_poolbuilder_receiver(this.__wbg_ptr)}build(){o.wbg_rayon_poolbuilder_build(this.__wbg_ptr)}}function k(){const e={wbg:{}};return e.wbg.__wbindgen_object_drop_ref=function(n){u(n)},e.wbg.__wbg_crypto_c48a774b022d20ac=function(n){return f(i(n).crypto)},e.wbg.__wbindgen_is_object=function(n){const e=i(n);return"object"==typeof e&&null!==e},e.wbg.__wbg_process_298734cf255a885d=function(n){return f(i(n).process)},e.wbg.__wbg_versions_e2e78e134e3e5d01=function(n){return f(i(n).versions)},e.wbg.__wbg_node_1cd7a5d853dbea79=function(n){return f(i(n).node)},e.wbg.__wbindgen_is_string=function(n){return"string"==typeof i(n)},e.wbg.__wbg_msCrypto_bcb970640f50a1e8=function(n){return f(i(n).msCrypto)},e.wbg.__wbg_require_8f08ceecec0f4fee=function(){return l((function(){return f(n.require)}),arguments)},e.wbg.__wbindgen_is_function=function(n){return"function"==typeof i(n)},e.wbg.__wbindgen_string_new=function(n,e){return f(w(n,e))},e.wbg.__wbg_getRandomValues_37fa2ca9e4e07fab=function(){return l((function(n,e){i(n).getRandomValues(i(e))}),arguments)},e.wbg.__wbg_randomFillSync_dc1e9a60c158336d=function(){return l((function(n,e){i(n).randomFillSync(u(e))}),arguments)},e.wbg.__wbg_newnoargs_581967eacc0e2604=function(n,e){return f(new Function(w(n,e)))},e.wbg.__wbg_call_cb65541d95d71282=function(){return l((function(n,e){return f(i(n).call(i(e)))}),arguments)},e.wbg.__wbindgen_object_clone_ref=function(n){return f(i(n))},e.wbg.__wbg_self_1ff1d729e9aae938=function(){return l((function(){return f(self.self)}),arguments)},e.wbg.__wbg_window_5f4faef6c12b79ec=function(){return l((function(){return f(window.window)}),arguments)},e.wbg.__wbg_globalThis_1d39714405582d3c=function(){return l((function(){return f(globalThis.globalThis)}),arguments)},e.wbg.__wbg_global_651f05c6a0944d1c=function(){return l((function(){return f(t.g.global)}),arguments)},e.wbg.__wbindgen_is_undefined=function(n){return void 0===i(n)},e.wbg.__wbg_call_01734de55d61e11d=function(){return l((function(n,e,t){return f(i(n).call(i(e),i(t)))}),arguments)},e.wbg.__wbg_buffer_085ec1f694018c4f=function(n){return f(i(n).buffer)},e.wbg.__wbg_newwithbyteoffsetandlength_6da8e527659b86aa=function(n,e,t){return f(new Uint8Array(i(n),e>>>0,t>>>0))},e.wbg.__wbg_new_8125e318e6245eed=function(n){return f(new Uint8Array(i(n)))},e.wbg.__wbg_set_5cf90238115182c3=function(n,e,t){i(n).set(i(e),t>>>0)},e.wbg.__wbg_newwithbyteoffsetandlength_69193e31c844b792=function(n,e,t){return f(new Float32Array(i(n),e>>>0,t>>>0))},e.wbg.__wbg_new_d086a66d1c264b3f=function(n){return f(new Float32Array(i(n)))},e.wbg.__wbg_newwithlength_e5d69174d6984cd7=function(n){return f(new Uint8Array(n>>>0))},e.wbg.__wbg_subarray_13db269f57aa838d=function(n,e,t){return f(i(n).subarray(e>>>0,t>>>0))},e.wbg.__wbg_new_abda76e883ba8a5f=function(){return f(new Error)},e.wbg.__wbg_stack_658279fe44541cf6=function(n,e){const t=function(n,e,t){if(void 0===t){const t=y.encode(n),r=e(t.length,1)>>>0;return b().subarray(r,r+t.length).set(t),d=t.length,r}let r=n.length,o=e(r,1)>>>0;const _=b();let i=0;for(;i<r;i++){const e=n.charCodeAt(i);if(e>127)break;_[o+i]=e}if(i!==r){0!==i&&(n=n.slice(i)),o=t(o,r,r=i+3*n.length,1)>>>0;const e=b().subarray(o+i,o+r);i+=m(n,e).written}return d=i,o}(i(e).stack,o.__wbindgen_malloc,o.__wbindgen_realloc),r=d;h()[n/4+1]=r,h()[n/4+0]=t},e.wbg.__wbg_error_f851667af71bcfc6=function(n,e){let t,r;try{t=n,r=e,console.error(w(n,e))}finally{o.__wbindgen_free(t,r,1)}},e.wbg.__wbindgen_throw=function(n,e){throw new Error(w(n,e))},e.wbg.__wbindgen_module=function(){return f(C.__wbindgen_wasm_module)},e.wbg.__wbindgen_memory=function(){return f(o.memory)},e.wbg.__wbg_startWorkers_6fd3af285ea11136=function(n,e,t){return f((0,r.Q)(u(n),u(e),W.__wrap(t)))},e}function x(n,e){n.wbg.memory=e||new WebAssembly.Memory({initial:18,maximum:16384,shared:!0})}function R(n,e){return o=n.exports,C.__wbindgen_wasm_module=e,p=null,a=null,o.__wbindgen_start(),o}function E(n,e){if(void 0!==o)return o;const t=k();return x(t,e),n instanceof WebAssembly.Module||(n=new WebAssembly.Module(n)),R(new WebAssembly.Instance(n,t),n)}async function C(n,e){if(void 0!==o)return o;void 0===n&&(n=new URL(t(255),t.b));const r=k();("string"==typeof n||"function"==typeof Request&&n instanceof Request||"function"==typeof URL&&n instanceof URL)&&(n=fetch(n)),x(r,e);const{instance:_,module:i}=await async function(n,e){if("function"==typeof Response&&n instanceof Response){if("function"==typeof WebAssembly.instantiateStreaming)try{return await WebAssembly.instantiateStreaming(n,e)}catch(e){if("application/wasm"==n.headers.get("Content-Type"))throw e;console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",e)}const t=await n.arrayBuffer();return await WebAssembly.instantiate(t,e)}{const t=await WebAssembly.instantiate(n,e);return t instanceof WebAssembly.Instance?{instance:t,module:n}:t}}(await n,r);return R(_,i)}const S=C},255:(n,e,t)=>{n.exports=t.p+"f91fe0b8f2d4c2943280.wasm"}}]);