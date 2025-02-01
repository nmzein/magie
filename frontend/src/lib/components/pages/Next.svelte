<script lang="ts">
	import type { Snippet } from 'svelte';
	import { context } from './context.svelte.ts';
	import Button from '$components/Button.svelte';

	let { disabled, children }: { disabled: boolean; children: Snippet } = $props();

	const ctx = context.get();
</script>

<Button
	variant="primary"
	class="text-sm"
	invisible={ctx.lastPage}
	{disabled}
	onclick={() => ctx.next()}
>
	{@render children()}
</Button>
