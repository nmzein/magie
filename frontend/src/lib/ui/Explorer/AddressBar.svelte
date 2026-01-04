<script lang="ts">
	import Button from '$components/Button.svelte';
	import { context } from './context.svelte.ts';

	const explorer = context.get();
</script>

<div
	class="bg-primary/15 flex h-8.75 min-w-50 flex-6 flex-row items-center gap-1.25 overflow-x-auto overflow-y-hidden rounded-lg p-1.25 text-sm select-none"
>
	{#if explorer.path}
		{#each explorer.path as [segment, id], index}
			<Button
				onclick={() => explorer.goto(id)}
				class="rounded-md px-1.25 py-0.75 whitespace-nowrap hover:bg-black/25 focus:outline-2 focus:outline-offset-2"
			>
				{segment}
			</Button>
			{#if index < explorer.path.length - 1}
				<span class="text-secondary">/</span>
			{/if}
		{/each}
	{/if}
</div>
