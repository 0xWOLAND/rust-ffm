import { threads } from "wasm-feature-detect";
import { N, AU, N_plummer } from "./cosmology";
import * as Comlink from "comlink";

function wrapExports(wasm) {
  const fmm = new wasm.CosmoSim(N_plummer, 10 * AU, N_plummer * 1e24);
  return ({ timestep }) => {
    const start = performance.now();

    fmm.simulate(timestep);

    const position = fmm.get_position();
    const time = performance.now() - start;

    return {
      position,
      time,
    };
  };
}

async function initHandlers() {
  let [singleThread, multiThread] = await Promise.all([
    (async () => {
      const singleThread = await import("./pkg/rust_ffm.js");
      console.log("single thread available");
      await singleThread.default();
      return wrapExports(singleThread);
    })(),
    (async () => {
      if (!(await threads())) {
        console.error("threads unavailable");
        return;
      }
      console.log("threads available");
      const multiThread = await import("./pkg-parallel/rust_ffm.js");
      await multiThread.default();
      await multiThread.initThreadPool(navigator.hardwareConcurrency);
      return wrapExports(multiThread);
    })(),
  ]);

  return Comlink.proxy({
    singleThread,
    supportsThreads: !!multiThread,
    multiThread,
  });
}

Comlink.expose({
  handlers: initHandlers(),
});
