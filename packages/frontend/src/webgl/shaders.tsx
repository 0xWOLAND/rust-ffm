export const vertexShaderSrc = `#version 300 es

    precision highp float;

    in vec4 a_position;

    void main() {
      gl_Position = a_position;
    }
  `;

export const fragmentShaderSrc = `#version 300 es

    precision highp float;

    uniform vec2 iResolution;

    out vec4 color; 
    
    void main() {
      color = vec4(gl_FragCoord.x / iResolution.x, gl_FragCoord.y / iResolution.y, 0, 1);
    }
  `;
