import { Bench } from "tinybench";
import { bench, describe } from "vitest";
import { Renderer, WorldPoint as RustPoint } from "../../pkg/game_of_life.js";

function getRandomInt(min: number, max: number) {
	min = Math.ceil(min);
	max = Math.floor(max);
	return Math.floor(Math.random() * (max - min + 1)) + min;
}

const renderer = new Renderer(0, 50);
const GRID_SIZE = 8;
for (let i = -GRID_SIZE; i <= GRID_SIZE; i++) {
	for (let j = -GRID_SIZE; j <= GRID_SIZE; j++) {
		renderer.toggle_cell(new RustPoint(BigInt(i), BigInt(j)));
	}
}
describe("wasm benchmark", () => {
	bench(
		"filled 16x16 render",
		async () => {
			const MIN_POINT = new RustPoint(BigInt(-1e14), BigInt(-1e14));
			const MAX_POINT = new RustPoint(BigInt(1e14), BigInt(1e14));
			(globalThis as any).res = renderer.render(1, MIN_POINT, MAX_POINT);
		},
		{ time: 5000 },
	);
});
