<script lang="ts">
	import { defined } from '$helpers';
	import { image, transformer } from '$states';
	import AnnotationLayer from '$view/AnnotationLayer.svelte';
	import ImageLayer from '$view/ImageLayer.svelte';
	import { Mesh, OrthographicCamera, Scene, WebGLRenderer } from 'three';

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
		if (!image.initialised || image.properties === undefined) return;

		for (let i = 0; i < image.levels; i++) {
			if (image.properties.metadata[i].cols <= 4 || image.properties.metadata[i].rows <= 4) {
				transformer.maxLevel = i - 1;
				transformer.currentLevel = i - 1;
				break;
			}
		}

		// transformer.maxLevel = image.properties.metadata.length - 1;
		// transformer.currentLevel = image.properties.metadata.length - 1;
	});

	$effect(() => {
		document.addEventListener('mousemove', handleMouseMove);
		document.addEventListener('touchmove', handleTouchMove);
		document.addEventListener('mouseup', handlePanEnd);
		document.addEventListener('touchend', handlePanEnd);
		document.addEventListener('wheel', handleWheel);

		let script = document.createElement('script');
		script.onload = function () {
			let stats = new Stats();

			stats.showPanel(0);
			document.body.appendChild(stats.dom);
			requestAnimationFrame(function loop() {
				stats.update();
				requestAnimationFrame(loop);
			});
		};
		script.src = 'https://mrdoob.github.io/stats.js/build/stats.min.js';
		document.head.appendChild(script);

		return () => {
			document.removeEventListener('mousemove', handleMouseMove);
			document.removeEventListener('touchmove', handleTouchMove);
			document.removeEventListener('mouseup', handlePanEnd);
			document.removeEventListener('touchend', handlePanEnd);
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

	let canvas: HTMLCanvasElement | undefined = $state();

	const CANVAS_HEIGHT = 8000;
	let CANVAS_WIDTH = $derived.by(() => {
		if (!image.initialised || !defined(image.width) || !defined(image.height)) return;
		return CANVAS_HEIGHT * (image.width / image.height);
	});

	// Create a renderer with a transparent canvas.
	const renderer = $derived.by(() => {
		if (!defined(canvas)) return;

		return new WebGLRenderer({
			canvas,
			alpha: true,
			precision: 'highp',
			powerPreference: 'high-performance'
		});
	});

	let scene: Scene | undefined = $state();

	$effect(() => {
		if (defined(canvas)) {
			scene = new Scene();
		}
	});

	function render(tag: string, mesh: Mesh) {
		if (!defined(camera) || !defined(renderer) || !defined(scene)) return;

		let start = performance.now();

		// Remove mesh if it exists.
		scene.remove(mesh);
		// Add the mesh to the scene.
		scene.add(mesh);
		// Render the scene.
		renderer.render(scene, camera);

		console.log('Rendering Layer', tag, 'took', performance.now() - start, 'ms');
		console.log('Scene Polycount: ', renderer.info.render.triangles);
	}
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<div
	role="img"
	onmousedown={handleMouseDown}
	ontouchstart={handleTouchStart}
	class="absolute h-screen overflow-hidden"
	style="cursor: {isDragging ? 'grab' : 'crosshair'};"
>
	<div
		class="relative h-full w-screen origin-top-left"
		style="transform: translate({transformer.offsetX}px, {transformer.offsetY}px) scale({transformer.scale});
			   {isDragging ? '' : 'transition: transform 0.2s;'}"
	>
		{#if image.initialised && defined(image.properties) && image.properties.annotations.length > 0}
			<div id="annotation-layers" class="absolute z-20 h-full w-full">
				{#if defined(CANVAS_WIDTH)}
					<canvas bind:this={canvas} width={CANVAS_WIDTH} height={CANVAS_HEIGHT} class="w-full"
					></canvas>
				{/if}
				{#if defined(image.info) && defined(camera) && defined(renderer) && defined(scene)}
					{#each image.properties.annotations as layer}
						<AnnotationLayer imageId={image.info.id} {layer} {render} />
					{/each}
				{/if}
			</div>
		{/if}
		<div id="image-layers" class="absolute z-10 h-full w-full">
			{#if defined(transformer.currentLevel)}
				{#each image.tiles as layer, layerIndex}
					<ImageLayer {layer} {layerIndex} display={layerIndex === transformer.currentLevel} />
				{/each}
			{/if}
		</div>
	</div>
	{#if image.initialised}
		<div
			id="coordinates-panel"
			class="panel absolute bottom-[10px] left-[10px] select-none px-[7px] py-[3px]"
		>
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
