<script lang="ts">
	import { onMount } from 'svelte';
	import { image, metadata, annotations } from '$lib/stores';
	import AnnotationCanvas from '$lib/components/view/AnnotationCanvas.svelte';
	import ImageCanvas from '$lib/components/view/ImageCanvas.svelte';

	let isDragging = false;
	let mouseStartX: number;
	let mouseStartY: number;
	let offsetX = 0;
	let offsetY = 0;
	let scale = 1;
	let x = 0;
	let y = 0;
	let container: DOMRect | undefined;

	onMount(() => {
		document.addEventListener('mousemove', handleMouseMove);
		document.addEventListener('mouseup', handleMouseUp);
		document.addEventListener('wheel', handleWheel);

		container = document.getElementById('image-grid-layer-0')?.getBoundingClientRect();

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
		if (!isDragging) {
			const imageWidth = $metadata?.[0].width;
			const imageHeight = $metadata?.[0].height;
			if (!imageWidth || !imageHeight) {
				return;
			}

			const containerWidth = container?.width;
			const containerHeight = container?.height;
			if (!containerWidth || !containerHeight) {
				return;
			}

			x = Math.floor((event.clientX - offsetX) * (imageWidth / containerWidth));
			y = Math.floor((event.clientY - offsetY) * (imageHeight / containerHeight));

			return;
		}

		event.preventDefault();

		const delta_x = event.clientX - mouseStartX;
		const delta_y = event.clientY - mouseStartY;

		offsetX += delta_x;
		offsetY += delta_y;

		mouseStartX = event.clientX;
		mouseStartY = event.clientY;
	}

	function handleWheel(event: WheelEvent) {
		let newScale = scale * Math.exp(event.deltaY * -0.005);

		// Limit the scale factor within a reasonable range.
		if (newScale < 0.1) {
			newScale = 0.1;
		} else if (newScale > 50) {
			newScale = 50;
		}

		let ratio = 1 - newScale / scale;

		offsetX += (event.clientX - offsetX) * ratio;
		offsetY += (event.clientY - offsetY) * ratio;

		scale = newScale;
	}
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
	id="view"
	on:mousedown={handleMouseDown}
	style="height: 100vh; cursor: {isDragging ? 'grab' : 'crosshair'};"
>
	<div
		id="container"
		style="transform: translate({offsetX}px, {offsetY}px) scale({scale}); {isDragging
			? ''
			: 'transition: transform 0.2s;'}"
	>
		{#if $metadata && $image && $image.length > 0}
			{#if $annotations}
				<AnnotationCanvas />
			{/if}
			<ImageCanvas />
		{/if}
	</div>
	<div id="coordinates-panel" class="panel">
		<p><b>x:</b> {x}, <b>y:</b> {y}</p>
	</div>
</div>

<style lang="scss">
	#container {
		height: auto;
		transform-origin: 0 0;
	}

	#coordinates-panel {
		position: absolute;
		bottom: 10px;
		left: 10px;
		font-family: 'JetBrains Mono', monospace;
		padding: 3px 7px;

		p {
			margin: 0;
		}
	}
</style>
