import {
	type PerfStats,
	Renderer,
	WorldPoint as RustPoint,
} from "../pkg/game_of_life.js";

export type Point = [number, number];

export const CELL_SIZE = 50;
export const WORLD_BORDER = 1e14;

if (WORLD_BORDER * CELL_SIZE > Number.MAX_SAFE_INTEGER) {
	throw "WORLD_BORDER*CELL_SIZE>max safe int";
}

class World {
	centre: Point = [0, 0];
	renderedCnt = 0;
	zoom = 1;
	ticking = false;
	frameCounter = 0;
	prevFpsTime = 0;
	fps = 0;
	stepExp = 0;
	generation = 0n;
	worldCursor: Point = [-1, -1];
}

export const world = new World();
export const renderer = new Renderer(world.stepExp, CELL_SIZE);
export const canvas = document.getElementById("grid") as HTMLCanvasElement;
requestAnimationFrame(repaint);
function updateStats() {
	document.getElementById("stats-zoom")!.textContent =
		`Zoom: ${world.zoom.toPrecision(3)}`;
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
	ctx.setTransform(
		world.zoom,
		0,
		0,
		-world.zoom,
		canvas.width / 2,
		canvas.height / 2,
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
		world.zoom,
		new RustPoint(BigInt(bl[0]), BigInt(bl[1])),
		new RustPoint(BigInt(tr[0]), BigInt(tr[1])),
	);
	world.renderedCnt = alives.length;
	updateStats();
	for (const point of alives) {
		const x = Number(point.min.x) * CELL_SIZE;
		const y = Number(point.min.y) * CELL_SIZE;
		const [translatedX, translatedY] = translateToScreen([x, y]);
		ctx.fillRect(
			translatedX,
			translatedY,
			CELL_SIZE * (1 << point.size_exp),
			CELL_SIZE * (1 << point.size_exp),
		);
	}
	if (CELL_SIZE * world.zoom >= 1) {
		for (let i = tl[0]; i <= tr[0]; ++i) {
			ctx.beginPath();
			ctx.strokeStyle = "#f0f0f0";
			const start = translateToScreen([i * CELL_SIZE, (tl[1] + 1) * CELL_SIZE]);
			const end = translateToScreen([i * CELL_SIZE, (bl[1] - 1) * CELL_SIZE]);
			ctx.moveTo(...start);
			ctx.lineTo(...end);
			ctx.stroke();
			ctx.closePath();
		}
		for (let j = bl[1]; j <= tl[1]; ++j) {
			ctx.beginPath();
			ctx.strokeStyle = "#f0f0f0";
			const start = translateToScreen([(tl[0] - 1) * CELL_SIZE, j * CELL_SIZE]);
			const end = translateToScreen([(tr[0] + 1) * CELL_SIZE, j * CELL_SIZE]);
			ctx.moveTo(...start);
			ctx.lineTo(...end);
			ctx.stroke();
			ctx.closePath();
		}
	}
	ctx.strokeStyle = "#000000";
	const border = translateToScreen([
		-WORLD_BORDER * CELL_SIZE,
		-WORLD_BORDER * CELL_SIZE,
	]);
	ctx.strokeRect(
		border[0],
		border[1],
		(WORLD_BORDER * 2 + 1) * CELL_SIZE,
		(WORLD_BORDER * 2 + 1) * CELL_SIZE,
	);
	document.body.classList.add("ready"); //fouc from empty spans
	requestAnimationFrame(repaint);
}
/** do this outside of canvas to avoid float imprecision */
function translateToScreen(p: Point): Point {
	return [
		p[0] - world.centre[0] / world.zoom,
		p[1] - world.centre[1] / -world.zoom,
	];
}
export function screenToWorld(p: Point): Point {
	return [
		(p[0] + world.centre[0] - canvas.width / 2) / world.zoom,
		(p[1] + world.centre[1] - canvas.height / 2) / -world.zoom,
	];
}
export function next_step() {
	renderer.next_step();
	world.generation += 2n ** BigInt(world.stepExp);
}
