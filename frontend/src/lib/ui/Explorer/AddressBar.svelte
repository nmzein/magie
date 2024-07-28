<script lang="ts">
	import { explorer } from '$states';
</script>

<div class="flex-row light-layer">
	{#if explorer.currentDirectory !== undefined}
		{#each explorer.currentDirectory.path as path, index}
			<button onclick={() => explorer.up(index)} class="path-item">{path}</button>
			{#if index < explorer.currentDirectory.path.length - 1}
				<span class="grey-text">/</span>
			{/if}
		{/each}
	{/if}
</div>

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
