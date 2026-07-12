<script lang="ts">
	import { camera, uiState, canvasDims, CELL_SIZE } from '$lib/shared.svelte';
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
		renderer.clear_grid();
		const content = pattern
			.trim()
			.split('\n')
			.map((x) => x.trim())
			.filter((x) => x.length && !x.startsWith('#'));
		const header = /^x = (\d+), y = (\d+), rule = [bB]?(\d+)\/[sS]?(\d+)/.exec(content.shift()!);
		const width = parseInt(header![1]!);
		const height = parseInt(header![2]!);
		const borns = header![3]!.split('').map((x) => parseInt(x));
		const survives = header![4]!.split('').map((x) => parseInt(x));
		updateRule(borns, survives);
		let x = Math.floor(-width / 2);
		let y = Math.floor(height / 2);
		camera.centre = [0, 0];
		camera.zoomOutExpFloat = Math.max(
			0,
			Math.ceil(
				Math.log2(
					Math.max((width * CELL_SIZE) / canvasDims.width, (height * CELL_SIZE) / canvasDims.height)
				)
			)
		);
		const lines = content.join('').split('$');
		for (const line of lines) {
			let cnt_str = '';
			for (const c of line) {
				if (c == '!') return;
				if (/\d/.test(c)) {
					cnt_str += c;
				} else {
					const cnt = cnt_str ? parseInt(cnt_str) : 1;
					if (c != 'b') {
						for (let _ = 0; _ < cnt; _++) {
							renderer.toggle_cell(BigInt(x), BigInt(y));
							x += 1;
						}
					} else {
						x += cnt;
					}
					cnt_str = '';
				}
			}
			y -= cnt_str ? parseInt(cnt_str) : 1;
			x = Math.floor(-width / 2);
		}
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
