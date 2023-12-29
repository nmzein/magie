<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import type { ImageMetadata } from '$lib/types';
	import { GetMetadata } from '$lib/api';
	import { ImageStore, MetadataStore } from '$lib/stores';
	import Annotation from './Annotation.svelte';

	let images: HTMLImageElement[] = [];
	let metadata: ImageMetadata | undefined;

	let isDragging = false;
	let mouseStartX: number;
	let mouseStartY: number;
	let offsetX = 0;
	let offsetY = 0;
	let scale = 1;

	const UnsubscribeImageStore = ImageStore.subscribe((values) => {
		images = values;
	});

	const UnsubscribeMetadataStore = MetadataStore.subscribe((values) => {
		metadata = values;
	});

	onMount(() => {
		// Get image metadata from the server.
		GetMetadata();

		document.addEventListener('mousemove', handleMouseMove);
		document.addEventListener('mouseup', handleMouseUp);
		document.addEventListener('wheel', handleWheel);

		return () => {
			document.removeEventListener('mousemove', handleMouseMove);
			document.removeEventListener('mouseup', handleMouseUp);
			document.removeEventListener('wheel', handleWheel);
		};
	});

	function handleMouseDown(event: MouseEvent) {
		event.preventDefault();
		isDragging = true;
		mouseStartX = event.clientX;
		mouseStartY = event.clientY;
	}

	function handleMouseUp() {
		isDragging = false;
	}

	function handleMouseMove(event: MouseEvent) {
		if (isDragging) {
			event.preventDefault();

			const deltaX = event.clientX - mouseStartX;
			const deltaY = event.clientY - mouseStartY;

			offsetX += deltaX;
			offsetY += deltaY;

			mouseStartX = event.clientX;
			mouseStartY = event.clientY;
		}
	}

	// Listen for the wheel event to handle zoom.
	function handleWheel(event: WheelEvent) {
		event.preventDefault();

		// Get the container.
		let container = document.getElementById('container');
		if (!container) return;

		// Calculate the new scale factor based on the wheel delta.
		const newScale = scale + event.deltaY * -0.005;

		// Limit the scale factor within a reasonable range.
		scale = Math.min(Math.max(newScale, 0.5), 30);

		container.style.transition = 'transform 0.5s ease-out';
		container.style.transform = `translate(${offsetX}px, ${offsetY}px) scale(${scale})`;
	}

	onDestroy(() => {
		UnsubscribeImageStore();
		UnsubscribeMetadataStore();
	});
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
	id="container"
	on:mousedown={handleMouseDown}
	style={`transform: translate(${offsetX}px, ${offsetY}px) scale(${scale})`}
>
	<div id="annotation-canvas">
		<Annotation />
	</div>
	<div id="image-grid" style="--no-of-columns:{metadata?.cols}">
		{#each images as image, index (image.src)}
			<img src={image.src} alt="Image {index}" class="image" />
		{/each}
	</div>
</div>

<style>
	#container {
		cursor: grab;
		height: auto;
	}

	#image-grid {
		display: grid;
		grid-template-columns: repeat(var(--no-of-columns), 1fr);
	}

	#annotation-canvas {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
	}

	.image {
		width: 100%;
		height: auto;
		/* Prevent image selection when dragging */
		user-select: none;
		object-fit: cover;
	}
</style>
