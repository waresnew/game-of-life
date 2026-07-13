<script lang="ts">
	import { uiState, next_step } from '$lib/shared.svelte';
	import { renderer } from '$lib/wasm';
	function changeStepExp(delta: number) {
		uiState.stepExp = Math.max(0, uiState.stepExp + delta);
		renderer.set_step_exp(uiState.stepExp);
	}
	function play() {
		uiState.ticking = !uiState.ticking;
	}
	let onceRuntime = $state('');
	function runOnce() {
		uiState.ticking = false;
		const start = performance.now();
		next_step();
		const elapsed = (performance.now() - start) / 1000;
		onceRuntime = `Took ${elapsed}s`;
	}
</script>

<div class="flex flex-col gap-2">
	<div class="flex gap-2" id="stepsize">
		<button onclick={() => changeStepExp(-1)} id="stepsize-less">-</button>
		<span id="stepsize-display">Step size: 2^{uiState.stepExp}</span>
		<button onclick={() => changeStepExp(1)} id="stepsize-more">+</button>
	</div>
	<div class="flex gap-2" id="once">
		<button onclick={runOnce} id="once-button">Run once</button>
		<span class="min-w-0 flex-1 truncate" id="once-runtime">{onceRuntime}</span>
	</div>
	<div class="flex gap-2" id="play">
		<button onclick={play} id="play-button">{uiState.ticking ? 'Stop' : 'Play'}</button>
		<span class="min-w-0 flex-1 truncate" id="play-runtime"
			>{uiState.playRuntime != -1 ? `Took ${uiState.playRuntime}s` : ''}</span
		>
	</div>
</div>
