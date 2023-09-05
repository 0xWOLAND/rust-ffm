import { threads } from 'wasm-feature-detect';
import * as Comlink from 'comlink';


// Wrap wasm-bindgen exports (the `generate` function) to add time measurement.
function wrapExports(wasm) {
  const N = 10000;
  const astronomical_unit = 1e11;
  const fmm = new wasm.CosmoSim(N, 10 * astronomical_unit, 1e24);
  return ({ width, height, maxIterations }) => {
    const start = performance.now();

    // const rawImageData = generate(width, height, maxIterations);
    const time = performance.now() - start;
    fmm.simulate(time);

    const position = fmm.get_position();
    const velocity = fmm.get_velocity();

    console.log(position);

    return {
      // Little perf boost to transfer data to the main thread w/o copying.
    //   rawImageData: Comlink.transfer(rawImageData, [rawImageData.buffer]),
      position,
      velocity,
      time
    };
  };
}

async function initHandlers() {
  let [singleThread, multiThread] = await Promise.all([
    (async () => {
      const singleThread = await import('wasm-single');
      await singleThread.default();
      return wrapExports(singleThread);
    })(),
    (async () => {
      // If threads are unsupported in this browser, skip this handler.
      if (!(await threads())) return;
      const multiThread = await import(
      'wasm-parallel'
      );
      await multiThread.default();
      await multiThread.initThreadPool(navigator.hardwareConcurrency);
      return wrapExports(multiThread);
    })()
  ]);

  return Comlink.proxy({
    singleThread,
    supportsThreads: !!multiThread,
    multiThread
  });
}

Comlink.expose({
  handlers: initHandlers()
});