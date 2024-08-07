<script lang="ts">
	import { image, transformer } from '$states';
	import AnnotationLayer from '$view/AnnotationLayer.svelte';
	import ImageLayer from '$view/ImageLayer.svelte';
	import { OrthographicCamera } from 'three';

	let panStartX = $state(0);
	let panStartY = $state(0);
	let isDragging = $state(false);
	// FIX: Broken!
	let x = $state(0);
	let y = $state(0);

	let camera: OrthographicCamera | undefined = $derived.by(() => {
		if (image.width === undefined || image.height === undefined) return;
		const camera = new OrthographicCamera(0, image.width, 0, -1 * image.height, 0.1, 10);
		camera.position.z = 1;

		return camera;
	});

	// TODO: FIX
	$effect(() => {
		if (!image.initialised || image.levels === undefined || image.metadata === undefined) return;

		for (let i = 0; i < image.levels; i++) {
			if (image.metadata[i].cols <= 4 || image.metadata[i].rows <= 4) {
				transformer.maxLevel = i - 1;
				transformer.currentLevel = i - 1;
				break;
			}
		}

		// maxLevel = image.metadata.length - 1;
		// currentLevel = image.metadata.length - 1;
	});

	$effect(() => {
		document.addEventListener('touchmove', handleTouchMove);
		document.addEventListener('touchend', handlePanEnd);
		document.addEventListener('mouseup', handlePanEnd);
		document.addEventListener('wheel', handleWheel);

		return () => {
			document.removeEventListener('touchmove', handleTouchMove);
			document.removeEventListener('touchend', handlePanEnd);
			document.removeEventListener('mouseup', handlePanEnd);
			document.removeEventListener('wheel', handleWheel);
		};
	});

	function handleMouseDown(event: MouseEvent) {
		event.preventDefault();
		handlePanStart(event);
	}

	function handleTouchStart(event: TouchEvent) {
		handlePanStart(event.touches[0]);
	}

	function handlePanStart(event: MouseEvent | Touch) {
		isDragging = true;
		panStartX = event.clientX;
		panStartY = event.clientY;
	}

	function handleMouseMove(event: MouseEvent) {
		event.preventDefault();

		// Logic for calculating the coordinates of the mouse pointer.
		if (!isDragging) {
			if (image.width === undefined || image.height === undefined) return;

			// TODO: Don't do this on every mouse move.
			const currentLayer = document
				.getElementById('image-layer-' + transformer.currentLevel)
				?.getBoundingClientRect();

			if (!currentLayer) return;

			x = Math.floor((event.clientX - transformer.offsetX) * (image.width / currentLayer.width));
			y = Math.floor((event.clientY - transformer.offsetY) * (image.height / currentLayer.height));

			return;
		}

		handlePan(event);
	}

	function handleTouchMove(event: TouchEvent) {
		if (!isDragging) return;
		handlePan(event.touches[0]);
	}

	function handlePan(event: MouseEvent | Touch) {
		transformer.offsetX += event.clientX - panStartX;
		transformer.offsetY += event.clientY - panStartY;

		panStartX = event.clientX;
		panStartY = event.clientY;
	}

	function handlePanEnd() {
		isDragging = false;
	}

	function handleWheel(event: WheelEvent) {
		transformer.zoom(event.deltaY, event.clientX, event.clientY);
	}
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<div
	id="view"
	role="img"
	onmousedown={handleMouseDown}
	onmousemove={handleMouseMove}
	ontouchstart={handleTouchStart}
	style="cursor: {isDragging ? 'grab' : 'crosshair'};"
>
	<div
		id="container"
		style="transform: translate({transformer.offsetX}px, {transformer.offsetY}px) scale({transformer.scale});
			   {isDragging ? '' : 'transition: transform 0.2s;'}"
	>
		<div id="annotation-layers">
			{#if camera !== undefined}
				{#each image.annotations as annotationLayer, layerIndex}
					<AnnotationLayer {annotationLayer} {layerIndex} {camera} />
				{/each}
			{/if}
		</div>
		<div id="image-layers">
			{#if transformer.currentLevel !== undefined}
				{#each image.tiles as layer, layerIndex}
					<ImageLayer {layer} {layerIndex} display={layerIndex === transformer.currentLevel} />
				{/each}
			{/if}
		</div>
	</div>
	{#if image.initialised}
		<div id="coordinates-panel" class="panel">
			<span>x:</span>
			{x},
			<span>y:</span>
			{y}
		</div>
	{/if}
</div>

<style lang="scss">
	#view {
		height: 100vh;
	}

	#container {
		height: auto;
		transform-origin: 0 0;
	}

	#coordinates-panel {
		position: absolute;
		bottom: 10px;
		left: 10px;
		padding: 3px 7px;

		span {
			font-weight: bold;
		}
	}

	@media (hover: none) {
		#coordinates-panel {
			// Hide the element on touch-capable devices.
			display: none;
		}
	}
</style>
