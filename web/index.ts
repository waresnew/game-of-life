import init, { solve_wasm } from "../pkg/game_of_life.js";
import wasmUrl from "../pkg/game_of_life_bg.wasm";

await init(wasmUrl);

type Point = [number, number];
class World {
	centre: Point = [0, 0];
	alive: Set<string> = new Set();
	renderedCnt = 0;
	zoom = 1;
	ticking = false;
	prevTickTime = 0;
	tps = 0;
	tickAccum = 0;
	prevReportTpsTime = 0;
	dragSession: Set<string> = new Set();
}

const world = new World();
const canvas = document.getElementById("grid") as HTMLCanvasElement;
let worldCursor: Point = [-1, -1];
const CELL_SIZE = 50;
const TPS = 60;
function updateStats() {
	document.getElementById("debug-centre")!.textContent =
		`Centre: (${Math.floor(world.centre[0] / CELL_SIZE)},${Math.floor(world.centre[1] / CELL_SIZE)})`;
	document.getElementById("stats-zoom")!.textContent =
		`Zoom: ${world.zoom.toPrecision(3)}`;
	document.getElementById("stats-cursor")!.textContent =
		`Cursor: (${Math.floor(worldCursor[0] / CELL_SIZE)},${Math.floor(worldCursor[1] / CELL_SIZE)})`;
	document.getElementById("debug-rendered")!.textContent =
		`Rendered: ${world.renderedCnt}`;
	document.getElementById("stats-tps")!.textContent = `TPS: ${world.tps}`;
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
	if (world.ticking) {
		if (time - world.prevReportTpsTime >= 1000) {
			world.tps = world.tickAccum;
			world.tickAccum = 0;
			world.prevReportTpsTime = time;
		}
		if (time - world.prevTickTime >= 1000 / TPS) {
			++world.tickAccum;
			next_step();
			world.prevTickTime = time;
		}
	} else {
		world.tickAccum = world.tps = 0;
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
canvas.addEventListener("mousemove", (event) => {
	if (event.buttons == 1) {
		const cell: Point = [
			Math.floor(worldCursor[0] / CELL_SIZE),
			Math.floor(worldCursor[1] / CELL_SIZE),
		];
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
});
function next_step() {
	const flattened = BigInt64Array.from(
		Array.from(world.alive).flatMap((s) => {
			const [x, y] = s.split(" ") as [string, string];
			return [parseInt(x), parseInt(y)];
		}),
		BigInt,
	);
	const res = solve_wasm(flattened, 1n);
	world.alive.clear();
	for (let i = 0; i < res.length; i += 2) {
		world.alive.add([res[i], res[i + 1]].join(" "));
	}
}
const playButton = document.getElementById("play")!;
playButton.addEventListener("click", (event) => {
	if (playButton.textContent == "Play") {
		world.ticking = true;
		playButton.textContent = "Stop";
	} else {
		world.ticking = false;
		playButton.textContent = "Play";
	}
});
document.getElementById("jump")!.addEventListener("click", (event) => {
	next_step();
});
