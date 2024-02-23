<script lang="ts">
	import { annotations, image, metadata } from '$stores';
	import AnnotationLayer from '$view/AnnotationLayer.svelte';
	import ImageLayer from '$view/ImageLayer.svelte';

	// TODO: Change how start level is chosen.
	let currentLevel = $state(1);
	let panStartX = $state(0);
	let panStartY = $state(0);
	let isDragging = $state(false);
	let offsetX = $state(0);
	let offsetY = $state(0);
	let scale = $state(1);
	// Still some clunky behaviour.
	let x = $state(0);
	let y = $state(0);

	const minScale = 0.1;
	const maxScale = 50;
	const minLevel = 0;
	let maxLevel: number | undefined = $state();
	let imageWidth: number | undefined = $state();
	let imageHeight: number | undefined = $state();

	$effect(() => {
		document.addEventListener('mousemove', handleMouseMove);
		document.addEventListener('touchmove', handleTouchMove);
		document.addEventListener('mouseup', handlePanEnd);
		document.addEventListener('touchend', handlePanEnd);
		document.addEventListener('wheel', handleWheel);

		if (metadata.value) {
			maxLevel = metadata.value?.length - 1;
			imageWidth = metadata.value[0].width;
			imageHeight = metadata.value[0].height;
		}

		return () => {
			document.removeEventListener('mousemove', handleMouseMove);
			document.removeEventListener('touchmove', handleTouchMove);
			document.removeEventListener('mouseup', handlePanEnd);
			document.removeEventListener('touchend', handlePanEnd);
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

		if (!isDragging) {
			if (!imageWidth || !imageHeight) return;

			const currentLayer = document
				.getElementById('image-grid-layer-' + currentLevel)
				?.getBoundingClientRect();

			const currentLayerWidth = currentLayer?.width;
			const currentLayerHeight = currentLayer?.height;

			if (!currentLayerWidth || !currentLayerHeight) return;

			x = Math.floor((event.clientX - offsetX) * (imageWidth / currentLayerWidth));
			y = Math.floor((event.clientY - offsetY) * (imageHeight / currentLayerHeight));

			return;
		}

		handlePan(event);
	}

	function handleTouchMove(event: TouchEvent) {
		if (!isDragging) return;
		handlePan(event.touches[0]);
	}

	function handlePan(event: MouseEvent | Touch) {
		offsetX += event.clientX - panStartX;
		offsetY += event.clientY - panStartY;

		panStartX = event.clientX;
		panStartY = event.clientY;
	}

	function handlePanEnd() {
		isDragging = false;
	}

	function zoom(delta: number, mouseX: number | 0, mouseY: number | 0) {
		let newScale = scale * Math.exp(delta * -0.005);

		// Limit the scale factor within a reasonable range.
		if (newScale < minScale) {
			newScale = minScale;
		} else if (newScale > maxScale) {
			newScale = maxScale;
		}

		let ratio = 1 - newScale / scale;

		offsetX += (mouseX - offsetX) * ratio;
		offsetY += (mouseY - offsetY) * ratio;

		scale = newScale;

		// If at highest detail level and zooming in,
		// or if at lowest detail level and zooming out, do nothing.
		if ((currentLevel == minLevel && delta < 0) || (currentLevel == maxLevel && delta > 0)) {
			let s = delta < 0 ? 'in' : 'out';
			console.log('At level', currentLevel, 'and zooming', s + '. Skip computation.');
			return;
		}

		const currentLayerWidth = document
			.getElementById('image-grid-layer-' + currentLevel)
			?.getBoundingClientRect()?.width;

		if (!currentLayerWidth) return;

		const viewportWidth = window.innerWidth;
		const threshold = currentLayerWidth / viewportWidth;

		// If current layer width is larger than viewport width, switch to next level.
		if (threshold > 1) {
			if (currentLevel == minLevel) return;

			currentLevel -= 1;
			console.log('Switching to next level:', currentLevel + '.');
		} else if (threshold < 0.9) {
			if (currentLevel == maxLevel) return;

			currentLevel += 1;
			console.log('Switching to previous level:', currentLevel + '.');
		}
	}

	function handleWheel(event: WheelEvent) {
		zoom(event.deltaY, event.clientX, event.clientY);
	}
</script>

<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
<div
	id="view"
	role="img"
	onmousedown={handleMouseDown}
	ontouchstart={handleTouchStart}
	style="cursor: {isDragging ? 'grab' : 'crosshair'};"
>
	<div
		id="container"
		style="transform: translate({offsetX}px, {offsetY}px) scale({scale}); {isDragging
			? ''
			: 'transition: transform 0.2s;'}"
	>
		{#if metadata.value && image.state.value}
			{#if annotations.value && imageWidth && imageHeight}
				<div id="annotation-canvas">
					{#each annotations.value as layer, layerIndex}
						<AnnotationLayer {layer} {layerIndex} {imageWidth} {imageHeight} />
					{/each}
				</div>
			{/if}
			<div id="image-canvas">
				{#each image.state.value as layer, layerIndex}
					<ImageLayer {layer} {layerIndex} display={layerIndex === currentLevel} />
				{/each}
			</div>
		{/if}
	</div>
	{#if metadata.value}
		<div id="coordinates-panel" class="panel">
			<span>x:</span>
			{x}, <span>y:</span>
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

	#annotation-canvas {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
	}
</style>
