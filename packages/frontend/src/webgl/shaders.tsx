export const vertexShaderSrc = `
  varying vec3 vPosition;
  varying vec3 vVelocity;

  attribute vec3 velocity;

		attribute float mass;
	    const float earth_mass = 1e24; //kg


		void main() {
			vPosition = position;
      vVelocity = velocity;

			gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.);
      gl_PointSize = 1.;
		}
  `;

export const fragmentShaderSrc = `
        const float PI = 3.1415926;

        varying vec3 vPosition;
        varying vec3 vVelocity;



        void main() {
            gl_FragColor = vec4(1.);
        }

  `;
