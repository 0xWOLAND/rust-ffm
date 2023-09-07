import * as Comlink from "comlink";
import * as THREE from "three";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls.js";
import { N, AU, N_plummer, N_spheres } from "./cosmology";

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
        camera.position.set(0, 0, 1.7 * AU);

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
        let positions = new Float32Array(3 * N);
        let masses = new Float32Array(N).fill(1, 0, N_spheres * N_plummer);
        console.log(`using ${N} particles`);
        particleGeometry.setAttribute(
          "position",
          new THREE.BufferAttribute(positions, 3)
        );
        particleGeometry.setAttribute(
          "mass",
          new THREE.BufferAttribute(masses, 1)
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
          let { position, time } = await handler({
            timestep,
          });

          timeOutput.innerText = time / 1000 + " ms";

          const buf = new THREE.BufferAttribute(new Float32Array(position), 3);
          particleGeometry.setAttribute("position", buf);
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
  varying float vMass;

  attribute float mass;

  void main() {
    vPosition = position;
    vMass = mass;

    gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.);
    gl_PointSize = 1.;
  }
`;

const fragmentShaderSrc = `
const float PI = 3.1415926;

varying vec3 vPosition;
varying float vMass;

void main() {
    vec3 color = vMass * vec3(236., 46., 0.) + (1. - vMass) * vec3(1., 94., 158.);
    gl_FragColor = vec4(color / 255., 1.0);
}

  `;
