import { get_config, type PerfStats, Renderer } from "../pkg/game_of_life.js";

export type Point = [number, number];

export const config = get_config();
export const WORLD_BORDER = 1 << config.MAX_HEIGHT;

if (WORLD_BORDER * config.CELL_SIZE > Number.MAX_SAFE_INTEGER) {
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
export const renderer = new Renderer(world.stepExp);
export const canvas = document.getElementById("grid") as HTMLCanvasElement;
requestAnimationFrame(repaint);
function updateStats() {
	document.getElementById("stats-zoom")!.textContent =
		`Zoom: ${world.zoom.toPrecision(3)}`;
	document.getElementById("stats-cursor")!.textContent =
		`Cursor: (${Math.floor(world.worldCursor[0] / config.CELL_SIZE)},${Math.floor(world.worldCursor[1] / config.CELL_SIZE)})`;
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
		Math.floor(x / config.CELL_SIZE),
	) as Point;
	const tr = screenToWorld([canvas.width, 0]).map((x) =>
		Math.floor(x / config.CELL_SIZE),
	) as Point;
	const bl = screenToWorld([0, canvas.height]).map((x) =>
		Math.floor(x / config.CELL_SIZE),
	) as Point;
	const br = screenToWorld([canvas.width, canvas.height]).map((x) =>
		Math.floor(x / config.CELL_SIZE),
	) as Point;
	const alives = renderer.render(
		world.zoom,
		BigInt(bl[0]),
		BigInt(bl[1]),
		BigInt(tr[0]),
		BigInt(tr[1]),
	);
	let minSizeExp = 1000;
	world.renderedCnt = alives.length / config.RENDER_OUTPUT_SIZE;
	updateStats();
	ctx.beginPath();
	for (let i = 0; i < alives.length; i += config.RENDER_OUTPUT_SIZE) {
		const x = Number(alives[i]) * config.CELL_SIZE,
			y = Number(alives[i + 1]) * config.CELL_SIZE,
			size_exp = Number(alives[i + 2]);
		const [translatedX, translatedY] = translateToScreen([x, y]);
		ctx.rect(
			translatedX,
			translatedY,
			config.CELL_SIZE * (1 << size_exp),
			config.CELL_SIZE * (1 << size_exp),
		);
		minSizeExp = Math.min(minSizeExp, size_exp);
	}
	console.log(minSizeExp);
	ctx.fill();
	if (config.CELL_SIZE * world.zoom >= 1) {
		ctx.beginPath();
		ctx.strokeStyle = "#f0f0f0";
		for (let i = tl[0]; i <= tr[0]; ++i) {
			const start = translateToScreen([
				i * config.CELL_SIZE,
				(tl[1] + 1) * config.CELL_SIZE,
			]);
			const end = translateToScreen([
				i * config.CELL_SIZE,
				(bl[1] - 1) * config.CELL_SIZE,
			]);
			ctx.moveTo(...start);
			ctx.lineTo(...end);
		}
		for (let j = bl[1]; j <= tl[1]; ++j) {
			const start = translateToScreen([
				(tl[0] - 1) * config.CELL_SIZE,
				j * config.CELL_SIZE,
			]);
			const end = translateToScreen([
				(tr[0] + 1) * config.CELL_SIZE,
				j * config.CELL_SIZE,
			]);
			ctx.moveTo(...start);
			ctx.lineTo(...end);
		}
		ctx.stroke();
	}
	ctx.strokeStyle = "#000000";
	const border = translateToScreen([
		-WORLD_BORDER * config.CELL_SIZE,
		-WORLD_BORDER * config.CELL_SIZE,
	]);
	ctx.strokeRect(
		border[0],
		border[1],
		(WORLD_BORDER * 2 + 1) * config.CELL_SIZE,
		(WORLD_BORDER * 2 + 1) * config.CELL_SIZE,
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
