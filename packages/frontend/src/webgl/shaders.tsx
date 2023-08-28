export const vertexShaderSrc = `
    attribute vec4 a_position;

    void main() {
      gl_Position = a_position;
    }
  `;

export const fragmentShaderSrc = `
    precision highp float;

    uniform vec2 iResolution;

    void main() {
      gl_FragColor = vec4(gl_FragCoord.x / iResolution.x, gl_FragCoord.y / iResolution.y, 0, 1);
    }
  `;
