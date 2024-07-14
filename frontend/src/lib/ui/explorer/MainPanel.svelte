<script lang="ts">
	import { explorer } from '$states';
	import Item from './Item.svelte';
	import DirectoryCreator from './DirectoryCreator.svelte';
</script>

{#if explorer.currentDirectory !== undefined}
	<div>
		{#each explorer.currentDirectory.directory.subdirectories as subdirectory, index}
			<Item type="directory" value={subdirectory} {index} />
		{/each}
		{#each explorer.currentDirectory.directory.files as file, index}
			<Item type="file" value={file} {index} />
		{/each}
		{#if explorer.showDirectoryCreator}
			<DirectoryCreator />
		{/if}
	</div>
{/if}

<style lang="scss">
	div {
		display: grid;
		grid-template-columns: repeat(4, calc(25% - 7.5px));
		grid-template-rows: repeat(4, 1fr);
		padding: 10px 20px;
		gap: 10px;
		z-index: 1;

		height: 400px;
	}
</style>
