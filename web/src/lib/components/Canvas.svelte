<script lang="ts">
	import { ScreenPoint as RustScreenPoint, ViewportInfo } from '$wasm/game_of_life.js';
	import { onMount } from 'svelte';
	import {
		uiState,
		updatePerfStats,
		canvasDims,
		fpsCounters,
		next_step,
		setCanvasDims,
		type Point,
		updateRenderStats,
		toRustScreenPoint
	} from '$lib/shared.svelte';
	import { GestureHandler } from '$lib/GestureHandler.js';
	import { renderer } from '$lib/wasm.js';

	let canvas: HTMLCanvasElement;
	const gestureHandler = new GestureHandler();
	function repaint(time: DOMHighResTimeStamp) {
		++fpsCounters.frameCounter;
		if (time - fpsCounters.prevFpsTime >= 1000) {
			uiState.fps = fpsCounters.frameCounter;
			fpsCounters.frameCounter = 0;
			fpsCounters.prevFpsTime = time;
		}
		if (uiState.ticking) {
			const start = performance.now();
			next_step();
			const elapsed = (performance.now() - start) / 1000;
			uiState.playRuntime = elapsed;
		}
		updatePerfStats(renderer.perf_stats);
		renderer.update_viewport(
			new ViewportInfo(toRustScreenPoint([canvasDims.width, canvasDims.height]))
		);
		renderer.update_render_stats(toRustScreenPoint(uiState.cursor));
		updateRenderStats(renderer.render_stats);

		const ctx = canvas.getContext('2d')!;
		ctx.clearRect(0, 0, canvas.width, canvas.height);
		const imageData = new ImageData(
			new Uint8ClampedArray(renderer.render()),
			canvas.width,
			canvas.height
		);
		ctx.putImageData(imageData, 0, 0);

		requestAnimationFrame(repaint);
	}
	onMount(() => {
		function resizeCanvas() {
			setCanvasDims(canvas.getBoundingClientRect());
			canvas.width = canvasDims.width;
			canvas.height = canvasDims.height;
		}
		const canvasResizeObserver = new ResizeObserver(resizeCanvas);
		canvasResizeObserver.observe(canvas);
		resizeCanvas();
		setCanvasDims(canvas.getBoundingClientRect());
		requestAnimationFrame(repaint);
	});
</script>

<canvas
	bind:this={canvas}
	onpointermove={(event) => {
		setCanvasDims(canvas.getBoundingClientRect());
		gestureHandler.updateCursors(event);
		gestureHandler.handlePan(event);
		gestureHandler.handleDrawMove(event);
		gestureHandler.handleTouchZoom(event);
	}}
	onpointerdown={(event) => {
		gestureHandler.updateCursors(event);
		gestureHandler.handleDrawStart(event);
	}}
	onpointerup={(event) => {
		gestureHandler.endDrawSession(event);
	}}
	onpointercancel={(event) => {
		gestureHandler.endDrawSession(event);
	}}
	onwheel={(event) => {
		event.preventDefault();
		const mouseDelta =
			event.deltaMode == 0x00 ? event.deltaY * 0.004 : Math.sign(event.deltaY) * 0.5;
		gestureHandler.zoom(mouseDelta);
	}}
	oncontextmenu={(event) => event.preventDefault()}
	id="grid"
	class="border border-black [image-rendering:pixelated] h-full w-full touch-none min-h-0 min-w-0"
	>Game of Life grid</canvas
>
