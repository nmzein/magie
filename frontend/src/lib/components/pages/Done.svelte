<script lang="ts">
	import type { Snippet } from 'svelte';
	import { context } from './context.svelte.js';
	import Button from '$components/Button.svelte';

	let { enabled, done, children }: { enabled: boolean; done: () => void; children: Snippet } =
		$props();

	const ctx = context.get();
</script>

<Button variant="primary" class="text-sm" hidden={!ctx.lastPage} disabled={!enabled} onclick={done}>
	{@render children()}
</Button>
