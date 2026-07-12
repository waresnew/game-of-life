import { canvasDims, CELL_SIZE, WORLD_BORDER, type Point } from './shared.svelte';
import { camera, getEffectiveZoomOutExp, uiState } from './shared.svelte';
import { config, renderer } from './wasm';

export class GestureHandler {
	currentPointers: Map<number, Point> = new Map();
	drawSession: Set<string> = new Set(); // js quirk
	prevPanX = -1;
	prevPanY = -1;
	pointerInCanvas = false;
	prevTouchZoomDist = -1;
	handleTouchZoom(event: PointerEvent) {
		if (this.currentPointers.size == 2) {
			const [[x1, y1], [x2, y2]] = Array.from(this.currentPointers.values()) as [Point, Point];
			const dx = x1 - x2;
			const dy = y1 - y2;
			const dist = Math.sqrt(dx * dx + dy * dy);
			if (this.prevTouchZoomDist != -1) {
				this.zoom((dist - this.prevTouchZoomDist) * 0.002);
			}
			this.prevTouchZoomDist = dist;
		}
	}
	getAveragePoint(points: Point[]): Point {
		let sumX = 0;
		let sumY = 0;
		for (const [x, y] of points) {
			sumX += x;
			sumY += y;
		}
		return [sumX / points.length, sumY / points.length];
	}
	handlePan(event: PointerEvent) {
		let [pointerX, pointerY] =
			event.pointerType == 'mouse'
				? [event.clientX, event.clientY]
				: this.getAveragePoint(Array.from(this.currentPointers.values()));
		pointerX -= canvasDims.x;
		pointerY -= canvasDims.y;
		if (event.buttons == 2 || this.currentPointers.size == 2) {
			if (this.prevPanX != -1 && this.prevPanY != -1) {
				camera.centre = [
					-(pointerX - this.prevPanX) + camera.centre[0],
					-(pointerY - this.prevPanY) + camera.centre[1]
				];
			}
			this.prevPanX = pointerX;
			this.prevPanY = pointerY;
		}
	}
	updateCursors(event: PointerEvent) {
		const mouseX = event.clientX - canvasDims.x;
		const mouseY = event.clientY - canvasDims.y;
		if (mouseX < 0 || mouseX > canvasDims.width || mouseY < 0 || mouseY > canvasDims.height) {
			this.currentPointers.delete(event.pointerId);
			return;
		}
		uiState.worldCursor = this.screenToWorld([mouseX, mouseY]);
		if (event.pointerType != 'mouse') {
			this.currentPointers.set(event.pointerId, [event.clientX, event.clientY]);
		}
	}
	handleDrawMove(event: PointerEvent) {
		if (event.pointerType == 'mouse' && event.buttons == 1) {
			this.doDraw();
		} else if (this.currentPointers.size == 1) {
			setTimeout(() => {
				// user may have small delay b/n pressing 2 fingers
				if (this.currentPointers.size == 1) {
					this.doDraw();
				}
			}, 50);
		}
	}
	handleDrawStart(event: PointerEvent) {
		if (event.pointerType == 'mouse' && event.button == 0) {
			this.doDraw();
		} else if (this.currentPointers.size == 1) {
			setTimeout(() => {
				if (this.currentPointers.size == 1) {
					this.doDraw();
				}
			}, 50);
		}
	}
	endDrawSession(event: PointerEvent) {
		this.currentPointers.delete(event.pointerId);
		this.drawSession.clear();
		this.prevPanX = this.prevPanY = -1;
		this.prevTouchZoomDist = -1;
	}
	zoom(zoomDelta: number) {
		let newZoomExpFloat = camera.zoomOutExpFloat + zoomDelta;
		newZoomExpFloat = Math.min(config.MAX_HEIGHT, Math.max(0, newZoomExpFloat));
		const newZoom = 2 ** -Math.trunc(newZoomExpFloat);
		camera.centre = [
			newZoom * uiState.worldCursor[0] -
				this.getEffectiveZoom() * uiState.worldCursor[0] +
				camera.centre[0],
			-newZoom * uiState.worldCursor[1] +
				this.getEffectiveZoom() * uiState.worldCursor[1] +
				camera.centre[1]
		];
		camera.zoomOutExpFloat = newZoomExpFloat;
	}
	doDraw() {
		const cell: Point = [
			Math.floor(uiState.worldCursor[0] / CELL_SIZE),
			Math.floor(uiState.worldCursor[1] / CELL_SIZE)
		];
		if (
			cell[0] > WORLD_BORDER ||
			cell[0] < -WORLD_BORDER ||
			cell[1] > WORLD_BORDER ||
			cell[1] < -WORLD_BORDER
		) {
			alert(
				`Cannot draw if x or y is outside of [-${WORLD_BORDER.toExponential()}, ${WORLD_BORDER.toExponential()}]`
			);
			this.drawSession.clear();
			return;
		}
		const strCell = cell.join(' ');
		if (this.drawSession.has(strCell)) {
			return;
		}
		this.drawSession.add(strCell);
		renderer.toggle_cell(BigInt(cell[0]), BigInt(cell[1]));
	}
	screenToWorld(p: Point): Point {
		return [
			(p[0] + camera.centre[0] - canvasDims.width / 2) / this.getEffectiveZoom(),
			(p[1] + camera.centre[1] - canvasDims.height / 2) / -this.getEffectiveZoom()
		];
	}
	getEffectiveZoom() {
		return 2 ** -getEffectiveZoomOutExp();
	}
}
