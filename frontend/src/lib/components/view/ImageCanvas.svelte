<script lang="ts">
	import { GetImageSelection } from '$lib/api';
	import { image_name, image, metadata } from '$lib/stores';
	import { onMount } from 'svelte';

	onMount(() => {
		let callback = (entries: IntersectionObserverEntry[], observer: IntersectionObserver) => {
			entries.forEach((entry) => {
				if (entry.isIntersecting) {
					let levelString = (entry.target as HTMLElement).dataset.level;
					let xString = (entry.target as HTMLElement).dataset.x;
					let yString = (entry.target as HTMLElement).dataset.y;

					if (!$image_name || !levelString || !xString || !yString) {
						return;
					}

					let level = parseInt(levelString);
					let x = parseInt(xString);
					let y = parseInt(yString);

					GetImageSelection({
						image_name: $image_name,
						level,
						start: {
							x,
							y
						},
						end: {
							x: x + 1,
							y: y + 1
						}
					});

					observer.unobserve(entry.target);
				}
			});
		};

		let observer = new IntersectionObserver(callback, {
			rootMargin: '350px'
		});

		document.querySelectorAll('.tile').forEach((tile) => {
			observer.observe(tile);
		});
	});
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
					<img
						class="tile"
						src={tile.src || '/placeholder.png'}
						data-level={layerIndex}
						data-x={tileIndex}
						data-y={rowIndex}
						alt="Tile ({tileIndex}, {rowIndex})"
						on:error={() => console.error('Tile Load Error <' + rowIndex + ', ' + tileIndex + '>')}
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
