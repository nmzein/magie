<script lang="ts">
	import Button from '$components/Button.svelte';
	import { context } from './context.svelte.ts';

	const explorer = context.get();
</script>

<div
	class="bg-primary/15 flex h-[35px] min-w-50 flex-6 flex-row items-center gap-[5px] overflow-x-auto overflow-y-hidden rounded-lg p-[5px] text-sm select-none"
>
	{#if explorer.path}
		{#each explorer.path as [segment, id], index}
			<Button
				onclick={() => explorer.goto(id)}
				class="rounded-md px-[5px] py-[3px] whitespace-nowrap hover:bg-black/25"
			>
				{segment}
			</Button>
			{#if index < explorer.path.length - 1}
				<span class="text-secondary">/</span>
			{/if}
		{/each}
	{/if}
</div>
