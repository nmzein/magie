<script lang="ts">
	import type { Snippet } from 'svelte';
	import { getDropdownState } from './context.svelte';

	let { children }: { children: Snippet } = $props();

	let dState = getDropdownState();
</script>

<button class={dState.classes.trigger} onclick={() => dState.toggle()}>
	{@render children()}
</button>
