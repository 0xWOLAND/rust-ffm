import { threads } from "wasm-feature-detect";
import { N, AU, N_plummer } from "./cosmology";
import * as Comlink from "comlink";

function wrapExports(wasm) {
  const fmm = new wasm.CosmoSim(N_plummer, 10 * AU, N_plummer * 1e24);
  return ({ timestep }) => {
    const start = performance.now();

    const time = performance.now() - start;
    fmm.simulate(timestep);

    const position = fmm.get_position();
    const velocity = fmm.get_velocity();

    return {
      // Little perf boost to transfer data to the main thread w/o copying.
      //   rawImageData: Comlink.transfer(rawImageData, [rawImageData.buffer]),
      position,
      velocity,
      time,
    };
  };
}

async function initHandlers() {
  let [singleThread, multiThread] = await Promise.all([
    (async () => {
      const singleThread = await import("../pkg");
      await singleThread.default();
      return wrapExports(singleThread);
    })(),
    (async () => {
      if (!(await threads())) return;
      const multiThread = await import("../pkg-parallel");
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
