<script lang="ts">
	import { getRenderStats } from '$lib/shared.svelte';
	import { renderer } from '$lib/wasm';
	//TODO: cleanup div ids

	let formattedB = $state('3');
	let formattedS = $state('23');
	let succeededB = $state(false);
	let succeededS = $state(false);
	let prevB = [3];
	let prevS = [2, 3];
	const regex = /^[0-8]{0,8}$/;
	function handleRuleTextboxChange() {
		if (regex.test(formattedB) && regex.test(formattedS)) {
			const b = formattedB.split('').map((x) => parseInt(x));
			const s = formattedS.split('').map((x) => parseInt(x));
			if (b.length < 9 && s.length < 9) {
				renderer.set_rule(new Uint32Array(b), new Uint32Array(s));
			}
		}
	}
	$effect(() => {
		tryUpdateRuleTextbox(Array.from(getRenderStats().rule_b), Array.from(getRenderStats().rule_s));
	});
	function tryUpdateRuleTextbox(b: number[], s: number[]) {
		if (b.join('') != prevB.join('')) {
			formattedB = b.join('');
			succeededB = true;
			setTimeout(() => {
				succeededB = false;
			}, 200);
		}
		if (s.join('') != prevS.join('')) {
			formattedS = s.join('');
			succeededS = true;
			setTimeout(() => {
				succeededS = false;
			}, 200);
		}
		prevB = b;
		prevS = s;
	}
</script>

<div>
	<span>Rule:</span>
	<label
		class="cursor-help underline"
		title="Condition for a cell to be born (# of alive neighbours)"
		for="rule-b">B</label
	>
	<input
		onchange={handleRuleTextboxChange}
		class={['max-w-8 invalid:border-red-400', { 'border-green-400': succeededB }]}
		name="rule-b"
		id="rule-b-input"
		maxlength="9"
		bind:value={formattedB}
		autocomplete="off"
		pattern="[0-8]*"
	/>
	<span>/</span>
	<label
		class="cursor-help underline"
		title="Condition for a cell to survive (# of alive neighbours)"
		for="rule-s">S</label
	>
	<input
		onchange={handleRuleTextboxChange}
		class={['max-w-8 invalid:border-red-400', { 'border-green-400': succeededS }]}
		name="rule-s"
		id="rule-s-input"
		maxlength="9"
		bind:value={formattedS}
		autocomplete="off"
		pattern="[0-8]*"
	/>
</div>
