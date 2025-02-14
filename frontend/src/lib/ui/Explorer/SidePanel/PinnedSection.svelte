<script lang="ts">
	import { explorer, NewImageViewer } from '$states';
	import Button from '$components/Button.svelte';
	import Section from './Section.svelte';
	import { defined } from '$helpers';
</script>

<Section title="PINNED">
	{#each explorer!.pinned as id}
		{@const item = explorer!.get(id)}
		{#if defined(item)}
			<Button
				onclick={() => {
					switch (item.type) {
						case 'Directory':
							explorer!.goto(item.id);
							break;
						case 'File':
							NewImageViewer(explorer!.storeId, item);
					}
				}}
				class="flex w-full items-center gap-[10px] rounded-lg text-left hover:underline"
			>
				{item?.name}
			</Button>
		{/if}
	{/each}
</Section>
