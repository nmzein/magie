<script lang="ts">
	import { defined } from '$helpers';
	import { image, transformer } from '$states';
	import AnnotationLayer from '$view/AnnotationLayer.svelte';
	import ImageLayer from '$view/ImageLayer.svelte';
	import { OrthographicCamera } from 'three';

	let panStartX = $state(0);
	let panStartY = $state(0);
	let isDragging = $state(false);
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
		if (!image.initialised) return;

		event.preventDefault();
		handlePanStart(event);
	}

	function handleTouchStart(event: TouchEvent) {
		if (!image.initialised) return;

		handlePanStart(event.touches[0]);
	}

	function handlePanStart(event: MouseEvent | Touch) {
		if (!image.initialised) return;

		isDragging = true;
		panStartX = event.clientX;
		panStartY = event.clientY;
	}

	function handleMouseMove(event: MouseEvent) {
		if (!image.initialised) return;

		event.preventDefault();

		// Logic for calculating the coordinates of the mouse pointer.
		if (!isDragging && defined(image.width) && defined(image.height)) {
			// TODO: Don't do this on every mouse move.
			const currentLayer = document
				.getElementById('image-layer-' + transformer.currentLevel)
				?.getBoundingClientRect();

			if (!currentLayer) return;

			const xTemp = Math.floor(
				(event.clientX - transformer.offsetX) * (image.width / currentLayer.width)
			);
			const yTemp = Math.floor(
				(event.clientY - transformer.offsetY) * (image.height / currentLayer.height)
			);

			if (Number.isFinite(xTemp) && !isNaN(xTemp)) x = xTemp;
			if (Number.isFinite(yTemp) && !isNaN(yTemp)) y = yTemp;

			return;
		}

		handlePan(event);
	}

	function handleTouchMove(event: TouchEvent) {
		if (!image.initialised) return;

		if (!isDragging) return;
		handlePan(event.touches[0]);
	}

	function handlePan(event: MouseEvent | Touch) {
		if (!image.initialised) return;

		transformer.offsetX += event.clientX - panStartX;
		transformer.offsetY += event.clientY - panStartY;

		panStartX = event.clientX;
		panStartY = event.clientY;
	}

	function handlePanEnd() {
		if (!image.initialised) return;

		isDragging = false;
	}

	function handleWheel(event: WheelEvent) {
		if (!image.initialised) return;

		transformer.zoom(event.deltaY, event.clientX, event.clientY);
	}
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<div
	role="img"
	onmousedown={handleMouseDown}
	onmousemove={handleMouseMove}
	ontouchstart={handleTouchStart}
	class="h-screen overflow-hidden"
	style="cursor: {isDragging ? 'grab' : 'crosshair'};"
>
	<div
		class="h-auto origin-top-left"
		style="transform: translate({transformer.offsetX}px, {transformer.offsetY}px) scale({transformer.scale});
			   {isDragging ? '' : 'transition: transform 0.2s;'}"
	>
		<div id="annotation-layers">
			{#if camera !== undefined}
				{#each image.annotations as layer, layerIndex}
					<AnnotationLayer {layer} {layerIndex} {camera} />
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
		<div id="coordinates-panel" class="panel absolute bottom-[10px] left-[10px] px-[7px] py-[3px]">
			<span class="font-bold">x:</span>
			{x},
			<span class="font-bold">y:</span>
			{y}
		</div>
	{/if}
</div>

<style>
	@media (hover: none) {
		#coordinates-panel {
			/* Hide the element on touch-capable devices. */
			display: none;
		}
	}
</style>
