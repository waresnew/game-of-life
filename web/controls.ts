import {
	canvas,
	config,
	type Point,
	renderer,
	screenToWorld,
	WORLD_BORDER,
	world,
} from "./app";

const currentPointers: Map<number, Point> = new Map();
const drawSession: Set<string> = new Set(); // js quirk
let prevPanX = -1;
let prevPanY = -1;
const pointerInCanvas = false;
let prevTouchZoomDist = -1;

canvas.addEventListener("pointermove", (event) => {
	updateCursors(event);
	handlePan(event);
	handleDrawMove(event);
	handleTouchZoom(event);
});
canvas.addEventListener("pointerdown", (event) => {
	updateCursors(event);
	handleDrawStart(event);
});
canvas.addEventListener("pointerup", (event) => {
	endDrawSession(event);
});
canvas.addEventListener("pointercancel", (event) => {
	endDrawSession(event);
});
canvas.addEventListener("wheel", (event) => {
	event.preventDefault();
	const mouseDelta = -event.deltaY * 0.001;
	zoom(Math.exp(mouseDelta));
});

function handleTouchZoom(event: PointerEvent) {
	if (currentPointers.size == 2) {
		const [[x1, y1], [x2, y2]] = Array.from(currentPointers.values()) as [
			Point,
			Point,
		];
		const dx = x1 - x2;
		const dy = y1 - y2;
		const dist = Math.sqrt(dx * dx + dy * dy);
		if (prevTouchZoomDist != -1) {
			zoom(Math.exp((dist - prevTouchZoomDist) * 0.002));
		}
		prevTouchZoomDist = dist;
	}
}
function getAveragePoint(points: Point[]): Point {
	let sumX = 0;
	let sumY = 0;
	for (const [x, y] of points) {
		sumX += x;
		sumY += y;
	}
	return [sumX / points.length, sumY / points.length];
}
function handlePan(event: PointerEvent) {
	const dims = canvas.getBoundingClientRect();
	let [pointerX, pointerY] =
		event.pointerType == "mouse"
			? [event.clientX, event.clientY]
			: getAveragePoint(Array.from(currentPointers.values()));
	pointerX -= dims.x;
	pointerY -= dims.y;
	if (event.buttons == 2 || currentPointers.size == 2) {
		if (prevPanX != -1 && prevPanY != -1) {
			world.centre = [
				-(pointerX - prevPanX) + world.centre[0],
				-(pointerY - prevPanY) + world.centre[1],
			];
		}
		prevPanX = pointerX;
		prevPanY = pointerY;
	}
}
function updateCursors(event: PointerEvent) {
	const dims = canvas.getBoundingClientRect();
	const mouseX = event.clientX - dims.x;
	const mouseY = event.clientY - dims.y;
	if (
		mouseX < 0 ||
		mouseX > canvas.width ||
		mouseY < 0 ||
		mouseY > canvas.height
	) {
		currentPointers.delete(event.pointerId);
		return;
	}
	world.worldCursor = screenToWorld([mouseX, mouseY]);
	if (event.pointerType != "mouse") {
		currentPointers.set(event.pointerId, [event.clientX, event.clientY]);
	}
}
function handleDrawMove(event: PointerEvent) {
	if (event.pointerType == "mouse" && event.buttons == 1) {
		doDraw();
	} else if (currentPointers.size == 1) {
		console.log(currentPointers);
		setTimeout(() => {
			// user may have small delay b/n pressing 2 fingers
			if (currentPointers.size == 1) {
				doDraw();
			}
		}, 50);
	}
}
function handleDrawStart(event: PointerEvent) {
	if (event.pointerType == "mouse" && event.button == 0) {
		doDraw();
	} else if (currentPointers.size == 1) {
		setTimeout(() => {
			if (currentPointers.size == 1) {
				doDraw();
			}
		}, 50);
	}
}
function endDrawSession(event: PointerEvent) {
	currentPointers.delete(event.pointerId);
	drawSession.clear();
	prevPanX = prevPanY = -1;
	prevTouchZoomDist = -1;
}
function zoom(zoomFactor: number) {
	let newZoom = world.zoom * zoomFactor;
	newZoom = Math.min(1, newZoom);
	world.centre = [
		newZoom * world.worldCursor[0] -
			world.zoom * world.worldCursor[0] +
			world.centre[0],
		-newZoom * world.worldCursor[1] +
			world.zoom * world.worldCursor[1] +
			world.centre[1],
	];
	world.zoom = newZoom;
}
function doDraw() {
	const cell: Point = [
		Math.floor(world.worldCursor[0] / config.CELL_SIZE),
		Math.floor(world.worldCursor[1] / config.CELL_SIZE),
	];
	if (
		cell[0] > WORLD_BORDER ||
		cell[0] < -WORLD_BORDER ||
		cell[1] > WORLD_BORDER ||
		cell[1] < -WORLD_BORDER
	) {
		alert(
			`Cannot draw if x or y is outside of [-${WORLD_BORDER.toExponential()}, ${WORLD_BORDER.toExponential()}]`,
		);
		drawSession.clear();
		return;
	}
	const strCell = cell.join(" ");
	if (drawSession.has(strCell)) {
		return;
	}
	drawSession.add(strCell);
	renderer.toggle_cell(BigInt(cell[0]), BigInt(cell[1]));
}
