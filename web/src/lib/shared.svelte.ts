import { RenderStats, ScreenPoint as RustScreenPoint, type PerfStats } from '$wasm/game_of_life.js';
import { config, renderer } from './wasm.js';
export type Point = [number, number];

export const WORLD_BORDER = Number(1n << BigInt(config.MAX_HEIGHT - 1));
export const CELL_SIZE = 1 << config.CELL_SIZE_EXP;

if (WORLD_BORDER * CELL_SIZE > Number.MAX_SAFE_INTEGER) {
	throw 'WORLD_BORDER*CELL_SIZE>max safe int';
}
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

const rule = $state({
	b: [3],
	s: [2, 3]
});
export function getRuleB() {
	return rule.b;
}
export function getRuleS() {
	return rule.s;
}
export function updateRule(b: number[], s: number[]) {
	if (b.join('') != rule.b.join('') || s.join('') != rule.s.join('')) {
		rule.b = b;
		rule.s = s;
	}
}

export function toRustScreenPoint(p: Point) {
	return new RustScreenPoint(BigInt(Math.floor(p[0])), BigInt(Math.floor(p[1])));
}
