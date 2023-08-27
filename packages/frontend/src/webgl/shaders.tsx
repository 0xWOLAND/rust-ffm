export const vertexShaderSrc = `
    attribute vec4 a_position;

    void main() {
      gl_Position = a_position;
    }
  `;

export const fragmentShaderSrc = `
    void main() {
      gl_FragColor = vec4(gl_FragCoord.x / 255., 0, 0, 1);
    }
  `;
