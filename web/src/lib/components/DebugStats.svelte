<script lang="ts">
	import { getStats } from '$lib/shared.svelte';
</script>

<div class="flex flex-col gap-1">
	<span class="min-w-0 flex-1 truncate">Cache hit rate: {getStats().cache_hit_rate}%</span>
	<span class="min-w-0 flex-1 truncate">Pool memory: {getStats().pool_mem} MB</span>
</div>
