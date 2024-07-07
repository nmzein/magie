<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		showContent = $bindable(),
		class: className,
		children
	}: { showContent: boolean; class: string; children: Snippet } = $props();

	let dropdown: HTMLDivElement | undefined = $state();

	function addEventListener() {
		$effect(() => {
			setTimeout(() => {
				document.addEventListener('click', handleClickOutside);
			}, 10);

			return () => {
				document.removeEventListener('click', handleClickOutside);
			};
		});
	}

	function handleClickOutside(event: MouseEvent) {
		if (showContent && dropdown && !dropdown.contains(event.target as Node)) {
			showContent = false;
		}
	}
</script>

{#if showContent}
	{addEventListener()}
	<div class={className} bind:this={dropdown}>
		{@render children()}
	</div>
{/if}
