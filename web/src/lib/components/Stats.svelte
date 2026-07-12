<script lang="ts">
	import { getPerfStats, uiState, CELL_SIZE, getRenderStats } from '$lib/shared.svelte';
</script>

<div class="flex flex-col gap-2 w-full flex-none">
	<div class="flex gap-8">
		<span class="min-w-0 flex-1 truncate">Generation: {uiState.generation}</span>
		<span class="min-w-0 flex-1 truncate">Alive: {getPerfStats().alives}</span>
	</div>
	<div class="flex gap-8" id="stats-display">
		<span class="min-w-0 flex-1 truncate">FPS: {uiState.fps}</span>
		<span class="min-w-0 flex-1 truncate"
			>Cursor: ({getRenderStats().cell_cursor.x}, {getRenderStats().cell_cursor.y})</span
		>
		<span class="min-w-0 flex-1 truncate">Zoom: 2^{-getRenderStats().zoom_out_exp}</span>
	</div>
</div>
