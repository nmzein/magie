<script lang="ts">
	import Button from '$components/Button.svelte';
	import Section from './Section.svelte';
	import { registry } from '$states';
	import { context } from '../context.svelte.ts';

	const explorer = context.get();
</script>

<Section title="stores">
	{#each registry.stores as store}
		<Button
			onclick={() => explorer.gotoStore(store.id)}
			class="flex w-full items-center gap-2.5 rounded-lg py-0.5 text-left hover:underline"
		>
			{store.name}
		</Button>
	{/each}
</Section>
