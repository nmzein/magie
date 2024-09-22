<script lang="ts">
	import { defined } from '$helpers';
	import { explorer, repository } from '$states';
	import Button from '$components/Button.svelte';
	import Section from './Section.svelte';
</script>

<Section title="STORES">
	{#if defined(repository.registry)}
		{#each repository.registry.subdirectories as item}
			<!-- If not the Bin. -->
			{#if item.id !== 1}
				<Button
					onclick={() => explorer.routeTo([item.id])}
					class="hover:bg-primary/15 flex w-full items-center gap-[10px] rounded-lg px-[10px] py-[7.5px] text-left"
				>
					{item.name}
				</Button>
			{/if}
		{/each}
	{/if}
</Section>
