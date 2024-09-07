<script lang="ts">
	import type { Snippet } from 'svelte';
	import { getPagesState } from './context.svelte.ts';
	import Button from '$components/Button.svelte';

	let { children }: { children: Snippet } = $props();

	let pState = getPagesState();
</script>

<Button variant="primary" invisible={pState.firstPage} onclick={() => pState.back()}>
	{@render children()}
</Button>
