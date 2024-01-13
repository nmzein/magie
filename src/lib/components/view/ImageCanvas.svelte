<script lang="ts">
	import { image, metadata } from '$lib/stores';
</script>

<div id="image-canvas">
	{#each $image as layer, layer_index}
		<div
			id="image-grid-layer-{layer_index}"
			class="image-grid"
			style="--no-of-columns: {$metadata?.cols};"
		>
			{#each layer as row, row_index (row_index)}
				{#each row as tile, tile_index (tile_index)}
					<img
						src={tile.src || '/placeholder.png'}
						alt="Tile ({row_index}, {tile_index})"
						on:load={() => console.log('Image loaded')}
						on:error={() => console.error('Image failed to load')}
					/>
				{/each}
			{/each}
		</div>
	{/each}
</div>

<style lang="scss">
	.image-grid {
		display: grid;
		grid-template-columns: repeat(var(--no-of-columns), 1fr);
	}

	img {
		width: 100%;
		height: auto;
		// Prevent image selection when dragging.
		user-select: none;
		object-fit: cover;
	}
</style>
