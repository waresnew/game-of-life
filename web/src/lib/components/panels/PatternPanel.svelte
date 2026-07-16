<script lang="ts">
	import { uiState, canvasDims, toRustScreenPoint, backend } from '$lib/shared.svelte';

	const patterns: Record<string, string> = import.meta.glob('$assets/patterns/*.rle', {
		query: '?raw',
		import: 'default',
		eager: true
	});
	//spec: https://conwaylife.com/wiki/Run_Length_Encoded
	function applyRlePattern(pattern: string) {
		uiState.generation = 0n;
		uiState.ticking = false;
		if (!pattern) {
			backend.clear_grid();
		} else {
			backend.load_pattern(pattern);
		}
	}
</script>

<div class="flex gap-2 flex-col">
	<label for="pattern-presets">Patterns:</label>
	<select
		onchange={(event) => {
			const option = (event.target as HTMLSelectElement).value;
			applyRlePattern(patterns[option]!);
		}}
		name="pattern-presets"
	>
		<option selected value="blank">Blank</option>
		{#each Object.keys(patterns) as key}
			<option value={key}>{key.slice(key.lastIndexOf('/') + 1)}</option>
		{/each}
	</select>
	<input
		onchange={async (event) => {
			const file = (event.target as HTMLInputElement).files![0];
			if (!file) return;
			applyRlePattern(await file.text());
		}}
		type="file"
		accept=".rle"
	/>
</div>
