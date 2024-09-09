<script lang="ts">
	import { defined } from '$helpers';
	import { image } from '$states';
	import type { ImageLayer } from '$types';

	let {
		layerIndex,
		layer,
		display
	}: {
		layerIndex: number;
		layer: ImageLayer;
		display: boolean;
	} = $props();

	const options = {
		rootMargin: '150px'
	};

	function callback(entries: IntersectionObserverEntry[], observer: IntersectionObserver) {
		entries.forEach((entry) => {
			if (!image.initialised || image.info === undefined || !entry.isIntersecting) return;

			let target = entry.target as HTMLElement;
			let levelString = target.dataset.level;
			let xString = target.dataset.x;
			let yString = target.dataset.y;
			if (levelString === undefined || xString === undefined || yString === undefined) return;

			let level = parseInt(levelString);
			let x = parseInt(xString);
			let y = parseInt(yString);

			const ready = image.getTile({
				id: image.info.id,
				level,
				x,
				y
			});
			if (!ready) return;

			observer.unobserve(entry.target);
			console.log('Unobserving', level);
		});
	}

	let observer = new IntersectionObserver(callback, options);

	$effect(() => {
		document.querySelectorAll('[data-level="' + layerIndex + '"]').forEach((tile) => {
			observer.observe(tile);
		});

		return () => {
			observer.disconnect();
		};
	});
</script>

{#if defined(image.properties)}
	<div
		id="image-layer-{layerIndex}"
		class="absolute grid w-screen"
		style:grid-template-columns={`repeat(${image.properties.metadata[layerIndex].cols}, 1fr)`}
		style:grid-template-rows={`repeat(${image.properties.metadata[layerIndex].rows}, 1fr)`}
		style:z-index={image.levels - layerIndex}
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
{/if}

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
