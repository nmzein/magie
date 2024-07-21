<script lang="ts">
	import type { Snippet } from 'svelte';
	import { getDropdownState } from './context.svelte';

	let { onclick, children }: { onclick: () => void; children: Snippet } = $props();

	let dState = getDropdownState();
</script>

<button
	onclick={() => {
		dState.close();
		onclick();
	}}
	class={dState.classes.item}
>
	{@render children()}
</button>
