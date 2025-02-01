<script lang="ts">
	import type { Snippet } from 'svelte';
	import { context } from './context.svelte';
	import Button from '$components/Button.svelte';

	let { children }: { children: Snippet } = $props();

	const ctx = context.get();
</script>

<Button class={ctx.classes.trigger} onclick={() => ctx.toggle()}>
	{@render children()}
</Button>
