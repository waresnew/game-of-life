import { ViewportInfo } from '$wasm/game_of_life';
import { canvasDims, toRustScreenPoint, type Point } from './shared.svelte';
import { uiState } from './shared.svelte';
import { renderer } from './wasm';

export class GestureHandler {
	currentPointers: Map<number, Point> = new Map();
	prevPanX = -1;
	prevPanY = -1;
	pointerInCanvas = false;
	prevTouchZoomDist = -1;
	zoomProgress = 0;
	zoom(delta: number) {
		this.zoomProgress += delta;
		if (Math.abs(this.zoomProgress) >= 1) {
			const whole = Math.trunc(this.zoomProgress);
			this.zoomProgress -= whole;
			renderer.handle_zoom(whole, toRustScreenPoint(uiState.cursor));
		}
	}
	handleTouchZoom(event: PointerEvent) {
		if (this.currentPointers.size == 2) {
			const [[x1, y1], [x2, y2]] = Array.from(this.currentPointers.values()) as [Point, Point];
			const dx = x1 - x2;
			const dy = y1 - y2;
			const dist = Math.sqrt(dx * dx + dy * dy);
			if (this.prevTouchZoomDist != -1) {
				this.zoom((dist - this.prevTouchZoomDist) * -0.01);
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

		renderer.update_viewport(
			new ViewportInfo(
				toRustScreenPoint([canvasDims.width, canvasDims.height]),
				toRustScreenPoint(uiState.cursor)
			)
		);
		if (event.buttons == 2 || this.currentPointers.size == 2) {
			if (this.prevPanX != -1 && this.prevPanY != -1) {
				renderer.handle_pan(
					toRustScreenPoint([-(pointerX - this.prevPanX), -(pointerY - this.prevPanY)])
				);
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
		renderer.update_viewport(
			new ViewportInfo(
				toRustScreenPoint([canvasDims.width, canvasDims.height]),
				toRustScreenPoint([mouseX, mouseY])
			)
		);
		uiState.cursor = [mouseX, mouseY];
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
		renderer.end_draw_session();
		this.prevPanX = this.prevPanY = -1;
		this.prevTouchZoomDist = -1;
	}
	doDraw() {
		renderer.handle_draw(toRustScreenPoint(uiState.cursor));
	}
}
