<script lang="ts">
	import Item from './Item.svelte';
	import { defined } from '$helpers';
	import { repository } from '$states';
</script>

<div id="cont">
	<span id="title" class="grey-text"> STORES </span>
	<div>
		{#if defined(repository.registry)}
			{#each repository.registry.subdirectories as item}
				<!-- TODO: Change completely. -->
				{#if item.id !== -1}
					<Item route={[item.id]}>
						<div class="item">
							{item.name}
						</div>
					</Item>
				{/if}
			{/each}
		{/if}
	</div>
</div>

<style lang="scss">
	#cont {
		display: flex;
		flex-direction: column;
		flex-grow: 1;
		gap: 5px;
	}

	#title {
		font-size: 14px;
		user-select: none;
	}

	.item {
		width: 100%;
		display: flex;
		align-items: center;
		justify-content: space-between;
	}
</style>
