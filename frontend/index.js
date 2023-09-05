import * as Comlink from 'comlink';

const maxIterations = 1000;

const canvas = document.getElementById('canvas');
const { width, height } = canvas;
const ctx = canvas.getContext('2d');
const timeOutput = document.getElementById('time');

(async function init() {
  // Create a separate thread from wasm-worker.js and get a proxy to its handlers.
  let handlers = await Comlink.wrap(
    new Worker(new URL('./wasm-worker.js', import.meta.url), {
      type: 'module'
    })
  ).handlers;

  function setupBtn(id) {
    // Handlers are named in the same way as buttons.
    let handler = handlers[id];
    // If handler doesn't exist, it's not supported.
    if (!handler) return;
    // Assign onclick handler + enable the button.
    Object.assign(document.getElementById(id), {
      async onclick() {
        let { positions, velocity, time} = await handler({
          width,
          height,
          maxIterations
        });
        timeOutput.value = `${time.toFixed(2)} ms`;
        console.log(positions, velocity)
        // const imgData = new ImageData(rawImageData, width, height);
        // ctx.putImageData(imgData, 0, 0);
      },
      disabled: false
    });
  }

  setupBtn('singleThread');
  if (await handlers.supportsThreads) {
    setupBtn('multiThread');
  }
})();