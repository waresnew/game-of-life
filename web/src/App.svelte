<script>
	import Canvas from '$lib/components/Canvas.svelte';
	import ControlPanel from '$lib/components/ControlPanel.svelte';
	import Instructions from '$lib/components/Instructions.svelte';
	import Stats from '$lib/components/Stats.svelte';
</script>

<div class="flex gap-4 items-center-safe flex-wrap p-(--page-padding)">
	<div class="flex flex-col gap-2 flex-1 h-[calc(100dvh-2*var(--page-padding))] w-full min-w-100">
		<Stats />
		<Canvas />
		<Instructions />
	</div>
	<ControlPanel />
</div>
