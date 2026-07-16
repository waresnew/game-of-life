import { App, Stats, ScreenPoint as RustScreenPoint } from '$wasm/game_of_life.js';
export const backend = new App(0);
export type Point = [number, number];
export { ScreenPoint as RustScreenPoint } from '$wasm/game_of_life.js';

export const fpsCounters = {
	frameCounter: 0,
	prevFpsTime: 0
};
export function next_step() {
	backend.next_step();
	uiState.generation += 2n ** BigInt(uiState.stepExp);
}
export let canvasDims: DOMRect;
export function setCanvasDims(dims: DOMRect) {
	canvasDims = dims;
}
let stats = $state(Stats.default());
export function getStats() {
	return stats;
}
export function updateStats(newStats: Stats) {
	stats = newStats;
}
export const uiState = $state({
	fps: 0,
	cursor: [0, 0] as Point,
	generation: 0n,
	ticking: false,
	stepExp: 0,
	playRuntime: -1
});

export function toRustScreenPoint(p: Point) {
	return new RustScreenPoint(BigInt(Math.floor(p[0])), BigInt(Math.floor(p[1])));
}
