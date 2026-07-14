<script lang="ts">
	import { getPerfStats, uiState, getRenderStats } from '$lib/shared.svelte';
	function absBigInt(n: bigint) {
		return n < 0n ? -n : n;
	}
	function formatBigInt(n: bigint) {
		const sciNotationThreshold = 9_999_999_999n;
		if (absBigInt(n) > sciNotationThreshold) {
			return n.toLocaleString('en-US', { notation: 'scientific', maximumFractionDigits: 3 });
		} else {
			return n.toString();
		}
	}
</script>

<div class="flex flex-col gap-2 w-full flex-none">
	<div class="flex gap-8">
		<span class="min-w-0 flex-1 truncate">Generation: {formatBigInt(uiState.generation)}</span>
		<span class="min-w-0 flex-1 truncate">Alive: {formatBigInt(BigInt(getPerfStats().alives))}</span
		>
	</div>
	<div class="flex gap-8" id="stats-display">
		<span class="min-w-0 flex-1 truncate">FPS: {uiState.fps}</span>
		<span class="min-w-0 flex-1 truncate"
			>Cursor: ({formatBigInt(BigInt(getRenderStats().cell_cursor_x))}, {formatBigInt(
				BigInt(getRenderStats().cell_cursor_y)
			)})</span
		>
		<span class="min-w-0 flex-1 truncate">Zoom: 2^{-getRenderStats().zoom_out_exp}</span>
	</div>
</div>
