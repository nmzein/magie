<script lang="ts">
	let {
		layerIndex,
		layer,
		display
	}: {
		layerIndex: number;
		layer: ImageLayer;
		display: boolean;
	} = $props();

	import { websocket, loadedImage, metadata } from '$states';
	import type { ImageLayer } from '$types';

	const options = {
		rootMargin: '150px'
	};

	function callback(entries: IntersectionObserverEntry[], observer: IntersectionObserver) {
		entries.forEach((entry) => {
			if (loadedImage.value === undefined || !entry.isIntersecting) return;

			let levelString = (entry.target as HTMLElement).dataset.level;
			let xString = (entry.target as HTMLElement).dataset.x;
			let yString = (entry.target as HTMLElement).dataset.y;
			if (!levelString || !xString || !yString) return;

			let level = parseInt(levelString);
			let x = parseInt(xString);
			let y = parseInt(yString);

			const ready = websocket.GetTile({
				id: loadedImage.value.id,
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

{#if metadata.value != undefined}
	<div
		id="image-layer-{layerIndex}"
		class="image-layer"
		style="--no-of-columns: {metadata.value[layerIndex].cols}; z-index: {metadata.value.length -
			layerIndex};"
	>
		{#each layer as row, rowIndex (rowIndex)}
			{#each row as tile, tileIndex (tileIndex)}
				<!--
					We want this to always be mounted, however, it should only
					show to the IntersectionObserver (and the user) in two cases.
					
					1) This layer is the current layer (i.e. display == true).
					2) The tile has been loaded (i.e. tile.src != '').
				-->
				<img
					src={tile.src || '/placeholder.png'}
					style="display: {display || tile.src != '' ? 'block' : 'none'};"
					data-level={layerIndex}
					data-x={tileIndex}
					data-y={rowIndex}
					alt="Tile ({tileIndex}, {rowIndex})"
					onerror={() => console.error('Tile Load Error <' + rowIndex + ', ' + tileIndex + '>')}
				/>

				<!-- 
					In the case where the tile has not been loaded and this layer
					is not the current layer, we want to show a placeholder image
					or else the loaded tiles would not be in the correct position.	
					This tile should never be observable by the IntersectionObserver.	
				 -->
				{#if tile.src == '' && !display}
					<img src="/placeholder.png" alt="" />
				{/if}
			{/each}
		{/each}
	</div>
{/if}

<style lang="scss">
	.image-layer {
		width: 100vw;
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
		margin: 0;
		padding: 0;
	}
</style>
