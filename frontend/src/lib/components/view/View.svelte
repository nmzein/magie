<script lang="ts">
	import { onMount } from 'svelte';
	import { metadata, annotations } from '$lib/stores';
	import AnnotationCanvas from '$lib/components/view/AnnotationCanvas.svelte';
	import ImageCanvas from '$lib/components/view/ImageCanvas.svelte';

	$: currentLevel = 1;
	let isDragging = false;
	let panStartX: number;
	let panStartY: number;
	let offsetX = 0;
	let offsetY = 0;
	let scale = 1;
	let x = 0;
	let y = 0;
	let container: DOMRect | undefined;

	onMount(() => {
		document.addEventListener('mousemove', handleMouseMove);
		document.addEventListener('touchmove', handleTouchMove);
		document.addEventListener('mouseup', handlePanEnd);
		document.addEventListener('touchend', handlePanEnd);
		document.addEventListener('wheel', handleWheel);

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

		isDragging = true;
		panStartX = event.clientX;
		panStartY = event.clientY;
	}

	function handleTouchStart(event: TouchEvent) {
		event.preventDefault();

		isDragging = true;
		const touch = event.touches[0];
		panStartX = touch.clientX;
		panStartY = touch.clientY;
	}

	function handleMouseMove(event: MouseEvent) {
		event.preventDefault();

		if (!isDragging) {
			const imageWidth = $metadata?.[0].width;
			const imageHeight = $metadata?.[0].height;
			if (!imageWidth || !imageHeight) {
				return;
			}

			const containerWidth = container?.width;
			const containerHeight = container?.height;
			if (!containerWidth || !containerHeight) {
				container = document.getElementById('image-grid-layer-0')?.getBoundingClientRect();
				return;
			}

			x = Math.floor((event.clientX - offsetX) * (imageWidth / (containerWidth * scale)));
			y = Math.floor((event.clientY - offsetY) * (imageHeight / (containerHeight * scale)));

			return;
		}

		const deltaX = event.clientX - panStartX;
		const deltaY = event.clientY - panStartY;

		offsetX += deltaX;
		offsetY += deltaY;

		panStartX = event.clientX;
		panStartY = event.clientY;
	}

	function handleTouchMove(event: TouchEvent) {
		if (!isDragging) {
			return;
		}
		const touch = event.touches[0];

		const deltaX = touch.clientX - panStartX;
		const deltaY = touch.clientY - panStartY;

		offsetX += deltaX;
		offsetY += deltaY;

		panStartX = touch.clientX;
		panStartY = touch.clientY;
	}

	function handlePanEnd() {
		isDragging = false;
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

		if (!$metadata) {
			return;
		}

		// If at highest detail level and zooming in,
		// or if at lowest detail level and zooming out, do nothing.
		// if (
		// 	(currentLevel === 0 && event.deltaY < 0) ||
		// 	(currentLevel === $metadata.length - 1 && event.deltaY > 0)
		// ) {
		// 	let s = event.deltaY < 0 ? 'in' : 'out';
		// 	console.log('currentLevel', currentLevel, 'and zooming', s);
		// 	return;
		// }

		const currentLayer = document
			.getElementById('image-grid-layer-' + currentLevel)
			?.getBoundingClientRect();

		if (!currentLayer || !currentLayer.width) {
			return;
		}

		const viewportWidth = window.innerWidth;
		if (!currentLayer || !currentLayer.width) {
			return;
		}
		// console.log('currentLayer', currentLayer);
		// console.log('viewport width', viewportWidth);

		// If current layer width is larger than viewport width, switch to next level.
		if (currentLayer.width / viewportWidth > 1) {
			if (currentLevel == 0) {
				return;
			}
			currentLevel -= 1;
			console.log('switching to next level: ', currentLevel);
		} else if (currentLayer.width / viewportWidth < 0.9) {
			if (currentLevel == $metadata.length - 1) {
				return;
			}
			currentLevel += 1;
			console.log('switching to prev level: ', currentLevel);
		}
	}
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
	id="view"
	on:mousedown={handleMouseDown}
	on:touchstart={handleTouchStart}
	style="height: 100vh; cursor: {isDragging ? 'grab' : 'crosshair'};"
>
	<div
		id="container"
		style="transform: translate({offsetX}px, {offsetY}px) scale({scale}); {isDragging
			? ''
			: 'transition: transform 0.2s;'}"
	>
		{#if $metadata}
			{#if $annotations}
				<AnnotationCanvas />
			{/if}
			<ImageCanvas {currentLevel} />
		{/if}
	</div>
	{#if $metadata}
		<div id="coordinates-panel" class="panel">
			<p><b>x:</b> {x}, <b>y:</b> {y}</p>
		</div>
	{/if}
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

	@media (hover: none) {
		#coordinates-panel {
			// Hide the element on touch-capable devices.
			display: none;
		}
	}
</style>
