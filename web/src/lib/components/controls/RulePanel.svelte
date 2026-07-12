<script lang="ts">
	import { renderer } from '$lib/wasm';
	import { getRuleB, getRuleS } from '$lib/shared.svelte';

	let formattedB = $derived(getRuleB().join(''));
	let formattedS = $derived(getRuleS().join(''));
	let succeeded = $state(false);
	const regex = /^[0-8]{0,8}$/;
	function handleRuleInputChange() {
		if (regex.test(formattedB) && regex.test(formattedS)) {
			handleRuleChange(
				formattedB.split('').map((x) => parseInt(x)),
				formattedS.split('').map((x) => parseInt(x))
			);
		}
	}
	let ruleChangedOnce = false;
	$effect(() => {
		handleRuleChange(getRuleB(), getRuleS());
	});
	function handleRuleChange(b: number[], s: number[]) {
		if (b.length < 9 && s.length < 9) {
			renderer.set_rules(new Uint32Array(b), new Uint32Array(s));
			if (!ruleChangedOnce) {
				ruleChangedOnce = true;
			} else {
				//TODO: flash the b and s textboxes independently
				succeeded = true;
				setTimeout(() => {
					succeeded = false;
				}, 200);
			}
		}
	}
</script>

<div>
	<span>Rule:</span>
	<label
		class="cursor-help"
		title="Condition for a cell to be born (# of alive neighbours)"
		for="rule-b">B</label
	>
	<input
		onchange={handleRuleInputChange}
		class={['max-w-8 invalid:border-red-400', { 'border-green-400': succeeded }]}
		name="rule-b"
		id="rule-b-input"
		maxlength="9"
		bind:value={formattedB}
		autocomplete="off"
		pattern="[0-8]*"
	/>
	<span>/</span>
	<label
		class="cursor-help"
		title="Condition for a cell to survive (# of alive neighbours)"
		for="rule-s">S</label
	>
	<input
		onchange={handleRuleInputChange}
		class={['max-w-8 invalid:border-red-400', { 'border-green-400': succeeded }]}
		name="rule-s"
		id="rule-s-input"
		maxlength="9"
		bind:value={formattedS}
		autocomplete="off"
		pattern="[0-8]*"
	/>
</div>
