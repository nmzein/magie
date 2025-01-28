<script lang="ts">
	import type { Snippet } from 'svelte';

	let { title, children }: { title: string; children: Snippet } = $props();
</script>

<span class="text-secondary select-none text-sm"> {title} </span>
<div class="flex-2 flex select-none flex-col gap-[6px] overflow-y-auto pr-[10px]">
	{@render children()}
</div>
