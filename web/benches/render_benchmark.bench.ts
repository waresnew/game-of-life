import { bench, describe } from 'vitest';
import { ScreenPoint as RustScreenPoint, App } from '$wasm/game_of_life.js';

function getRandomInt(min: number, max: number) {
	min = Math.ceil(min);
	max = Math.floor(max);
	return Math.floor(Math.random() * (max - min + 1)) + min;
}

const backend = new App(0);
const GRID_SIZE = 16;
const CANVAS_DIMS = 1500;
backend.update_canvas_dims(new RustScreenPoint(BigInt(CANVAS_DIMS), BigInt(CANVAS_DIMS)));
for (let i = 0; i <= GRID_SIZE; i++) {
	for (let j = 0; j <= GRID_SIZE; j++) {
		backend.handle_draw(new RustScreenPoint(BigInt(i * 64), BigInt(j * 64)));
	}
}
const canvas = document.createElement('canvas');
canvas.width = CANVAS_DIMS;
canvas.height = CANVAS_DIMS;
document.body.appendChild(canvas);
const ctx = canvas.getContext('2d')!;
backend.handle_zoom(-3, new RustScreenPoint(0n, 0n));
describe('render benchmark', () => {
	bench(
		'filled 16x16 canvas render',
		async () => {
			backend.render();
			const imageData = new ImageData(
				new Uint8ClampedArray(backend.render()),
				canvas.width,
				canvas.height
			);
			ctx.putImageData(imageData, 0, 0);
			ctx.fill();
		},
		{ time: 5000 }
	);
});
