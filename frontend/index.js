import * as Comlink from "comlink";
import * as THREE from "three";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls.js";
import { N, AU } from "./cosmology";

const canvas = document.getElementById("canvas");
const { width, height } = canvas;
const timeOutput = document.getElementById("time");

(async function init() {
  let handlers = await Comlink.wrap(
    new Worker(new URL("./wasm-worker.js", import.meta.url), {
      type: "module",
    })
  ).handlers;

  function setupBtn(id) {
    let handler = handlers[id];
    if (!handler) return;
    Object.assign(document.getElementById(id), {
      async onclick() {
        // Scene
        const scene = new THREE.Scene();
        let clock = new THREE.Clock();

        // Camera
        let camera = new THREE.PerspectiveCamera(
          35,
          height / width,
          0.01 * AU,
          10000 * AU
        );
        camera.position.set(AU, 0, AU);
        camera.position.z = 5;

        // Renderer
        const renderer = new THREE.WebGLRenderer({
          antialias: true,
          canvas,
        });
        renderer.setSize(width, height);

        // Controls
        const cameraControls = new OrbitControls(camera, renderer.domElement);
        cameraControls.noPan = false;

        // Particle Geometry
        const particleGeometry = new THREE.BufferGeometry();
        let velocities = new Float32Array(3 * N);
        let positions = new Float32Array(3 * N);
        particleGeometry.setAttribute(
          "position",
          new THREE.BufferAttribute(positions, 3)
        );
        particleGeometry.setAttribute(
          "velocity",
          new THREE.BufferAttribute(velocities, 3)
        );

        const particleShader = new THREE.ShaderMaterial({
          vertexShader: vertexShaderSrc,
          fragmentShader: fragmentShaderSrc,
          uniforms: {},
        });

        const particleSystem = new THREE.Points(
          particleGeometry,
          particleShader
        );

        scene.add(camera);
        scene.add(particleSystem);

        async function animate() {
          var seconds = clock.getDelta();
          if (seconds > 1) {
            seconds = 1;
          }
          const timestep = seconds * 60 * 60 * 24 * 15;
          let { position, velocity, time } = await handler({
            timestep,
          });

          timeOutput.innerText = time / 1000 + " ms";

          particleGeometry.attributes.position.array = position;
          particleGeometry.attributes.velocity.array = velocity;
          particleGeometry.attributes.position.needsUpdate = true;

          renderer.render(scene, camera);
          requestAnimationFrame(animate);
        }

        await animate();
      },
      disabled: false,
    });
  }

  setupBtn("singleThread");
  if (await handlers.supportsThreads) {
    setupBtn("multiThread");
  }
})();

const vertexShaderSrc = `
  varying vec3 vPosition;
  varying vec3 vVelocity;
  attribute vec3 velocity;

  void main() {
    vPosition = position;
    vVelocity = velocity;

    gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.);
    gl_PointSize = 1.;
  }
`;

const fragmentShaderSrc = `
const float PI = 3.1415926;

varying vec3 vPosition;
varying vec3 vVelocity;

void main() {
    gl_FragColor = vec4(1.);
}

  `;
