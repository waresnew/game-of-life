<script lang="ts">
	import { uiState, next_step, renderer } from '$lib/shared.svelte';
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
	<div class="flex gap-2">
		<button onclick={() => changeStepExp(-1)}>-</button>
		<span>Step size: 2^{uiState.stepExp}</span>
		<button onclick={() => changeStepExp(1)}>+</button>
	</div>
	<div class="flex gap-2">
		<button onclick={runOnce}>Run once</button>
		<span class="min-w-0 flex-1 truncate">{onceRuntime}</span>
	</div>
	<div class="flex gap-2">
		<button onclick={play}>{uiState.ticking ? 'Stop' : 'Play'}</button>
		<span class="min-w-0 flex-1 truncate"
			>{uiState.playRuntime != -1 ? `Took ${uiState.playRuntime}s` : ''}</span
		>
	</div>
</div>
