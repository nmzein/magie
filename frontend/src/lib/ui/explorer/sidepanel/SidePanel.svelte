<script lang="ts">
	import PinnedItems from './PinnedItems.svelte';
	import StoresList from './StoresList.svelte';
	import TrashBin from './TrashBin.svelte';
</script>

<div class="light-layer">
	<PinnedItems />
	<StoresList />
	<TrashBin />
</div>

<style lang="scss">
	div {
		flex: 2;
		border-radius: 0px 0px 0px 10px;
		padding: 15px;
		display: flex;
		flex-direction: column;
		border-right: 1px solid rgba(168, 168, 168, 0.2);
	}
</style>
