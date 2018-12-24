
/**
 * Driver program for emulator
 */

import * as wasm from "../pkg/chip8";
import { memory } from '../pkg/chip8_bg'
const THREE = require('./lib/three.min');

const ROWS = 32;
const COLUMNS = 64;

var camera, scene, renderer, grid, material;
var vBuffer = null;
var key;

export function getKeys() {
	return key;
}

export function init() {
	camera = new THREE.PerspectiveCamera( 120, window.innerWidth / window.innerHeight, 0.001, 1000 );
	camera.position.z = 15;

	scene = new THREE.Scene();
	material = new THREE.MeshPhongMaterial({ color: 0x40DBDB });

	var light = new THREE.DirectionalLight( 0xf000f0 );
	light.position.set( -0.5, 0.5, 1 ).normalize();
	scene.add(light);

	const hCount = ROWS;
    const vCount = COLUMNS;
    const size = 1.01;

	grid = new THREE.Object3D(); // just to hold them all together
	for (let h=0; h<hCount; h+=1) {
		for (let v=0; v<vCount; v+=1) {
			let box = new THREE.Mesh(new THREE.BoxGeometry(size,size,size), material);
			box.position.x = (v-vCount/2); 
			box.position.y = (h-hCount/2);
			grid.add(box);
		}
	}	
	grid.rotateX(Math.PI);
	scene.add(grid);

	renderer = new THREE.WebGLRenderer( { antialias: true } );
	renderer.setSize( window.innerWidth - 10, window.innerHeight - 10);
	renderer.setClearColor( 0x0d0d0d, 1);
	document.body.appendChild( renderer.domElement );
	document.addEventListener('keydown', keyCallBackDown);
	document.addEventListener('keyup', () => { key = 200 });
}

var stats = new Stats();
stats.showPanel( 0 );
document.body.appendChild( stats.dom );

export function animate() {
	requestAnimationFrame( animate );
	stats.begin();
	if(!vBuffer) return;
	let index = 0;
	for (let i = 0; i < COLUMNS; i++) {
		for (let j = 0; j < ROWS; j++) {
			index = i * ROWS + j;	
			if(vBuffer[index] == 0) {
				grid.children[index].visible = false;
			} else {
				grid.children[index].visible = true;
			}
		}
	}
	renderer.render( scene, camera );
	window.main();
	stats.end();
} 

function loadRom() {
	console.log("Loading rom...");
	const selectedFIle = document.getElementById("input").files[0];
	const reader = new FileReader();
	reader.onload = function (evt) {
		console.log("loaded file");
		window.cb = wasm.run(new Uint8Array(evt.target.result));
	}
	reader.readAsArrayBuffer(selectedFIle);
	animate();
}

window.loadRom = loadRom;

var main = () => {
	console.log("main loop not set");
}

window.main = main;

export function setMainLoop(mainLoop) {
	console.log("Setting main loop from Rust");
	window.main = mainLoop;
}

export function setVideoBuffer(buffer) {
	vBuffer = new Uint8Array(memory.buffer, buffer, COLUMNS*ROWS);
}

function keyCallBackDown(e) {
      switch (e.keyCode) {
		case 88: //X
			key = 0x0;
			break;
		case 49: //1
			key = 0x1;
			break;
		case 50: //2
			key = 0x02;
			break;
		case 51: //3
			key = 0x03;
			break;
		case 81: 
			key = 0x4
			break;
		case 87: //W
			key = 0x5;
			break;
		case 69: //E
			key = 0x6;
			break;
		case 65: //A
			key = 0x7;
			break;
		case 83: //S
			key = 0x8;
			break;
		case 68: //D
			key = 0x9;
			break;
		case 90: //Z
			key = 0xa;
			break;
		case 67: //C
			key = 0xb;
			break;
		case 52: //4
			key = 0x0c;
			break;
		case 82: //R
			key = 0xd;
			break;
		case 70: //F
			key = 0xe;
			break;
		case 86: //V
			key = 0xf;
			break;
		default:
          return;
	}
	e.preventDefault();
}
