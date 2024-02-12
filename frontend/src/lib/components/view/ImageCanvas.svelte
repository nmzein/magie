<script lang="ts">
	import { image, metadata } from '$lib/stores';
</script>

<div id="image-canvas">
	{#each $image as layer, layerIndex}
		<div
			id="image-grid-layer-{layerIndex}"
			class="image-grid"
			style="--no-of-columns: {$metadata?.[layerIndex].cols};"
		>
			{#each layer as row, rowIndex (rowIndex)}
				{#each row as tile, tileIndex (tileIndex)}
					<img src={tile.src || '/placeholder.png'} alt="Tile ({rowIndex}, {tileIndex})" />
				{/each}
			{/each}
		</div>
	{/each}
</div>

<!-- on:load={() => console.log('Image loaded')} -->
<!-- on:error={() => console.error('Image failed to load')} -->

<style lang="scss">
	.image-grid {
		display: grid;
		grid-template-columns: repeat(var(--no-of-columns), 1fr);
		position: absolute;
	}

	img {
		width: 100%;
		height: auto;
		// Prevent image selection when dragging.
		user-select: none;
		object-fit: cover;
	}
</style>
