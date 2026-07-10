import { Bench } from "tinybench";
import { bench, describe } from "vitest";
import { get_config, Renderer } from "../../pkg/game_of_life.js";

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
const canvas = document.createElement("canvas");
canvas.width = 500;
canvas.height = 500;
document.body.appendChild(canvas);
const ctx = canvas.getContext("2d")!;
describe("render benchmark", () => {
	bench(
		"filled 16x16 canvas render",
		async () => {
			//assume 1 zoom, camera at (0,0)
			const alives = renderer.render(1, min, min, max, max);
			ctx.beginPath();
			for (let i = 0; i < alives.length; i += config.RENDER_OUTPUT_SIZE) {
				const x = Number(alives[i]) * CELL_SIZE,
					y = Number(alives[i + 1]) * CELL_SIZE,
					size_exp = Number(alives[i + 2]);
				ctx.rect(
					x,
					y,
					CELL_SIZE * (1 << size_exp),
					CELL_SIZE * (1 << size_exp),
				);
			}
			ctx.fill();
		},
		{ time: 5000 },
	);
});
