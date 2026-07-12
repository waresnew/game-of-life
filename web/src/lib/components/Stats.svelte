<script lang="ts">
	import { getEffectiveZoomOutExp, getPerfStats, uiState, CELL_SIZE } from '$lib/shared.svelte';
</script>

<div class="flex flex-col gap-2 w-full flex-none">
	<div class="flex gap-8">
		<span class="min-w-0 flex-1 truncate">Generation: {uiState.generation}</span>
		<span class="min-w-0 flex-1 truncate">Alive: {getPerfStats().alives}</span>
	</div>
	<div class="flex gap-8" id="stats-display">
		<span class="min-w-0 flex-1 truncate">FPS: {uiState.fps}</span>
		<span class="min-w-0 flex-1 truncate"
			>{`Cursor: (${Math.floor(uiState.worldCursor[0] / CELL_SIZE)},${Math.floor(
				uiState.worldCursor[1] / CELL_SIZE
			)})`}</span
		>
		<span class="min-w-0 flex-1 truncate">Zoom: 2^{-getEffectiveZoomOutExp()}</span>
	</div>
</div>
