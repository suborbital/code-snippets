const { babel } = require('@rollup/plugin-babel');
const nodeResolve = require('@rollup/plugin-node-resolve');
const commonjs = require('@rollup/plugin-commonjs');
const replace = require('@rollup/plugin-replace');

const globals =  require('rollup-plugin-node-globals');
const builtins =  require('rollup-plugin-node-builtins');
const { terser } = require('rollup-plugin-terser');
const styles = require('rollup-plugin-styles');
const image = require('@rollup/plugin-image');


const babelOptions = {
  "presets": [
    '@babel/preset-react'
  ]
}

module.exports = [
  {
    input: 'src/index.js',
    output: {
      file: 'build/index.js',
      format: 'umd',
	  name: "Suborbital"
    }, 
    plugins: [
		image(),
		styles(),
		terser(),
		nodeResolve(),
		commonjs(),
		babel(babelOptions),
		globals(),
		builtins(),
		replace({
			'process.env.NODE_ENV': JSON.stringify( 'production' )
		}),
    ],
  },
]