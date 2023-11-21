<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import type { ImageMetadata, ImageSelection } from '$lib/types';
	import { webSocketURL } from '$lib/urls';
	import { sendMetadataRequest } from '$lib/api';
	import { imagesStore, metadataStore } from '$lib/stores';

	let images: HTMLImageElement[] = [];
	let metadata: ImageMetadata | undefined;
	let requests: ImageSelection[] = [];

	let isDragging = false;
	let mouseStartX: number;
	let mouseStartY: number;
	let offsetX = 0;
	let offsetY = 0;
	let scale = 1;

	// Bind stores to local vars.
	const unsubscribeImagesStore = imagesStore.subscribe((values) => {
		images = values;
	});

	const unsubscribeMetadataStore = metadataStore.subscribe((values) => {
		metadata = values;
	});

	onMount(() => {
		// Send metadata request to the server.
		sendMetadataRequest();

		const webSocket = new WebSocket(webSocketURL);

		// Handle messages received from the server
		webSocket.addEventListener('message', (event: MessageEvent) => {
			// Assuming each message is an image binary data.
			// Read binary data from the Blob.
			const imageData: Blob = event.data;

			// Convert Blob to HTMLImageElement.
			const image = new Image();
			image.src = URL.createObjectURL(imageData);

			// Update the images store.
			imagesStore.update((images) => [...images, image]);
		});

		let interval: NodeJS.Timeout;

		webSocket.addEventListener('open', () => {
			interval = setInterval(() => {
				// Iterate over all pending requests and send them to the server.
				while (requests.length > 0) {
					webSocket?.send(JSON.stringify(requests[0]));
					requests.shift();
				}
			}, 100);
		});

		document.addEventListener('mousemove', handleMouseMove);
		document.addEventListener('mouseup', handleMouseUp);
		document.addEventListener('wheel', handleWheel);

		return () => {
			webSocket?.close();
			clearInterval(interval);
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

	// Add selection request to RequestsStore.
	requests.push({ start: { x: 0, y: 0 }, end: { x: 2, y: 2 } });
	requests.push({ start: { x: 0, y: 0 }, end: { x: 5, y: 5 } });
	// requests.push({ start: { x: 0, y: 0 }, end: { x: 43, y: 34 } });
	console.log(requests);

	// Unsubscribe from stores when the component is destroyed.
	onDestroy(() => {
		unsubscribeImagesStore();
		unsubscribeMetadataStore();
	});
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
	id="container"
	on:mousedown={handleMouseDown}
	style={`transform: translate(${offsetX}px, ${offsetY}px) scale(${scale})`}
>
	<div id="image-grid" style="--no-of-columns:{metadata?.cols}">
		{#each images as image, index (image.src)}
			<img src={image.src} alt="Image {index}" class="image" />
		{/each}
	</div>
	<div id="annotation-canvas" />
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
