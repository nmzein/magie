<script lang="ts">
	import type { Snippet } from 'svelte';
	import { getDropdownState } from './context.svelte';
	import Button from '$components/Button.svelte';

	let {
		onclick,
		disabled = false,
		hidden = false,
		children
	}: { onclick?: () => void; disabled?: boolean; hidden?: boolean; children: Snippet } = $props();

	let dState = getDropdownState();
</script>

<Button
	onclick={() => {
		dState.close();
		onclick?.();
	}}
	class={dState.classes.item}
	{disabled}
	{hidden}
>
	{@render children()}
</Button>
