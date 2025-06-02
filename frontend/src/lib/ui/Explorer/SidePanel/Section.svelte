<script lang="ts">
	import type { Snippet } from 'svelte';

	let { title, children }: { title: string; children: Snippet } = $props();
</script>

<span class="text-secondary text-sm uppercase select-none"> {title} </span>
<div class="flex flex-2 flex-col gap-[6px] overflow-x-clip overflow-y-auto select-none">
	{@render children()}
</div>
