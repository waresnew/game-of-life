<script lang="ts">
	import { uiState, canvasDims, CELL_SIZE, toRustScreenPoint } from '$lib/shared.svelte';
	import { renderer } from '$lib/wasm';
	import { updateRule } from '$lib/shared.svelte';

	const patterns: Record<string, string> = import.meta.glob('$assets/patterns/*.rle', {
		query: '?raw',
		import: 'default',
		eager: true
	});
	//spec: https://conwaylife.com/wiki/Run_Length_Encoded
	function applyRlePattern(pattern: string) {
		uiState.generation = 0n;
		uiState.ticking = false;
		renderer.load_pattern(pattern);
	}
</script>

<div class="flex gap-2 flex-col" id="patterns">
	<label for="pattern-presets">Patterns:</label>
	<select
		onchange={(event) => {
			const option = (event.target as HTMLSelectElement).value;
			applyRlePattern(patterns[option]!);
		}}
		name="pattern-presets"
		id="pattern-dropdown"
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
		id="pattern-from-file"
		accept=".rle"
	/>
</div>
