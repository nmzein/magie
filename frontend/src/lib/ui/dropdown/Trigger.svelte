<script lang="ts">
	import type { Snippet } from 'svelte';
	let {
		showContent = $bindable(),
		class: className,
		children
	}: { showContent: boolean; class: string; children: Snippet } = $props();
</script>

<button
	class={className}
	onclick={() => {
		showContent = !showContent;
	}}
>
	{@render children()}
</button>
