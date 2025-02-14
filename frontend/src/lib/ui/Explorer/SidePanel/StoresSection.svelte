<script lang="ts">
	import { repository } from '$states';
	import Button from '$components/Button.svelte';
	import Section from './Section.svelte';
</script>

<Section title="STORES">
	<!-- TODO: Make functional again. -->
	{#each repository.storeNames as name}
		<Button class="flex w-full items-center gap-[10px] rounded-lg py-2 text-left hover:underline">
			{name}
		</Button>
	{/each}
</Section>
