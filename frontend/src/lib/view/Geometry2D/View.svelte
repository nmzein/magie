<script lang="ts">
	import { Mesh, OrthographicCamera, Scene, WebGLRenderer } from 'three';
	import type { GLTF } from 'three/examples/jsm/Addons.js';
	import { defined } from '$helpers';
	import Layer from './Layer.svelte';
	import type { Geometry2DLayer } from './types.ts';

	let {
		width,
		height,
		geometries,
		fetch
	}: {
		width: number;
		height: number;
		geometries: Geometry2DLayer[];
		fetch: (id: number) => Promise<GLTF>;
	} = $props();

	let canvas: HTMLCanvasElement | undefined = $state();

	const CANVAS_HEIGHT = 8000;
	const CANVAS_WIDTH = $derived(CANVAS_HEIGHT * (width / height));

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

	const scene: Scene | undefined = $derived.by(() => {
		if (!defined(canvas)) return;

		return new Scene();
	});

	const camera: OrthographicCamera | undefined = $derived.by(() => {
		const camera = new OrthographicCamera(0, width, 0, -1 * height, 0.1, 10);
		camera.position.z = 1;

		return camera;
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

<div class="absolute z-20 h-full w-full">
	{#if defined(CANVAS_WIDTH)}
		<canvas bind:this={canvas} width={CANVAS_WIDTH} height={CANVAS_HEIGHT} class="w-full"></canvas>
	{/if}
	{#if defined(camera) && defined(renderer) && defined(scene)}
		<!-- TODO: Dont make this a layer anymore -->
		{#each geometries as layer}
			<Layer {layer} {fetch} {render} />
		{/each}
	{/if}
</div>
