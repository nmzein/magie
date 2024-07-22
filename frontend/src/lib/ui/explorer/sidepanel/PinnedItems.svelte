<script lang="ts">
	import Item from './Item.svelte';
	import { explorer } from '$states';
</script>

<div id="cont">
	<span id="title" class="grey-text"> PINNED </span>
	<div id="content">
		{#each explorer.pinned as item}
			<Item route={item.route}>
				<div class="item">
					{item.data.name}
					<!-- <span class="id grey-text">{item.path.join('/')}</span> -->
				</div>
			</Item>
		{/each}
	</div>
</div>

<style lang="scss">
	#cont {
		display: flex;
		flex-direction: column;
		flex-grow: 1;
		gap: 5px;
		overflow-y: auto;
	}

	#title {
		font-size: 14px;
		user-select: none;
	}

	#content {
		display: flex;
		flex-direction: column;
		gap: 5px;
	}

	.item {
		width: 100%;
		display: flex;
		flex-direction: column;
		align-items: left;
		gap: 2px;
		justify-content: space-between;
	}

	// .id {
	// 	font-size: 10px;
	// 	height: 100%;
	// 	overflow: hidden;
	// 	text-overflow: ellipsis;
	// 	white-space: nowrap;
	// 	max-width: 125px;
	// }
</style>
