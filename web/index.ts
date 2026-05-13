import init, {
	type PerfStats,
	Point as RustPoint,
	Solver,
} from "../pkg/game_of_life.js";
import wasmUrl from "../pkg/game_of_life_bg.wasm";

const wasm = await init(wasmUrl);

type Point = [number, number];

const CELL_SIZE = 50;
const TPS = 60;
const WORLD_BOUND = 1e15;

class World {
	#centre: Point = [0, 0];
	alive: Set<string> = new Set();
	renderedCnt = 0;
	zoom = 1;
	ticking = false;
	frameCounter = 0;
	prevTpsTime = 0;
	tps = 0;
	dragSession: Set<string> = new Set();
	stepSize = 1n;
	generation = 0n;
	dirty = false;
	get centre() {
		return this.#centre;
	}
	set centre(p) {
		this.#centre = [
			Math.max(-WORLD_BOUND, Math.min(p[0], WORLD_BOUND)),
			Math.max(-WORLD_BOUND, Math.min(p[1], WORLD_BOUND)),
		];
	}
}

const world = new World();
const solver = new Solver();
const canvas = document.getElementById("grid") as HTMLCanvasElement;
let worldCursor: Point = [-1, -1];
function updateStats() {
	document.getElementById("stats-zoom")!.textContent =
		`Zoom: ${world.zoom.toPrecision(3)}`;
	document.getElementById("stats-cursor")!.textContent =
		`Cursor: (${Math.floor(worldCursor[0] / CELL_SIZE)},${Math.floor(worldCursor[1] / CELL_SIZE)})`;
	document.getElementById("debug-rendered")!.textContent =
		`Rendered: ${world.renderedCnt}`;
	document.getElementById("stats-tps")!.textContent = `TPS: ${world.tps}`;
	document.getElementById("stats-alive")!.textContent =
		`Alive: ${world.alive.size}`;
	const totalCache =
		solver.perf_stats.cache_hits + solver.perf_stats.cache_misses;
	document.getElementById("debug-cache_hitrate")!.textContent =
		`Cache hit rate: ${totalCache > 0 ? (solver.perf_stats.cache_hits * 100n) / totalCache : "0"}%`;
	document.getElementById("debug-memory")!.textContent =
		`Wasm memory: ${Math.round(wasm.memory.buffer.byteLength / 1e6)} MB`;
	document.getElementById("stats-generation")!.textContent =
		`Generation: ${world.generation}`;
}
document.getElementById("toggle-debug")!.addEventListener("click", (event) => {
	const debug = document.getElementById("debug")!;
	if (debug.style.visibility == "hidden") {
		debug.style.visibility = "visible";
	} else {
		debug.style.visibility = "hidden";
	}
});
function repaint(time: DOMHighResTimeStamp) {
	++world.frameCounter;
	if (time - world.prevTpsTime >= 1000) {
		world.tps = world.frameCounter;
		world.frameCounter = 0;
		world.prevTpsTime = time;
	}
	if (world.ticking) {
		const start = performance.now();
		next_step();
		document.getElementById("play-runtime")!.textContent =
			`Took ${(performance.now() - start) / 1000}s`;
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
	requestAnimationFrame(repaint);
}
requestAnimationFrame(repaint);
function inverseTransform(p: Point): Point {
	return [
		(p[0] + world.centre[0] - canvas.width / 2) / world.zoom,
		(p[1] + world.centre[1] - canvas.height / 2) / -world.zoom,
	];
}
canvas.addEventListener("wheel", (event) => {
	//TODO: pinch support (mobile)

	const mouseDelta = -event.deltaY * 0.001;
	const zoomFactor = Math.exp(mouseDelta);
	let newZoom = world.zoom * zoomFactor;
	newZoom = Math.min(1, newZoom);
	world.centre = [
		newZoom * worldCursor[0] - world.zoom * worldCursor[0] + world.centre[0],
		-newZoom * worldCursor[1] + world.zoom * worldCursor[1] + world.centre[1],
	];
	world.zoom = newZoom;
});
canvas.addEventListener("contextmenu", (event) => event.preventDefault());
let prevMouseX = -1,
	prevMouseY = -1;
canvas.addEventListener("mousemove", (event) => {
	const dims = canvas.getBoundingClientRect();
	const mouseX = event.clientX - dims.x;
	const mouseY = event.clientY - dims.y;
	worldCursor = inverseTransform([mouseX, mouseY]);
	if (event.buttons == 2) {
		if (prevMouseX != -1 && prevMouseY != -1) {
			world.centre = [
				-(mouseX - prevMouseX) + world.centre[0],
				-(mouseY - prevMouseY) + world.centre[1],
			];
		}
		prevMouseX = mouseX;
		prevMouseY = mouseY;
	} else {
		prevMouseX = prevMouseY = -1;
	}
});
canvas.addEventListener("mouseup", (event) => {
	world.dragSession.clear();
});
function doDrag() {
	world.dirty = true;
	const cell: Point = [
		Math.floor(worldCursor[0] / CELL_SIZE),
		Math.floor(worldCursor[1] / CELL_SIZE),
	];
	if (
		cell[0] > WORLD_BOUND ||
		cell[0] < -WORLD_BOUND ||
		cell[1] > WORLD_BOUND ||
		cell[1] < -WORLD_BOUND
	) {
		alert(`Cannot draw if x or y is outside of [-1e15, 1e15]`);
		world.dragSession.clear();
		return;
	}
	const strCell = cell.join(" ");
	if (world.dragSession.has(strCell)) {
		return;
	}
	world.dragSession.add(strCell);
	if (world.alive.has(strCell)) {
		world.alive.delete(strCell);
	} else {
		world.alive.add(strCell);
	}
}
canvas.addEventListener("mousemove", (event) => {
	if (event.buttons == 1) {
		doDrag();
	}
});
canvas.addEventListener("mousedown", (event) => {
	if (event.button == 0) {
		doDrag();
	}
});
function next_step() {
	if (world.dirty) {
		world.dirty = false;
		const formatted = Array.from(world.alive).map((s) => {
			const [x, y] = s.split(" ") as [string, string];
			return new RustPoint(BigInt(parseInt(x)), BigInt(parseInt(y)));
		});
		solver.reset(formatted);
	}

	const res = solver.solve(world.stepSize);
	world.alive.clear();
	for (const coord of res) {
		world.alive.add([coord.x, coord.y].join(" "));
	}
	world.generation += world.stepSize;
}
function updateStepSize() {
	const input = document.getElementById("stepsize") as HTMLInputElement;
	try {
		const x = BigInt(input.value);
		if (x < 1n) {
			alert("Step size must be positive");
			return;
		} else if (x > 18_446_744_073_709_551_615n) {
			alert("Step size must be smaller than 2^64-1");
		}
		world.stepSize = x;
	} catch (e) {
		alert("Step size must be an integer");
	}
}
//TODO: updatestepsize after each update to the textbox, don't alert every time tho (report another way)
document.getElementById("once-button")!.addEventListener("click", (event) => {
	updateStepSize();
	world.ticking = false;
	playButton.textContent = "Play";
	const start = performance.now();
	next_step();
	document.getElementById("once-runtime")!.textContent =
		`Took ${(performance.now() - start) / 1000}s`;
});
const playButton = document.getElementById("play-button")!;
playButton.addEventListener("click", (event) => {
	updateStepSize();
	if (playButton.textContent == "Play") {
		world.ticking = true;
		playButton.textContent = "Stop";
	} else {
		world.ticking = false;
		playButton.textContent = "Play";
	}
});
