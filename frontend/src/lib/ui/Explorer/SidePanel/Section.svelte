<script lang="ts">
	import type { Snippet } from 'svelte';

	let { title, children }: { title: string; children: Snippet } = $props();
</script>

<div class="flex-2 flex select-none flex-col gap-[5px] overflow-y-auto">
	<span class="text-secondary select-none text-sm"> {title} </span>
	<div class="flex flex-col gap-1 overflow-y-auto pr-[10px]">
		{@render children()}
	</div>
</div>
