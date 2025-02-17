<script lang="ts">
	import { NewImageViewer } from '$states';
	import Button from '$components/Button.svelte';
	import Section from './Section.svelte';
	import { defined } from '$helpers';
	import { context } from '../context.svelte.ts';

	const explorer = context.get();
</script>

<Section title="pinned">
	{#each explorer.pinned as id}
		{@const item = explorer.get(id)}
		{#if defined(item)}
			<Button
				onclick={() => {
					switch (item.type) {
						case 'Directory':
							explorer.goto(item.id);
							break;
						case 'File':
							NewImageViewer(explorer.storeId, item);
							break;
					}
				}}
				class="flex w-full items-center gap-[10px] rounded-lg py-0.5 text-left hover:underline"
			>
				{item?.name}
			</Button>
		{/if}
	{/each}
</Section>
