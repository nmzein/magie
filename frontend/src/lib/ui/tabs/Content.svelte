<script lang="ts">
	import type { Snippet } from 'svelte';
	let {
		class: className,
		value,
		disabled = false,
		currentTab,
		children
	}: {
		class: string;
		value: string;
		disabled: boolean;
		currentTab: string | undefined;
		children: Snippet;
	} = $props();
</script>

{#if !disabled && value === currentTab}
	<div class={className}>
		{@render children()}
	</div>
{/if}
