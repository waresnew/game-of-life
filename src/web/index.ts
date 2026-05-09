type Point = [number, number];
class World {
	centre: Point = [0, 0];
	alive: Set<string> = new Set();
	zoom = 1;
}

const world = new World();
const canvas = document.getElementById("grid") as HTMLCanvasElement;
let worldCursor: Point = [-1, -1];
const CELL_SIZE = 10;
function updateStats() {
	document.getElementById("stats-centre")!.textContent =
		`Centre: (${Math.floor(world.centre[0])},${Math.floor(world.centre[1])})`;
	document.getElementById("stats-zoom")!.textContent =
		`Zoom: ${world.zoom.toPrecision(3)}`;
	document.getElementById("stats-cursor")!.textContent =
		`Cursor: (${Math.floor(worldCursor[0] / CELL_SIZE)},${Math.floor(worldCursor[1] / CELL_SIZE)})`;
}
function repaint() {
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
	const tl = inverseTransform([0, 0]);
	const tr = inverseTransform([canvas.width, 0]);
	const bl = inverseTransform([0, canvas.height]);
	const br = inverseTransform([canvas.width, canvas.height]);
	for (const s of world.alive) {
		const [x, y] = s.split(" ").map((x) => parseInt(x)) as Point;
		ctx.fillRect(x, y, CELL_SIZE, CELL_SIZE);
	}
	if (CELL_SIZE * world.zoom >= 1) {
		for (
			let i = Math.ceil(tl[0] / CELL_SIZE) * CELL_SIZE;
			i <= Math.floor(tr[0] / CELL_SIZE) * CELL_SIZE;
			i += CELL_SIZE
		) {
			ctx.beginPath();
			ctx.strokeStyle = "#f0f0f0";
			ctx.moveTo(i, tl[1]);
			ctx.lineTo(i, bl[1]);
			ctx.stroke();
			ctx.closePath();
		}
		for (
			let j = Math.ceil(bl[1] / CELL_SIZE) * CELL_SIZE;
			j <= Math.floor(tl[1] / CELL_SIZE) * CELL_SIZE;
			j += CELL_SIZE
		) {
			ctx.beginPath();
			ctx.strokeStyle = "#f0f0f0";
			ctx.moveTo(tl[0], j);
			ctx.lineTo(tr[0], j);
			ctx.stroke();
			ctx.closePath();
		}
	}
	requestAnimationFrame(() => repaint());
}
requestAnimationFrame(() => repaint());
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
	const newZoom = world.zoom * zoomFactor;
	world.centre = [
		newZoom * worldCursor[0] - world.zoom * worldCursor[0] + world.centre[0],
		newZoom * worldCursor[1] - world.zoom * worldCursor[1] + world.centre[1],
	];
	world.zoom = newZoom;
});
let prevMouseX = -1,
	prevMouseY = -1;
canvas.addEventListener("mousemove", (event) => {
	const dims = canvas.getBoundingClientRect();
	const mouseX = event.clientX - dims.x;
	const mouseY = event.clientY - dims.y;
	worldCursor = inverseTransform([mouseX, mouseY]);
	if (event.buttons == 1) {
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
canvas.addEventListener("click", (event) => {
	const cell: Point = [
		Math.floor(worldCursor[0] / CELL_SIZE) * CELL_SIZE,
		Math.floor(worldCursor[1] / CELL_SIZE) * CELL_SIZE,
	];
	if (world.alive.has(cell.join(" "))) {
		world.alive.delete(cell.join(" "));
	} else {
		world.alive.add(cell.join(" "));
	}
});
