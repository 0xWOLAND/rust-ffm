import { useContext, useState, useEffect, useRef } from "react";
import { WASMContext } from "../context/wasm";
import { fragmentShaderSrc, vertexShaderSrc } from "@/webgl/shaders";

const initShader = (
  gl: any,
  type: "VERTEX_SHADER" | "FRAGMENT_SHADER",
  source: string
) => {
  const shader = gl.createShader(gl[type]);

  if (!shader) {
    throw new Error("Unable to create a shader.");
  }

  gl.shaderSource(shader, source);

  gl.compileShader(shader);

  if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
    throw new Error(
      `An error occurred compiling the shaders: ${gl.getShaderInfoLog(shader)}`
    );
  }

  return shader;
};

export const RustFFM = () => {
  const ctx = useContext(WASMContext);
  const wasm = ctx.wasm!;
  if (!wasm) return <></>;

  console.log(wasm.simulate(10, 100.0, 10.0, 300, 300));
  const canvasRef = useRef(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!(canvas instanceof HTMLCanvasElement)) {
      throw new Error("No html canvas element.");
    }

    const gl = canvas.getContext("webgl");

    if (!gl) {
      throw new Error("Unable to initialize WebGL.");
    }
    gl.clearColor(255, 255, 255, 1);
    gl.clear(gl.COLOR_BUFFER_BIT);

    const vertexShader = initShader(gl, "VERTEX_SHADER", vertexShaderSrc);
    const fragmentShader = initShader(gl, "FRAGMENT_SHADER", fragmentShaderSrc);

    const program = gl.createProgram();

    if (!program) {
      throw new Error("Unable to create the program.");
    }

    gl.attachShader(program, vertexShader);
    gl.attachShader(program, fragmentShader);

    gl.linkProgram(program);

    if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
      throw new Error(
        `Unable to link the shaders: ${gl.getProgramInfoLog(program)}`
      );
    }

    gl.useProgram(program);

    // UNIFORMS
    const iResolutionLoc = gl.getUniformLocation(program, "iResolution");
    gl.uniform2f(iResolutionLoc, canvas.width, canvas.height);

    // position buffer
    const positionBuffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);
    gl.bufferData(
      gl.ARRAY_BUFFER,
      new Float32Array([-1, -1, 1, -1, -1, 1, -1, 1, 1, -1, 1, 1]),
      gl.DYNAMIC_DRAW
    );

    // index buffer

    // create the buffer
    const indexBuffer = gl.createBuffer();

    // make this buffer the current 'ELEMENT_ARRAY_BUFFER'
    gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, indexBuffer);
    const index = gl.getAttribLocation(program, "a_position");
    const size = 2;
    const type = gl.FLOAT;
    const normalized = false;
    const stride = 0;
    const offset = 0;
    gl.vertexAttribPointer(index, size, type, normalized, stride, offset);
    gl.enableVertexAttribArray(index);
    gl.drawArrays(gl.TRIANGLES, 0, 6);
  }, []);

  return (
    <div>
      <h1 className="text-3xl font-semibold">rust-ffm</h1>
      <canvas width="600" height="600" ref={canvasRef} />
    </div>
  );
};
