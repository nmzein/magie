<script lang="ts">
	import type { Snippet } from 'svelte';
	import { context } from './context.svelte.ts';
	import Button from '$components/Button.svelte';

	let {
		onclick,
		disabled = false,
		hidden = false,
		children
	}: { onclick?: () => void; disabled?: boolean; hidden?: boolean; children: Snippet } = $props();

	const ctx = context.get();
</script>

<Button
	onclick={() => {
		ctx.close();
		onclick?.();
	}}
	class={ctx.classes.item}
	{disabled}
	{hidden}
>
	{@render children()}
</Button>
