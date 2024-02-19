<script lang="ts">
	import { GetImageSelection } from '$lib/api';
	import { image_name, metadata } from '$lib/stores';
	import type { ImageLayer } from '$lib/types';
	import { onMount } from 'svelte';

	export let layerIndex: number;
	export let layer: ImageLayer;
	export let display: boolean = false;

	let options = {
		rootMargin: '150px'
	};

	let callback = (entries: IntersectionObserverEntry[], observer: IntersectionObserver) => {
		entries.forEach((entry) => {
			if (!$image_name) {
				return;
			}

			if (entry.isIntersecting) {
				let levelString = (entry.target as HTMLElement).dataset.level;
				let xString = (entry.target as HTMLElement).dataset.x;
				let yString = (entry.target as HTMLElement).dataset.y;
				if (!levelString || !xString || !yString) {
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
				(entry.target as HTMLElement).dataset.level = '-1';
				console.log('Unobserving', level);
			}
		});
	};

	let observer = new IntersectionObserver(callback, options);

	onMount(() => {
		document.querySelectorAll('[data-level="' + layerIndex + '"]').forEach((tile) => {
			console.log('Observing', layerIndex);
			observer.observe(tile);
		});
	});
</script>

<div
	id="image-grid-layer-{layerIndex}"
	class="image-grid"
	style="--no-of-columns: {$metadata?.[layerIndex].cols};"
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
				on:error={() => console.error('Tile Load Error <' + rowIndex + ', ' + tileIndex + '>')}
			/>
		{/each}
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
