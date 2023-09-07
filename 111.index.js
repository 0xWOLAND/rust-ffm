(()=>{"use strict";var e={},t={};function n(r){var a=t[r];if(void 0!==a)return a.exports;var o=t[r]={id:r,loaded:!1,exports:{}};return e[r](o,o.exports,n),o.loaded=!0,o.exports}n.m=e,n.d=(e,t)=>{for(var r in t)n.o(t,r)&&!n.o(e,r)&&Object.defineProperty(e,r,{enumerable:!0,get:t[r]})},n.f={},n.e=e=>Promise.all(Object.keys(n.f).reduce(((t,r)=>(n.f[r](e,t),t)),[])),n.u=e=>e+".index.js",n.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),n.hmd=e=>((e=Object.create(e)).children||(e.children=[]),Object.defineProperty(e,"exports",{enumerable:!0,set:()=>{throw new Error("ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: "+e.id)}}),e),n.o=(e,t)=>Object.prototype.hasOwnProperty.call(e,t),n.r=e=>{"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},(()=>{var e;n.g.importScripts&&(e=n.g.location+"");var t=n.g.document;if(!e&&t&&(t.currentScript&&(e=t.currentScript.src),!e)){var r=t.getElementsByTagName("script");if(r.length)for(var a=r.length-1;a>-1&&!e;)e=r[a--].src}if(!e)throw new Error("Automatic publicPath is not supported in this browser");e=e.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),n.p=e})(),(()=>{n.b=self.location+"";var e={111:1};n.f.i=(t,r)=>{e[t]||importScripts(n.p+n.u(t))};var t=self.webpackChunk=self.webpackChunk||[],r=t.push.bind(t);t.push=t=>{var[a,o,i]=t;for(var s in o)n.o(o,s)&&(n.m[s]=o[s]);for(i&&i(n);a.length;)e[a.pop()]=1;r(t)}})();const r=Symbol("Comlink.proxy"),a=Symbol("Comlink.endpoint"),o=Symbol("Comlink.releaseProxy"),i=Symbol("Comlink.finalizer"),s=Symbol("Comlink.thrown"),c=e=>"object"==typeof e&&null!==e||"function"==typeof e,l=new Map([["proxy",{canHandle:e=>c(e)&&e[r],serialize(e){const{port1:t,port2:n}=new MessageChannel;return u(e,t),[n,[n]]},deserialize:e=>(e.start(),m(e,[],undefined))}],["throw",{canHandle:e=>c(e)&&s in e,serialize({value:e}){let t;return t=e instanceof Error?{isError:!0,value:{message:e.message,name:e.name,stack:e.stack}}:{isError:!1,value:e},[t,[]]},deserialize(e){if(e.isError)throw Object.assign(new Error(e.value.message),e.value);throw e.value}}]]);function u(e,t=globalThis,n=["*"]){t.addEventListener("message",(function r(a){if(!a||!a.data)return;if(!function(e,t){for(const n of e){if(t===n||"*"===n)return!0;if(n instanceof RegExp&&n.test(t))return!0}return!1}(n,a.origin))return void console.warn(`Invalid origin '${a.origin}' for comlink proxy`);const{id:o,type:c,path:l}=Object.assign({path:[]},a.data),d=(a.data.argumentList||[]).map(E);let f;try{const t=l.slice(0,-1).reduce(((e,t)=>e[t]),e),n=l.reduce(((e,t)=>e[t]),e);switch(c){case"GET":f=n;break;case"SET":t[l.slice(-1)[0]]=E(a.data.value),f=!0;break;case"APPLY":f=n.apply(t,d);break;case"CONSTRUCT":f=v(new n(...d));break;case"ENDPOINT":{const{port1:t,port2:n}=new MessageChannel;u(e,n),f=function(e,t){return b.set(e,t),e}(t,[t])}break;case"RELEASE":f=void 0;break;default:return}}catch(e){f={value:e,[s]:0}}Promise.resolve(f).catch((e=>({value:e,[s]:0}))).then((n=>{const[a,s]=w(n);t.postMessage(Object.assign(Object.assign({},a),{id:o}),s),"RELEASE"===c&&(t.removeEventListener("message",r),p(t),i in e&&"function"==typeof e[i]&&e[i]())})).catch((e=>{const[n,r]=w({value:new TypeError("Unserializable return value"),[s]:0});t.postMessage(Object.assign(Object.assign({},n),{id:o}),r)}))})),t.start&&t.start()}function p(e){(function(e){return"MessagePort"===e.constructor.name})(e)&&e.close()}function d(e){if(e)throw new Error("Proxy has been released and is not useable")}function f(e){return S(e,{type:"RELEASE"}).then((()=>{p(e)}))}const g=new WeakMap,h="FinalizationRegistry"in globalThis&&new FinalizationRegistry((e=>{const t=(g.get(e)||0)-1;g.set(e,t),0===t&&f(e)}));function m(e,t=[],n=function(){}){let r=!1;const i=new Proxy(n,{get(n,a){if(d(r),a===o)return()=>{!function(e){h&&h.unregister(e)}(i),f(e),r=!0};if("then"===a){if(0===t.length)return{then:()=>i};const n=S(e,{type:"GET",path:t.map((e=>e.toString()))}).then(E);return n.then.bind(n)}return m(e,[...t,a])},set(n,a,o){d(r);const[i,s]=w(o);return S(e,{type:"SET",path:[...t,a].map((e=>e.toString())),value:i},s).then(E)},apply(n,o,i){d(r);const s=t[t.length-1];if(s===a)return S(e,{type:"ENDPOINT"}).then(E);if("bind"===s)return m(e,t.slice(0,-1));const[c,l]=y(i);return S(e,{type:"APPLY",path:t.map((e=>e.toString())),argumentList:c},l).then(E)},construct(n,a){d(r);const[o,i]=y(a);return S(e,{type:"CONSTRUCT",path:t.map((e=>e.toString())),argumentList:o},i).then(E)}});return function(e,t){const n=(g.get(t)||0)+1;g.set(t,n),h&&h.register(e,t,e)}(i,e),i}function y(e){const t=e.map(w);return[t.map((e=>e[0])),(n=t.map((e=>e[1])),Array.prototype.concat.apply([],n))];var n}const b=new WeakMap;function v(e){return Object.assign(e,{[r]:!0})}function w(e){for(const[t,n]of l)if(n.canHandle(e)){const[r,a]=n.serialize(e);return[{type:"HANDLER",name:t,value:r},a]}return[{type:"RAW",value:e},b.get(e)||[]]}function E(e){switch(e.type){case"HANDLER":return l.get(e.name).deserialize(e.value);case"RAW":return e.value}}function S(e,t,n){return new Promise((r=>{const a=new Array(4).fill(0).map((()=>Math.floor(Math.random()*Number.MAX_SAFE_INTEGER).toString(16))).join("-");e.addEventListener("message",(function t(n){n.data&&n.data.id&&n.data.id===a&&(e.removeEventListener("message",t),r(n.data))})),e.start&&e.start(),e.postMessage(Object.assign({id:a},t),n)}))}function T(e){const t=new e.CosmoSim(10001,1e12,10001e24);return({timestep:e})=>{const n=performance.now();return t.simulate(e),{position:t.get_position(),time:performance.now()-n}}}u({handlers:async function(){let[e,t]=await Promise.all([(async()=>{const e=await n.e(194).then(n.bind(n,194));return console.log("single thread available"),await e.default(),T(e)})(),(async()=>{if(!await(async e=>{try{return"undefined"!=typeof MessageChannel&&(new MessageChannel).port1.postMessage(new SharedArrayBuffer(1)),WebAssembly.validate(e)}catch(e){return!1}})(new Uint8Array([0,97,115,109,1,0,0,0,1,4,1,96,0,0,3,2,1,0,5,4,1,3,1,1,10,11,1,9,0,65,0,254,16,2,0,26,11])))return void console.error("threads unavailable");console.log("threads available");const e=await n.e(361).then(n.bind(n,649));return await e.default(),await e.initThreadPool(navigator.hardwareConcurrency),T(e)})()]);return v({singleThread:e,supportsThreads:!!t,multiThread:t})}()})})();