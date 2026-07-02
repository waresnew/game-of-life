import init, {
	type PerfStats,
	Point as RustPoint,
	Solver,
} from "../pkg/game_of_life.js";

// @ts-expect-error
// the generated js doesn't use a relative path for wasm so bun ignores it
import wasmUrl from "../pkg/game_of_life_bg.wasm";

const wasm = await init(wasmUrl);
export type Point = [number, number];

export const CELL_SIZE = 50;
export const WORLD_BORDER = 1e14;

if (WORLD_BORDER * CELL_SIZE > Number.MAX_SAFE_INTEGER) {
	throw "WORLD_BORDER*CELL_SIZE>max safe int";
}

class World {
	centre: Point = [0, 0];
	alive: Set<string> = new Set();
	renderedCnt = 0;
	zoom = 1;
	ticking = false;
	frameCounter = 0;
	prevFpsTime = 0;
	fps = 0;
	stepExp = 0;
	generation = 0n;
	dirty = true;
	worldCursor: Point = [-1, -1];
}

export const world = new World();
let solver: Solver | null = null;
export const canvas = document.getElementById("grid") as HTMLCanvasElement;
resizeCanvas();
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
		`Alive: ${world.alive.size}`;
	if (solver) {
		const totalCache =
			solver.perf_stats.cache_hits + solver.perf_stats.cache_misses;
		document.getElementById("debug-cache_hitrate")!.textContent =
			`Cache hit rate: ${totalCache > 0 ? (solver.perf_stats.cache_hits * 100n) / totalCache : "0"}%`;
	} else {
		document.getElementById("debug-cache_hitrate")!.textContent =
			"Cache hit rate: 0%";
	}
	document.getElementById("debug-memory")!.textContent =
		`Wasm memory: ${Math.round(wasm.memory.buffer.byteLength / 1e6)} MB`;
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
window.addEventListener("resize", (event) => resizeCanvas());
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

	updateStats();
	const ctx = canvas.getContext("2d")!;
	ctx.resetTransform();
	ctx.clearRect(0, 0, canvas.width, canvas.height);
	ctx.setTransform(
		world.zoom,
		0,
		0,
		-world.zoom,
		-world.centre[0] + canvas.width / 2,
		-world.centre[1] + canvas.height / 2,
	);
	const tl = inverseTransform([0, 0]).map((x) =>
		Math.floor(x / CELL_SIZE),
	) as Point;
	const tr = inverseTransform([canvas.width, 0]).map((x) =>
		Math.floor(x / CELL_SIZE),
	) as Point;
	const bl = inverseTransform([0, canvas.height]).map((x) =>
		Math.floor(x / CELL_SIZE),
	) as Point;
	const br = inverseTransform([canvas.width, canvas.height]).map((x) =>
		Math.floor(x / CELL_SIZE),
	) as Point;
	let renderedCnt = 0;
	for (const s of world.alive) {
		const [x, y] = s.split(" ").map((x) => parseInt(x)) as Point;
		const tl_inside = tl[0] <= x && x <= tr[0] && bl[1] <= y && y <= tl[1];
		const br_inside =
			tl[0] <= x + 1 && x + 1 <= tr[0] && bl[1] <= y - 1 && y - 1 <= tl[1];
		if (tl_inside || br_inside) {
			ctx.fillRect(x * CELL_SIZE, y * CELL_SIZE, CELL_SIZE, CELL_SIZE);
			++renderedCnt;
		}
	}
	world.renderedCnt = renderedCnt;
	if (CELL_SIZE * world.zoom >= 1) {
		for (let i = tl[0]; i <= tr[0]; ++i) {
			ctx.beginPath();
			ctx.strokeStyle = "#f0f0f0";
			ctx.moveTo(i * CELL_SIZE, (tl[1] + 1) * CELL_SIZE);
			ctx.lineTo(i * CELL_SIZE, (bl[1] - 1) * CELL_SIZE);
			ctx.stroke();
			ctx.closePath();
		}
		for (let j = bl[1]; j <= tl[1]; ++j) {
			ctx.beginPath();
			ctx.strokeStyle = "#f0f0f0";
			ctx.moveTo((tl[0] - 1) * CELL_SIZE, j * CELL_SIZE);
			ctx.lineTo((tr[0] + 1) * CELL_SIZE, j * CELL_SIZE);
			ctx.stroke();
			ctx.closePath();
		}
	}
	ctx.strokeStyle = "#000000";
	ctx.strokeRect(
		-WORLD_BORDER * CELL_SIZE,
		-WORLD_BORDER * CELL_SIZE,
		(WORLD_BORDER * 2 + 1) * CELL_SIZE,
		(WORLD_BORDER * 2 + 1) * CELL_SIZE,
	);
	requestAnimationFrame(repaint);
}
export function inverseTransform(p: Point): Point {
	return [
		(p[0] + world.centre[0] - canvas.width / 2) / world.zoom,
		(p[1] + world.centre[1] - canvas.height / 2) / -world.zoom,
	];
}
export function next_step() {
	if (solver == null || world.dirty) {
		world.dirty = false;
		const formatted = Array.from(world.alive).map((s) => {
			const [x, y] = s.split(" ") as [string, string];
			return new RustPoint(BigInt(parseInt(x)), BigInt(parseInt(y)));
		});
		solver = new Solver(formatted, world.stepExp);
	}

	const res = solver.solve();
	world.alive.clear();
	for (const coord of res) {
		world.alive.add([coord.x, coord.y].join(" "));
	}
	world.generation += 2n ** BigInt(world.stepExp);
}
