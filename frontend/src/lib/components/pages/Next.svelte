<script lang="ts">
	import type { Snippet } from 'svelte';
	import { getPagesState } from './context.svelte.ts';
	import Button from '$components/Button.svelte';

	let { disabled, children }: { disabled: boolean; children: Snippet } = $props();

	let pState = getPagesState();
</script>

<Button variant="primary" hidden={pState.lastPage} {disabled} onclick={() => pState.next()}>
	{@render children()}
</Button>
