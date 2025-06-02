<script lang="ts">
	import type { Image2DView, Image2DLayer } from './types.ts';
	import { onMount } from 'svelte';

	type Props = {
		layer: Image2DLayer;
		layerIndex: number;
		fetch: (level: number, x: number, y: number) => Promise<boolean>;
		display: boolean;
		zIndex: number;
	};

	let { layer, layerIndex, fetch, display, zIndex }: Props = $props();

	function callback(entries: IntersectionObserverEntry[], observer: IntersectionObserver) {
		entries.forEach(async (entry) => {
			if (!entry.isIntersecting) return;

			const target = entry.target as HTMLElement;
			const level = parseInt(target.dataset.level!);
			const x = parseInt(target.dataset.x!);
			const y = parseInt(target.dataset.y!);

			const sent = await fetch(level, x, y);

			if (sent) observer.unobserve(entry.target);
		});
	}

	const observer = new IntersectionObserver(callback, { rootMargin: '150px' });

	$effect(() => {
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
	style:grid-template-columns="repeat({layer.cols}, 1fr)"
	style:grid-template-rows="repeat({layer.rows}, 1fr)"
	style:z-index={zIndex}
>
	{#each layer.tiles as row, rowIndex (rowIndex)}
		{#each row as tile, colIndex (colIndex)}
			<!--
					We want this to always be mounted, however, it should only
					show to the IntersectionObserver (and the user) in two cases.

					1) This layer is the current layer (i.e. display == true).
					2) The tile has been loaded (i.e. tile.src != '').
				-->
			<!-- svelte-ignore a11y_missing_attribute -->
			<img
				src={tile.src || 'placeholder.png'}
				style="display: {display || tile.src !== '' ? 'block' : 'none'};"
				data-level={layerIndex}
				data-x={colIndex}
				data-y={rowIndex}
				alt="Tile <{layerIndex}: {colIndex}, {rowIndex}>"
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
