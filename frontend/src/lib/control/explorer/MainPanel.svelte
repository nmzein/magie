<!-- 
    The main panel of the explorer where the files and directories are displayed.
-->
<script lang="ts">
	import { directoryStack } from './state.svelte';
	import Item from './Item.svelte';
	import { registry } from '$stores';

	let currentDirectory = $derived.by(() => {
		if (registry.value === undefined) return;
		let index = 0;
		let directory = registry.value.subdirectories[directoryStack.value[index]];
		while (index < directoryStack.value.length - 1) {
			index += 1;
			directory = directory.subdirectories[directoryStack.value[index]];
		}
		return directory;
	});
</script>

{#if currentDirectory !== undefined}
	<div>
		{#each currentDirectory.subdirectories as subdirectory, index}
			<Item type="directory" value={subdirectory} {index} />
		{/each}
		{#each currentDirectory.files as file, index}
			<Item type="file" value={file} {index} />
		{/each}
	</div>
{/if}

<style lang="scss">
	div {
		display: grid;
		grid-template-columns: repeat(5, 1fr);
		grid-template-rows: repeat(4, 1fr);
		padding: 10px 20px;
		gap: 10px;
		z-index: 1;

		height: 400px;
	}
</style>
