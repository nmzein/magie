<script lang="ts">
	import { annotations, image, metadata } from '$stores';
	import AnnotationLayer from '$view/AnnotationLayer.svelte';
	import ImageLayer from '$view/ImageLayer.svelte';
	import * as THREE from 'three';

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
	const maxScale = 100;
	const minLevel = 0;
	let maxLevel: number | undefined = $state();
	let currentLevel: number | undefined = $state();
	let imageWidth: number | undefined = $state();
	let imageHeight: number | undefined = $state();

	let scaleBreakpoints: number[] | undefined = $derived.by(() => {
		if (metadata.value === undefined || maxLevel === undefined) return;

		let lowestResolution = metadata.value[maxLevel].width * metadata.value[maxLevel].height;
		let scaleBreakpoints = [];
		// Start at highest resolution (minLevel) and go till second lowest (maxLevel - 1).
		for (let i = minLevel; i < maxLevel; i++) {
			scaleBreakpoints.push(
				Math.sqrt((metadata.value[i].width * metadata.value[i].height) / lowestResolution)
			);
		}

		return scaleBreakpoints;
	});

	let camera: THREE.OrthographicCamera | undefined = $derived.by(() => {
		if (imageWidth === undefined || imageHeight === undefined) return;

		const camera = new THREE.OrthographicCamera(0, imageWidth, 0, -1 * imageHeight, 0.1, 10);
		camera.position.z = 1;

		return camera;
	});

	$effect(() => {
		if (metadata.value === undefined) return;

		for (let i = 0; i < metadata.value.length; i++) {
			if (metadata.value[i].cols <= 4 || metadata.value[i].rows <= 4) {
				maxLevel = i - 1;
				currentLevel = i - 1;
				break;
			}
		}
		imageWidth = metadata.value[0].width;
		imageHeight = metadata.value[0].height;
	});

	$effect(() => {
		// document.addEventListener('touchmove', handleTouchMove);
		// document.addEventListener('touchend', handlePanEnd);
		document.addEventListener('mouseup', handlePanEnd);
		document.addEventListener('wheel', handleWheel);

		(function () {
			var script = document.createElement('script');
			script.onload = function () {
				var stats = new Stats();

				stats.showPanel(2);
				document.body.appendChild(stats.dom);
				requestAnimationFrame(function loop() {
					stats.update();
					requestAnimationFrame(loop);
				});
			};
			script.src = 'https://mrdoob.github.io/stats.js/build/stats.min.js';
			document.head.appendChild(script);
		})();

		return () => {
			// document.removeEventListener('touchmove', handleTouchMove);
			// document.removeEventListener('touchend', handlePanEnd);
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

		if (!isDragging) {
			if (imageWidth === undefined || imageHeight === undefined) return;

			const currentLayer = document
				.getElementById('image-layer-' + currentLevel)
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

		handleLevelChange(delta);
	}

	function handleLevelChange(delta: number) {
		if (currentLevel === undefined || maxLevel === undefined || scaleBreakpoints === undefined)
			return;

		// If at highest detail level and zooming in,
		// or if at lowest detail level and zooming out, do nothing.
		if ((currentLevel == minLevel && delta < 0) || (currentLevel == maxLevel && delta > 0)) {
			let s = delta < 0 ? 'in' : 'out';
			console.log('At level', currentLevel, 'and zooming', s + '. Skip computation.');
			return;
		}

		// If zooming out (not at lowest detail)
		// check current breakpoint (at currentLevel)
		// if scale <>> sB[cL] then cL += 1 (move to lower reso.)
		// e.g. sB = [32, 8] and currently at level 1 and zooming out
		// desired result: move to level 2 (cL + 1)
		// should happen when: scale < 8 (sB[cl])
		// result: cL += 1 (cL = 2)
		if (delta > 0 && scale < scaleBreakpoints[currentLevel]) {
			currentLevel += 1;
			console.log('Switching to lower resolution level:', currentLevel + '.');
		}

		// If zooming in (not at highest detail),
		// check next breakpoint (at currentLevel - 1)
		// if scale > sB[cL - 1] then cL -= 1 (move to higher reso.)
		if (delta < 0 && scale > scaleBreakpoints[currentLevel - 1]) {
			currentLevel -= 1;
			console.log('Switching to higher resolution level:', currentLevel + '.');
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
	onmousemove={handleMouseMove}
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
			{#if annotations.value && imageWidth && imageHeight && camera}
				<div id="annotation-layers">
					{#each annotations.value as annotationLayer, layerIndex}
						<AnnotationLayer {annotationLayer} {layerIndex} {imageWidth} {imageHeight} {camera} />
					{/each}
				</div>
			{/if}
			<div id="image-layers">
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
</style>
