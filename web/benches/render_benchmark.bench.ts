//TODO: fix this
import { bench, describe } from 'vitest';
import { get_config, Point, Renderer } from '$wasm/game_of_life.js';

function getRandomInt(min: number, max: number) {
	min = Math.ceil(min);
	max = Math.floor(max);
	return Math.floor(Math.random() * (max - min + 1)) + min;
}

const renderer = new Renderer(0);
const GRID_SIZE = 8;
for (let i = -GRID_SIZE; i <= GRID_SIZE; i++) {
	for (let j = -GRID_SIZE; j <= GRID_SIZE; j++) {
		renderer.toggle_cell(BigInt(i), BigInt(j));
	}
}
const min = BigInt(-1 << 50);
const max = BigInt(1 << 50);
const config = get_config();
const CELL_SIZE = 1 << config.CELL_SIZE_EXP;
const canvas = document.createElement('canvas');
canvas.width = 500;
canvas.height = 500;
document.body.appendChild(canvas);
const ctx = canvas.getContext('2d')!;
renderer.update_viewport({
	zoom_out_exp: 0,
	canvas_dims: new Point(500n, 500n),
	centre: new Point(0n, 0n)
});
describe('render benchmark', () => {
	bench(
		'filled 16x16 canvas render',
		async () => {
			//assume 1 zoom, camera at (0,0)
			const alives = renderer.render();
			const imageData = new ImageData(
				new Uint8ClampedArray(renderer.render()),
				canvas.width,
				canvas.height
			);
			ctx.putImageData(imageData, 0, 0);
			ctx.fill();
		},
		{ time: 5000 }
	);
});
