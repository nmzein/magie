<script lang="ts">
	import type { Snippet } from 'svelte';
	import { context } from './context.svelte.ts';
	import Button from '$components/Button.svelte';

	let { children }: { children: Snippet } = $props();

	const ctx = context.get();
</script>

<Button variant="accent" class="text-sm" invisible={ctx.firstPage} onclick={() => ctx.back()}>
	{@render children()}
</Button>
