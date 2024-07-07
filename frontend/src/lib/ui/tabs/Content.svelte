<script lang="ts">
	import type { Snippet } from 'svelte';
	let {
		class: className,
		value,
		currentTab,
		children
	}: { class: string; value: string; currentTab: string | undefined; children: Snippet } = $props();
</script>

{#if value === currentTab}
	<div class={className}>
		{@render children()}
	</div>
{/if}
