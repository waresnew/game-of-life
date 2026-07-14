import {
	type RenderStats,
	ScreenPoint as RustScreenPoint,
	type PerfStats
} from '$wasm/game_of_life.js';
import { renderer } from './wasm.js';
export type Point = [number, number];

export const fpsCounters = {
	frameCounter: 0,
	prevFpsTime: 0
};
export function next_step() {
	renderer.next_step();
	uiState.generation += 2n ** BigInt(uiState.stepExp);
}
export let canvasDims: DOMRect;
export function setCanvasDims(dims: DOMRect) {
	canvasDims = dims;
}
let perfStats = $state(renderer.perf_stats);
export function getPerfStats() {
	return perfStats;
}
export function updatePerfStats(stats: PerfStats) {
	perfStats = stats;
}
let renderStats = $state(renderer.render_stats);
export function getRenderStats() {
	return renderStats;
}
export function updateRenderStats(stats: RenderStats) {
	renderStats = stats;
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
