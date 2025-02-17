<script lang="ts">
	import Button from '$components/Button.svelte';
	import Section from './Section.svelte';
	import { registry } from '$states';
	import { context } from '../context.svelte.ts';

	const explorer = context.get();
</script>

<Section title="stores">
	{#each registry.registry ?? [] as store}
		<Button
			onclick={() => explorer.gotoStore(store.id)}
			class="flex w-full items-center gap-[10px] rounded-lg py-2 text-left hover:underline"
		>
			{store.name}
		</Button>
	{/each}
</Section>
