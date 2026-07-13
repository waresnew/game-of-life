//TODO: fix this
import { bench, describe } from 'vitest';
import { Point, Renderer } from '$wasm/game_of_life.js';

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
renderer.update_viewport({
	zoom_out_exp: 0,
	canvas_dims: new Point(500n, 500n),
	centre: new Point(0n, 0n)
});
describe('wasm benchmark', () => {
	bench(
		'filled 16x16 render',
		async () => {
			(globalThis as any).res = renderer.render();
		},
		{ time: 5000 }
	);
});
