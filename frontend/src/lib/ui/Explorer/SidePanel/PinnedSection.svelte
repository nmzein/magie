<script lang="ts">
	import { load as loadImage2D } from '$view/Image2D/state.svelte.ts';
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
						case 'Asset':
							loadImage2D(explorer.storeId, item.parentId, item.id, item.name);
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
