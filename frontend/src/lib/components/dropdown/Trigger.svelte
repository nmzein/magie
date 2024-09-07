<script lang="ts">
	import type { Snippet } from 'svelte';
	import { getDropdownState } from './context.svelte';
	import Button from '$components/Button.svelte';

	let { children }: { children: Snippet } = $props();

	let dState = getDropdownState();
</script>

<Button class={dState.classes.trigger} onclick={() => dState.toggle()}>
	{@render children()}
</Button>
