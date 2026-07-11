import {
	get_config,
	type PerfStats,
	Renderer,
	Point as RustCellPoint,
} from "../pkg/game_of_life.js";

export type Point = [number, number];

export const config = get_config();
export const WORLD_BORDER = Number(1n << BigInt(config.MAX_HEIGHT - 1));
export const CELL_SIZE = 1 << config.CELL_SIZE_EXP;

if (WORLD_BORDER * CELL_SIZE > Number.MAX_SAFE_INTEGER) {
	throw "WORLD_BORDER*CELL_SIZE>max safe int";
}

class World {
	#centre: Point = [0, 0];
	renderedCnt = 0;
	zoomOutExpFloat = 0;
	ticking = false;
	frameCounter = 0;
	prevFpsTime = 0;
	fps = 0;
	stepExp = 0;
	generation = 0n;
	worldCursor: Point = [-1, -1];
	get centre() {
		return this.#centre;
	}
	set centre(p) {
		this.#centre = [Math.floor(p[0]), Math.floor(p[1])];
	}
}

export const world = new World();
export const renderer = new Renderer(world.stepExp);
export const canvas = document.getElementById("grid") as HTMLCanvasElement;
requestAnimationFrame(repaint);
function updateStats() {
	document.getElementById("stats-zoom")!.textContent =
		`Zoom: 2^${-world.zoomOutExpFloat}`;
	document.getElementById("stats-cursor")!.textContent =
		`Cursor: (${Math.floor(world.worldCursor[0] / CELL_SIZE)},${Math.floor(world.worldCursor[1] / CELL_SIZE)})`;
	document.getElementById("stats-fps")!.textContent = `FPS: ${world.fps}`;
	document.getElementById("stats-alive")!.textContent =
		`Alive: ${renderer.perf_stats.alives}`;
	if (renderer) {
		const totalCache =
			renderer.perf_stats.cache_hits + renderer.perf_stats.cache_misses;
		document.getElementById("debug-cache_hitrate")!.textContent =
			`Cache hit rate: ${totalCache > 0 ? (renderer.perf_stats.cache_hits * 100n) / totalCache : "0"}%`;
	} else {
		document.getElementById("debug-cache_hitrate")!.textContent =
			"Cache hit rate: 0%";
	}
	document.getElementById("debug-pool_mem")!.textContent =
		`Pool memory: ${renderer.perf_stats.pool_mem} MB`;
	document.getElementById("stats-generation")!.textContent =
		`Generation: ${world.generation}`;
	document.getElementById("stepsize-display")!.textContent =
		`Step size: 2^${world.stepExp}`;
}
function resizeCanvas() {
	const rect = canvas.getBoundingClientRect();
	canvas.width = rect.width;
	canvas.height = rect.height;
}
const canvasResizeObserver = new ResizeObserver(resizeCanvas);
canvasResizeObserver.observe(canvas);
canvas.addEventListener("contextmenu", (event) => event.preventDefault());
function repaint(time: DOMHighResTimeStamp) {
	++world.frameCounter;
	if (time - world.prevFpsTime >= 1000) {
		world.fps = world.frameCounter;
		world.frameCounter = 0;
		world.prevFpsTime = time;
	}
	if (world.ticking) {
		const start = performance.now();
		next_step();
		const elapsed = (performance.now() - start) / 1000;
		document.getElementById("play-runtime")!.textContent = `Took ${elapsed}s`;
	}

	const ctx = canvas.getContext("2d")!;
	ctx.clearRect(0, 0, canvas.width, canvas.height);
	renderer.update_viewport({
		zoom_out_exp: getEffectiveZoomOutExp(),
		centre: toRustCellPoint(world.centre),
		canvas_dims: toRustCellPoint([canvas.width, canvas.height]),
	});
	const imageData = new ImageData(
		new Uint8ClampedArray(renderer.render()),
		canvas.width,
		canvas.height,
	);
	ctx.putImageData(imageData, 0, 0);
	updateStats();

	document.body.classList.add("ready"); //fouc from empty spans
	requestAnimationFrame(repaint);
}
/** do this outside of canvas to avoid float imprecision */
function translateToScreen(p: Point): Point {
	return [
		p[0] - Math.floor(world.centre[0]) / getEffectiveZoom(),
		p[1] - Math.floor(world.centre[1]) / -getEffectiveZoom(),
	];
}
export function screenToWorld(p: Point): Point {
	return [
		(p[0] + world.centre[0] - canvas.width / 2) / getEffectiveZoom(),
		(p[1] + world.centre[1] - canvas.height / 2) / -getEffectiveZoom(),
	];
}
export function next_step() {
	renderer.next_step();
	world.generation += 2n ** BigInt(world.stepExp);
}
function getEffectiveZoomOutExp() {
	return Math.trunc(world.zoomOutExpFloat);
}
export function getEffectiveZoom() {
	return 2 ** -getEffectiveZoomOutExp();
}
function toRustCellPoint(p: Point) {
	return new RustCellPoint(BigInt(p[0]), BigInt(p[1]));
}
