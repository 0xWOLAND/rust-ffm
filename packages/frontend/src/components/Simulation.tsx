import { useContext, useState, useEffect, useRef } from "react";
import { WASMContext } from "../context/wasm";
import { fragmentShaderSrc, vertexShaderSrc } from "@/webgl/shaders";
import * as THREE from "three";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls.js";
import React from "react";

export const RustFFM = () => {
  const ctx = useContext(WASMContext);
  const wasm = ctx.wasm!;
  if (!wasm) return <></>;

  const canvasRef = useRef(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    const renderer = new THREE.WebGLRenderer({ antialias: true, canvas });
    let clock = new THREE.Clock();

    const aspect = canvas.height / canvas.width; // the canvas default
    const astronomical_unit = wasm.get_scale_length();
    let camera = new THREE.PerspectiveCamera(
      35,
      aspect,
      0.01 * astronomical_unit,
      10000 * astronomical_unit
    );
    camera.position.set(astronomical_unit, 0, astronomical_unit);

    const N = 10000;
    const ffm = new wasm.CosmoSim(
      N,
      10 * astronomical_unit,
      N * 1e24,
      canvas.width,
      canvas.height
    );

    const scene = new THREE.Scene();

    const particleGeometry = new THREE.BufferGeometry();
    let velocities = ffm.get_velocity();
    let positions = ffm.get_position();
    let mass = new Float32Array(1 * N);
    particleGeometry.setAttribute(
      "position",
      new THREE.BufferAttribute(positions, 3)
    );
    particleGeometry.setAttribute(
      "velocity",
      new THREE.BufferAttribute(velocities, 3)
    );
    particleGeometry.setAttribute("mass", new THREE.BufferAttribute(mass, 1));

    const particleShader = new THREE.ShaderMaterial({
      vertexShader: vertexShaderSrc,
      fragmentShader: fragmentShaderSrc,
      uniforms: {},
    });

    const cameraControls = new OrbitControls(camera, renderer.domElement);
    cameraControls.noPan = false;
    var light = new THREE.AmbientLight(0xffffff);
    const particleSystem = new THREE.Points(particleGeometry, particleShader);

    scene.add(camera);
    scene.add(particleSystem);
    scene.add(light);

    function render() {
      renderer.render(scene, camera);
      var seconds = clock.getDelta();
      if (seconds > 1) {
        seconds = 1;
      }
      const timestep = seconds * 60 * 60 * 24 * 15;

      ffm.simulate(timestep);
      positions = ffm.get_position();
      velocities = ffm.get_velocity();
      particleGeometry.attributes.position.array = positions;
      particleGeometry.attributes.velocity.array = velocities;
      particleGeometry.attributes.position.needsUpdate = true;

      requestAnimationFrame(render);
    }
    requestAnimationFrame(render);
  }, []);

  return (
    <div>
      <h1 className="text-3xl font-semibold">rust-ffm</h1>
      <canvas width="600" height="600" ref={canvasRef} />
    </div>
  );
};
