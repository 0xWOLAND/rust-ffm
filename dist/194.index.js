"use strict";(self.webpackChunk=self.webpackChunk||[]).push([[194],{194:(n,e,t)=>{let r;t.r(e),t.d(e,{CosmoSim:()=>p,default:()=>W,get_scale_length:()=>w,initSync:()=>T}),n=t.hmd(n);const o=new Array(128).fill(void 0);function i(n){return o[n]}o.push(void 0,null,!0,!1);let _=o.length;function c(n){const e=i(n);return function(n){n<132||(o[n]=_,_=n)}(n),e}const u="undefined"!=typeof TextDecoder?new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw Error("TextDecoder not available")}};"undefined"!=typeof TextDecoder&&u.decode();let s=null;function f(){return null!==s&&0!==s.byteLength||(s=new Uint8Array(r.memory.buffer)),s}function a(n,e){return n>>>=0,u.decode(f().subarray(n,n+e))}function b(n){_===o.length&&o.push(o.length+1);const e=_;return _=o[e],o[e]=n,e}function w(){return r.get_scale_length()}function g(n,e){try{return n.apply(this,e)}catch(n){r.__wbindgen_exn_store(b(n))}}let l=0;const d="undefined"!=typeof TextEncoder?new TextEncoder("utf-8"):{encode:()=>{throw Error("TextEncoder not available")}},y="function"==typeof d.encodeInto?function(n,e){return d.encodeInto(n,e)}:function(n,e){const t=d.encode(n);return e.set(t),{read:n.length,written:t.length}};let m=null;function h(){return null!==m&&0!==m.byteLength||(m=new Int32Array(r.memory.buffer)),m}class p{static __wrap(n){n>>>=0;const e=Object.create(p.prototype);return e.__wbg_ptr=n,e}__destroy_into_raw(){const n=this.__wbg_ptr;return this.__wbg_ptr=0,n}free(){const n=this.__destroy_into_raw();r.__wbg_cosmosim_free(n)}constructor(n,e,t){const o=r.cosmosim_new(n,e,t);return p.__wrap(o)}simulate(n){r.cosmosim_simulate(this.__wbg_ptr,n)}get_position(){return c(r.cosmosim_get_position(this.__wbg_ptr))}get_velocity(){return c(r.cosmosim_get_velocity(this.__wbg_ptr))}}function A(){const e={wbg:{}};return e.wbg.__wbindgen_object_drop_ref=function(n){c(n)},e.wbg.__wbg_crypto_c48a774b022d20ac=function(n){return b(i(n).crypto)},e.wbg.__wbindgen_is_object=function(n){const e=i(n);return"object"==typeof e&&null!==e},e.wbg.__wbg_process_298734cf255a885d=function(n){return b(i(n).process)},e.wbg.__wbg_versions_e2e78e134e3e5d01=function(n){return b(i(n).versions)},e.wbg.__wbg_node_1cd7a5d853dbea79=function(n){return b(i(n).node)},e.wbg.__wbindgen_is_string=function(n){return"string"==typeof i(n)},e.wbg.__wbg_msCrypto_bcb970640f50a1e8=function(n){return b(i(n).msCrypto)},e.wbg.__wbg_require_8f08ceecec0f4fee=function(){return g((function(){return b(n.require)}),arguments)},e.wbg.__wbindgen_is_function=function(n){return"function"==typeof i(n)},e.wbg.__wbindgen_string_new=function(n,e){return b(a(n,e))},e.wbg.__wbg_getRandomValues_37fa2ca9e4e07fab=function(){return g((function(n,e){i(n).getRandomValues(i(e))}),arguments)},e.wbg.__wbg_randomFillSync_dc1e9a60c158336d=function(){return g((function(n,e){i(n).randomFillSync(c(e))}),arguments)},e.wbg.__wbg_newnoargs_581967eacc0e2604=function(n,e){return b(new Function(a(n,e)))},e.wbg.__wbg_call_cb65541d95d71282=function(){return g((function(n,e){return b(i(n).call(i(e)))}),arguments)},e.wbg.__wbindgen_object_clone_ref=function(n){return b(i(n))},e.wbg.__wbg_self_1ff1d729e9aae938=function(){return g((function(){return b(self.self)}),arguments)},e.wbg.__wbg_window_5f4faef6c12b79ec=function(){return g((function(){return b(window.window)}),arguments)},e.wbg.__wbg_globalThis_1d39714405582d3c=function(){return g((function(){return b(globalThis.globalThis)}),arguments)},e.wbg.__wbg_global_651f05c6a0944d1c=function(){return g((function(){return b(t.g.global)}),arguments)},e.wbg.__wbindgen_is_undefined=function(n){return void 0===i(n)},e.wbg.__wbg_call_01734de55d61e11d=function(){return g((function(n,e,t){return b(i(n).call(i(e),i(t)))}),arguments)},e.wbg.__wbg_buffer_085ec1f694018c4f=function(n){return b(i(n).buffer)},e.wbg.__wbg_newwithbyteoffsetandlength_6da8e527659b86aa=function(n,e,t){return b(new Uint8Array(i(n),e>>>0,t>>>0))},e.wbg.__wbg_new_8125e318e6245eed=function(n){return b(new Uint8Array(i(n)))},e.wbg.__wbg_set_5cf90238115182c3=function(n,e,t){i(n).set(i(e),t>>>0)},e.wbg.__wbg_newwithbyteoffsetandlength_69193e31c844b792=function(n,e,t){return b(new Float32Array(i(n),e>>>0,t>>>0))},e.wbg.__wbg_new_d086a66d1c264b3f=function(n){return b(new Float32Array(i(n)))},e.wbg.__wbg_newwithlength_e5d69174d6984cd7=function(n){return b(new Uint8Array(n>>>0))},e.wbg.__wbg_subarray_13db269f57aa838d=function(n,e,t){return b(i(n).subarray(e>>>0,t>>>0))},e.wbg.__wbg_new_abda76e883ba8a5f=function(){return b(new Error)},e.wbg.__wbg_stack_658279fe44541cf6=function(n,e){const t=function(n,e,t){if(void 0===t){const t=d.encode(n),r=e(t.length,1)>>>0;return f().subarray(r,r+t.length).set(t),l=t.length,r}let r=n.length,o=e(r,1)>>>0;const i=f();let _=0;for(;_<r;_++){const e=n.charCodeAt(_);if(e>127)break;i[o+_]=e}if(_!==r){0!==_&&(n=n.slice(_)),o=t(o,r,r=_+3*n.length,1)>>>0;const e=f().subarray(o+_,o+r);_+=y(n,e).written}return l=_,o}(i(e).stack,r.__wbindgen_malloc,r.__wbindgen_realloc),o=l;h()[n/4+1]=o,h()[n/4+0]=t},e.wbg.__wbg_error_f851667af71bcfc6=function(n,e){let t,o;try{t=n,o=e,console.error(a(n,e))}finally{r.__wbindgen_free(t,o,1)}},e.wbg.__wbindgen_throw=function(n,e){throw new Error(a(n,e))},e.wbg.__wbindgen_memory=function(){return b(r.memory)},e}function v(n,e){return r=n.exports,x.__wbindgen_wasm_module=e,m=null,s=null,r}function T(n){if(void 0!==r)return r;const e=A();return n instanceof WebAssembly.Module||(n=new WebAssembly.Module(n)),v(new WebAssembly.Instance(n,e),n)}async function x(n){if(void 0!==r)return r;void 0===n&&(n=new URL(t(463),t.b));const e=A();("string"==typeof n||"function"==typeof Request&&n instanceof Request||"function"==typeof URL&&n instanceof URL)&&(n=fetch(n));const{instance:o,module:i}=await async function(n,e){if("function"==typeof Response&&n instanceof Response){if("function"==typeof WebAssembly.instantiateStreaming)try{return await WebAssembly.instantiateStreaming(n,e)}catch(e){if("application/wasm"==n.headers.get("Content-Type"))throw e;console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",e)}const t=await n.arrayBuffer();return await WebAssembly.instantiate(t,e)}{const t=await WebAssembly.instantiate(n,e);return t instanceof WebAssembly.Instance?{instance:t,module:n}:t}}(await n,e);return v(o,i)}const W=x},463:(n,e,t)=>{n.exports=t.p+"9d695836724944b10453.wasm"}}]);