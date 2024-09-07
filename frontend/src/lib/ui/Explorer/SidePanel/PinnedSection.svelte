<script lang="ts">
	import { explorer } from '$states';
	import Button from '$components/Button.svelte';
	import Section from './Section.svelte';
</script>

<Section title="PINNED">
	{#each explorer.pinned as item}
		<Button
			onclick={() => explorer.routeTo(item.route)}
			class="hover:bg-primary/15 flex w-full items-center gap-[10px] rounded-lg px-[10px] py-[7.5px]"
		>
			{item.data.name}
		</Button>
	{/each}
</Section>
