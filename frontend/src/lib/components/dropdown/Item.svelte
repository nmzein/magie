<script lang="ts">
	import type { Snippet } from 'svelte';
	import { getDropdownState } from './context.svelte';
	import Button from '$components/Button.svelte';

	let {
		onclick,
		disabled = false,
		children
	}: { onclick?: () => void; disabled?: boolean; children: Snippet } = $props();

	let dState = getDropdownState();
</script>

<Button
	onclick={() => {
		dState.close();
		onclick?.();
	}}
	class={dState.classes.item}
	{disabled}
>
	{@render children()}
</Button>
