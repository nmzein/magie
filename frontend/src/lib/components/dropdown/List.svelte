<script lang="ts">
	import type { Snippet } from 'svelte';
	import { getDropdownState } from './context.svelte';

	let { children }: { children: Snippet } = $props();

	let dropdown: HTMLDivElement | undefined = $state();
	let dState = getDropdownState();

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
		if (dropdown && !dropdown.contains(event.target as Node)) {
			dState.close();
		}
	}
</script>

{#if dState.show}
	{addEventListener()}
	<div class={dState.classes.list} bind:this={dropdown}>
		{@render children()}
	</div>
{/if}
