<script lang="ts">
	import { images } from '$states';
	import type { ImageLayer } from '$types';
	import { onMount } from 'svelte';

	let {
		layerIndex,
		layer,
		display
	}: {
		layerIndex: number;
		layer: ImageLayer;
		display: boolean;
	} = $props();

	function callback(entries: IntersectionObserverEntry[], observer: IntersectionObserver) {
		entries.forEach(async (entry) => {
			if (!entry.isIntersecting || !images[0]?.initialised) return;

			const target = entry.target as HTMLElement;
			const level = parseInt(target.dataset.level!);
			const x = parseInt(target.dataset.x!);
			const y = parseInt(target.dataset.y!);

			images[0].getTile({
				store_id: images[0].storeId,
				image_id: images[0].info.id,
				level,
				x,
				y
			});

			observer.unobserve(entry.target);
		});
	}

	const options = { rootMargin: '150px' };

	let observer = new IntersectionObserver(callback, options);

	onMount(() => {
		document.querySelectorAll('[data-level="' + layerIndex + '"]').forEach((tile) => {
			observer.observe(tile);
		});

		return () => {
			observer.disconnect();
		};
	});
</script>

<div
	id="image-layer-{layerIndex}"
	class="absolute grid w-full"
	style:grid-template-columns="repeat({images[0].properties.metadata[layerIndex].cols}, 1fr)"
	style:grid-template-rows="repeat({images[0].properties.metadata[layerIndex].rows}, 1fr)"
	style:z-index={images[0].levels - layerIndex}
>
	{#each layer as row, rowIndex (rowIndex)}
		{#each row as tile, colIndex (colIndex)}
			<!--
					We want this to always be mounted, however, it should only
					show to the IntersectionObserver (and the user) in two cases.

					1) This layer is the current layer (i.e. display == true).
					2) The tile has been loaded (i.e. tile.src != '').
				-->
			<img
				src={tile.src || 'placeholder.png'}
				style="display: {display || tile.src !== '' ? 'block' : 'none'};"
				data-level={layerIndex}
				data-x={colIndex}
				data-y={rowIndex}
				alt="Tile ({layerIndex}: {colIndex}, {rowIndex})"
				onerror={() => console.error(`Tile Load Error <${layerIndex}: ${colIndex}, ${rowIndex}>`)}
			/>

			<!--
					In the case where the tile has not been loaded and this layer
					is not the current layer, we want to show a placeholder image
					or else the loaded tiles would not be in the correct position.
					This tile should never be observable by the IntersectionObserver.
				 -->
			{#if tile.src === '' && !display}
				<img src="placeholder.png" alt="" />
			{/if}
		{/each}
	{/each}
</div>

<style>
	img {
		width: 100%;
		height: auto;
		/* Prevent image selection when dragging. */
		user-select: none;
		object-fit: cover;
		margin: 0;
		padding: 0;
	}
</style>
