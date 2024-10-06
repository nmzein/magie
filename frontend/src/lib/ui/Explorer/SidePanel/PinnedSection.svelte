<script lang="ts">
	import { explorer, image } from '$states';
	import Button from '$components/Button.svelte';
	import Section from './Section.svelte';
</script>

<Section title="PINNED">
	{#each explorer.pinned as item}
		<Button
			onclick={() => {
				if (item.data.type === 'directory') {
					explorer.routeTo(item.route);
				} else {
					image.load(item.data);
				}
			}}
			class="flex w-full items-center gap-[10px] rounded-lg text-left hover:underline"
		>
			{item.data.name}
		</Button>
	{/each}
</Section>
