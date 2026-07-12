<script>
	import PatternPanel from './panels/PatternPanel.svelte';
	import RulePanel from './panels/RulePanel.svelte';
	import RunPanel from './panels/RunPanel.svelte';
	import DebugStats from './DebugStats.svelte';
</script>

<div class="flex flex-col gap-4 w-60 flex-none">
	<RulePanel />
	<PatternPanel />
	<div class="h-px w-full bg-zinc-300"></div>
	<RunPanel />
	<div class="h-px w-full bg-zinc-300"></div>
	<DebugStats />
</div>
