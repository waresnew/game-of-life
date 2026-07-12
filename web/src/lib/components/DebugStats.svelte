<script lang="ts">
	import { getPerfStats } from '$lib/shared.svelte';

	const totalCache = $derived(getPerfStats().cache_hits + getPerfStats().cache_misses);
</script>

<div class="flex flex-col gap-1">
	<span class="min-w-0 flex-1 truncate" id="debug-cache_hitrate"
		>Cache hit rate: {totalCache > 0 ? (getPerfStats().cache_hits * 100n) / totalCache : '0'}%</span
	>
	<span class="min-w-0 flex-1 truncate" id="debug-pool_mem"
		>Pool memory: {getPerfStats().pool_mem} MB</span
	>
</div>
