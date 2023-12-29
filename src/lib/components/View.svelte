<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { browser } from '$app/environment';
	import type { ImageMetadata, ImageSelection } from '$lib/types';
	import { WebSocketURL } from '$lib/urls';
	import { GetMetadata } from '$lib/api';
	import { WebSocketStore, ImageStore, MetadataStore } from '$lib/stores';
	import { Stage, Layer, Shape } from 'svelte-konva';

	let socket: WebSocket;
	let images: HTMLImageElement[] = [];
	let metadata: ImageMetadata | undefined;

	let isDragging = false;
	let mouseStartX: number;
	let mouseStartY: number;
	let offsetX = 0;
	let offsetY = 0;
	let scale = 1;

	$: selection = { start: { x: 0, y: 0 }, end: { x: 2, y: 2 } } as ImageSelection;

	const UnsubscribeImageStore = ImageStore.subscribe((values) => {
		images = values;
	});

	const UnsubscribeMetadataStore = MetadataStore.subscribe((values) => {
		metadata = values;
	});

	onMount(() => {
		// Get image metadata from the server.
		GetMetadata();

		const UnsubscribeWebSocketStore = WebSocketStore.subscribe((value) => {
			socket = value as WebSocket;
		});

		// socket = new WebSocket(WebSocketURL);

		// socket.addEventListener('message', (event: MessageEvent) => {
		// 	// Assuming each message is an image binary data.
		// 	// Read binary data from the Blob.
		// 	const imageData: Blob = event.data;

		// 	// Convert Blob to HTMLImageElement.
		// 	const image = new Image();
		// 	image.src = URL.createObjectURL(imageData);

		// 	// Update the images store.
		// 	ImageStore.update((images) => [...images, image]);
		// });

		document.addEventListener('mousemove', handleMouseMove);
		document.addEventListener('mouseup', handleMouseUp);
		document.addEventListener('wheel', handleWheel);

		return () => {
			// socket.close();
			UnsubscribeWebSocketStore();
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

	function GetSelection() {
		socket.send(JSON.stringify(selection));
	}

	// Unsubscribe from stores when the component is destroyed.
	onDestroy(() => {
		UnsubscribeImageStore();
		UnsubscribeMetadataStore();
	});
</script>

<button style="height: 100px; width: 100px; z-index: 100;" on:click={() => GetSelection()} />
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
	id="container"
	on:mousedown={handleMouseDown}
	style={`transform: translate(${offsetX}px, ${offsetY}px) scale(${scale})`}
>
	<div id="annotation-canvas">
		<Stage
			config={browser
				? { width: window.innerWidth, height: window.innerHeight }
				: { width: 0, height: 0 }}
		>
			<Layer>
				<Shape
					config={{
						sceneFunc: function (context, shape) {
							context.beginPath();
							context.moveTo(20, 50);
							context.lineTo(220, 80);
							context.quadraticCurveTo(150, 100, 260, 170);
							context.closePath();

							// special Konva.js method
							context.fillStrokeShape(shape);
						},
						fill: '#00D2FF',
						stroke: 'black',
						strokeWidth: 4
					}}
				/>
			</Layer>
		</Stage>
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
