<script lang="ts">
	let {
		layerIndex,
		layer,
		display = false
	} = $props<{
		layerIndex: number;
		layer: ImageLayer;
		display: boolean;
	}>();

	import { websocket, loadedImage, metadata } from '$stores';
	import type { ImageLayer } from '$types';

	const options = {
		rootMargin: '150px'
	};

	function callback(entries: IntersectionObserverEntry[], observer: IntersectionObserver) {
		entries.forEach((entry) => {
			if (!loadedImage.value) return;

			if (entry.isIntersecting) {
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
			}
		});
	}

	let observer = new IntersectionObserver(callback, options);

	$effect(() => {
		document.querySelectorAll('[data-level="' + layerIndex + '"]').forEach((tile) => {
			// console.log('Observing', layerIndex);
			observer.observe(tile);
		});

		return () => {
			observer.disconnect();
		};
	});
</script>

<div
	id="image-layer-{layerIndex}"
	class="image-layer"
	style="--no-of-columns: {metadata.value?.[layerIndex].cols};"
>
	{#each layer as row, rowIndex (rowIndex)}
		{#each row as tile, tileIndex (tileIndex)}
			<img
				src={tile.src || '/placeholder.png'}
				style="display: {display ? 'block' : 'none'};"
				data-level={layerIndex}
				data-x={tileIndex}
				data-y={rowIndex}
				alt="Tile ({tileIndex}, {rowIndex})"
				onerror={() => console.error('Tile Load Error <' + rowIndex + ', ' + tileIndex + '>')}
			/>
		{/each}
	{/each}
</div>

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
