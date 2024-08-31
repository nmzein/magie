<script lang="ts">
	import type { Snippet } from 'svelte';
	import { getPagesState } from './context.svelte.ts';
	import Button from '$components/Button.svelte';

	let { children }: { children: Snippet } = $props();

	let pState = getPagesState();
</script>

<Button variant="primary" hidden={pState.firstPage} onclick={() => pState.back()}>
	{@render children()}
</Button>
