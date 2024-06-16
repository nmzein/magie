<!-- 
	Displays the path to the current directory the user is in.
-->
<script lang="ts">
	import { directoryStack } from './state.svelte';
	import { registry } from '$states';

	let currentDirectoryPath = $derived.by(() => {
		if (registry.value === undefined) return;

		let path = [];
		let currentDirectory = registry.value; // Initial root node.

		for (let index of directoryStack.value) {
			currentDirectory = currentDirectory.subdirectories[index];
			path.push(currentDirectory.name);
		}
		return path;
	});
</script>

{#if currentDirectoryPath !== undefined}
	<div class="flex-row light-layer">
		{#each currentDirectoryPath as path, index}
			<button class="path-item">{path}</button>
			{#if index < currentDirectoryPath.length - 1}
				<span class="grey-text">/</span>
			{/if}
		{/each}
	</div>
{/if}

<style lang="scss">
	div {
		flex: 6;
		padding: 5px;
		height: 25px;
		overflow-x: auto;
		border-radius: var(--border-radius);

		align-items: center;
		gap: 5px;
	}

	.path-item {
		backdrop-filter: blur(45px);

		padding: 3px 5px;
		border-radius: 5px;

		white-space: nowrap;

		&:hover {
			background-color: rgba(0, 0, 0, 0.3);
		}
	}
</style>
