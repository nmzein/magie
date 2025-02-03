<script lang="ts">
	import { defined } from '$helpers';
	import { images } from '$states';
	import AnnotationLayer from '$view/AnnotationLayer.svelte';
	import ImageLayer from '$view/ImageLayer.svelte';
	import { Mesh, OrthographicCamera, Scene, WebGLRenderer } from 'three';

	// TODO: FIX
	let x = $state(0);
	let y = $state(0);

	const camera: OrthographicCamera | undefined = $derived.by(() => {
		if (!images[0]?.initialised) return;

		const camera = new OrthographicCamera(0, images[0].width!, 0, -1 * images[0].height!, 0.1, 10);
		camera.position.z = 1;

		return camera;
	});

	function onmousedown(event: MouseEvent) {
		if (!images[0]?.initialised) return;

		event.preventDefault();
		handlePanStart(event);
	}

	function ontouchstart(event: TouchEvent) {
		if (!images[0]?.initialised) return;

		handlePanStart(event.touches[0]);
	}

	function handlePanStart(event: MouseEvent | Touch) {
		if (!images[0]?.initialised) return;

		images[0].transformer.isDragging = true;
		images[0].transformer.panStartX = event.clientX;
		images[0].transformer.panStartY = event.clientY;
	}

	function onmousemove(event: MouseEvent) {
		if (!images[0]?.initialised) return;

		event.preventDefault();

		// Logic for calculating the coordinates of the mouse pointer.
		if (
			!images[0].transformer.isDragging &&
			defined(images[0].width) &&
			defined(images[0].height)
		) {
			// TODO: Don't do this on every mouse move.
			const currentLayer = document
				.getElementById('images[0]-layer-' + images[0].transformer.currentLevel)
				?.getBoundingClientRect();

			if (!currentLayer) return;

			const xTemp = Math.floor(
				(event.clientX - images[0].transformer.offsetX) * (images[0].width / currentLayer.width)
			);
			const yTemp = Math.floor(
				(event.clientY - images[0].transformer.offsetY) * (images[0].height / currentLayer.height)
			);

			if (Number.isFinite(xTemp) && !isNaN(xTemp)) x = xTemp;
			if (Number.isFinite(yTemp) && !isNaN(yTemp)) y = yTemp;

			return;
		}

		handlePan(event);
	}

	function ontouchmove(event: TouchEvent) {
		if (!images[0]?.initialised || !images[0].transformer.isDragging) return;

		handlePan(event.touches[0]);
	}

	function handlePan(event: MouseEvent | Touch) {
		if (!images[0]?.initialised) return;

		images[0].transformer.offsetX += event.clientX - images[0].transformer.panStartX;
		images[0].transformer.offsetY += event.clientY - images[0].transformer.panStartY;

		images[0].transformer.panStartX = event.clientX;
		images[0].transformer.panStartY = event.clientY;
	}

	function handlePanEnd() {
		if (!images[0]?.initialised) return;

		images[0].transformer.isDragging = false;
	}

	function onwheel(event: WheelEvent) {
		if (!images[0]?.initialised) return;

		images[0].transformer.zoom(event.deltaY, event.clientX, event.clientY);
	}

	let canvas: HTMLCanvasElement | undefined = $state();

	const CANVAS_HEIGHT = 8000;
	const CANVAS_WIDTH = $derived.by(() => {
		if (!images[0]?.initialised) return;

		return CANVAS_HEIGHT * (images[0].width / images[0].height);
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

<svelte:document
	{onmousemove}
	{ontouchmove}
	onmouseup={handlePanEnd}
	ontouchend={handlePanEnd}
	{onwheel}
/>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
{#if images[0]?.initialised}
	<div
		role="img"
		{onmousedown}
		{ontouchstart}
		class="absolute h-dvh overflow-hidden"
		style="cursor: {images[0].transformer.isDragging ? 'grab' : 'crosshair'};"
	>
		<div
			class="relative h-full w-screen origin-top-left"
			style="transform: translate({images[0].transformer.offsetX}px, {images[0].transformer
				.offsetY}px) scale({images[0].transformer.scale});
			   {images[0].transformer.isDragging ? '' : 'transition: transform 0.2s;'}"
		>
			{#if images[0].properties.annotations.length > 0}
				<div id="annotation-layers" class="absolute z-20 h-full w-full">
					{#if defined(CANVAS_WIDTH)}
						<canvas bind:this={canvas} width={CANVAS_WIDTH} height={CANVAS_HEIGHT} class="w-full"
						></canvas>
					{/if}
					{#if defined(camera) && defined(renderer) && defined(scene)}
						{#each images[0].properties.annotations as layer}
							<AnnotationLayer imageId={images[0].info.id} {layer} {render} />
						{/each}
					{/if}
				</div>
			{/if}
			<div id="images-layers" class="absolute z-10 h-full w-full">
				{#each images[0].tiles as layer, layerIndex}
					<ImageLayer
						{layer}
						{layerIndex}
						display={layerIndex === images[0].transformer.currentLevel}
					/>
				{/each}
			</div>
		</div>
		<div
			id="coordinates-panel"
			class="panel absolute bottom-[10px] left-[10px] select-none px-[7px] py-[3px]"
		>
			<span class="font-bold">x:</span>
			{x},
			<span class="font-bold">y:</span>
			{y}
		</div>
	</div>
{/if}

<style>
	@media (hover: none) {
		#coordinates-panel {
			/* Hide the element on touch-capable devices. */
			display: none;
		}
	}
</style>
