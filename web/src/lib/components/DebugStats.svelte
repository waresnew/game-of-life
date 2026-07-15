<script lang="ts">
	import { getPerfStats } from '$lib/shared.svelte';

	const totalCache = $derived(getPerfStats().cache_hits + getPerfStats().cache_misses);
</script>

<div class="flex flex-col gap-1">
	<span class="min-w-0 flex-1 truncate"
		>Cache hit rate: {totalCache > 0 ? (getPerfStats().cache_hits * 100n) / totalCache : '0'}%</span
	>
	<span class="min-w-0 flex-1 truncate">Pool memory: {getPerfStats().pool_mem} MB</span>
	<span class="min-w-0 flex-1 truncate">Quadtree height: {getPerfStats().height}</span>
</div>
