import { get_config, type PerfStats, Renderer } from "../pkg/game_of_life.js";

export type Point = [number, number];

export const config = get_config();
export const WORLD_BORDER = Number(1n << BigInt(config.MAX_HEIGHT - 1));
export const CELL_SIZE = 1 << config.CELL_SIZE_EXP;

if (WORLD_BORDER * CELL_SIZE > Number.MAX_SAFE_INTEGER) {
	throw "WORLD_BORDER*CELL_SIZE>max safe int";
}

class World {
	centre: Point = [0, 0];
	renderedCnt = 0;
	zoomExpFloat = 0;
	ticking = false;
	frameCounter = 0;
	prevFpsTime = 0;
	fps = 0;
	stepExp = 0;
	generation = 0n;
	worldCursor: Point = [-1, -1];
}

export const world = new World();
export const renderer = new Renderer(world.stepExp);
export const canvas = document.getElementById("grid") as HTMLCanvasElement;
requestAnimationFrame(repaint);
function updateStats() {
	document.getElementById("stats-zoom")!.textContent =
		`Zoom: 2^${world.zoomExpFloat}`;
	document.getElementById("stats-cursor")!.textContent =
		`Cursor: (${Math.floor(world.worldCursor[0] / CELL_SIZE)},${Math.floor(world.worldCursor[1] / CELL_SIZE)})`;
	document.getElementById("debug-rendered")!.textContent =
		`Rendered: ${world.renderedCnt}`;
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
	ctx.resetTransform();
	ctx.clearRect(0, 0, canvas.width, canvas.height);
	ctx.fillStyle = "#808080";
	ctx.fillRect(0, 0, canvas.width, canvas.height);
	ctx.setTransform(
		getEffectiveZoom(),
		0,
		0,
		-getEffectiveZoom(),
		canvas.width / 2,
		canvas.height / 2,
	);
	ctx.fillStyle = "#ffffff";
	const border = translateToScreen([
		-WORLD_BORDER * CELL_SIZE,
		-WORLD_BORDER * CELL_SIZE,
	]);
	ctx.fillRect(
		border[0],
		border[1],
		(WORLD_BORDER * 2 + 1) * CELL_SIZE,
		(WORLD_BORDER * 2 + 1) * CELL_SIZE,
	);
	const tl = screenToWorld([0, 0]).map((x) =>
		Math.floor(x / CELL_SIZE),
	) as Point;
	const tr = screenToWorld([canvas.width, 0]).map((x) =>
		Math.floor(x / CELL_SIZE),
	) as Point;
	const bl = screenToWorld([0, canvas.height]).map((x) =>
		Math.floor(x / CELL_SIZE),
	) as Point;
	const br = screenToWorld([canvas.width, canvas.height]).map((x) =>
		Math.floor(x / CELL_SIZE),
	) as Point;
	const alives = renderer.render(
		getEffectiveZoomExp(),
		BigInt(bl[0]),
		BigInt(bl[1]),
		BigInt(tr[0]),
		BigInt(tr[1]),
	);
	world.renderedCnt = alives.length / config.RENDER_OUTPUT_SIZE;
	updateStats();
	ctx.beginPath();
	for (let i = 0; i < alives.length; i += config.RENDER_OUTPUT_SIZE) {
		const x = Number(alives[i]) * CELL_SIZE,
			y = Number(alives[i + 1]) * CELL_SIZE,
			size_exp = Number(alives[i + 2]);
		const [translatedX, translatedY] = translateToScreen([x, y]);
		ctx.rect(
			translatedX,
			translatedY,
			CELL_SIZE * (1 << size_exp),
			CELL_SIZE * (1 << size_exp),
		);
	}
	ctx.fillStyle = "#000000";
	ctx.fill();
	if (config.CELL_SIZE_EXP + getEffectiveZoomExp() >= 0) {
		ctx.beginPath();
		ctx.strokeStyle = "#f0f0f0";
		for (let i = tl[0]; i <= tr[0]; ++i) {
			const start = translateToScreen([i * CELL_SIZE, (tl[1] + 1) * CELL_SIZE]);
			const end = translateToScreen([i * CELL_SIZE, (bl[1] - 1) * CELL_SIZE]);
			ctx.moveTo(...start);
			ctx.lineTo(...end);
		}
		for (let j = bl[1]; j <= tl[1]; ++j) {
			const start = translateToScreen([(tl[0] - 1) * CELL_SIZE, j * CELL_SIZE]);
			const end = translateToScreen([(tr[0] + 1) * CELL_SIZE, j * CELL_SIZE]);
			ctx.moveTo(...start);
			ctx.lineTo(...end);
		}
		ctx.stroke();
	}
	document.body.classList.add("ready"); //fouc from empty spans
	requestAnimationFrame(repaint);
}
/** do this outside of canvas to avoid float imprecision */
function translateToScreen(p: Point): Point {
	return [
		p[0] - world.centre[0] / getEffectiveZoom(),
		p[1] - world.centre[1] / -getEffectiveZoom(),
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
function getEffectiveZoomExp() {
	return Math.trunc(world.zoomExpFloat);
}
export function getEffectiveZoom() {
	return 2 ** getEffectiveZoomExp();
}
