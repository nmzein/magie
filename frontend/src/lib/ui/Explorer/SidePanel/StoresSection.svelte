<script lang="ts">
	import { explorer, repository } from '$states';
	import Button from '$components/Button.svelte';
	import Section from './Section.svelte';
</script>

<Section title="STORES">
	{#each repository.registry!.subdirectories as item}
		<!-- If not the Bin. -->
		{#if item.id !== 1}
			<Button
				onclick={() => explorer!.routeTo([item.id])}
				class="flex w-full items-center gap-[10px] rounded-lg py-2 text-left hover:underline"
			>
				{item.name}
			</Button>
		{/if}
	{/each}
</Section>
