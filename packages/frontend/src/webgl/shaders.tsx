export const vertexShaderSrc = `
  varying vec3 vPosition;
		attribute float mass;
	    const float earth_mass = 1e24; //kg


		void main() {
			vPosition = position;
			gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.);
			// gl_PointSize = 8. * mass / earth_mass;
      gl_PointSize = 5.;
		}
  `;

export const fragmentShaderSrc = `
        const float PI = 3.1415926;

        varying vec3 vPosition;

        //USER_INPUT_GOES_HERE

        void main() {
            gl_FragColor = vec4(1,1,1,1);
        }

  `;
