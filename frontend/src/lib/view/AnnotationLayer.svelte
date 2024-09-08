<script lang="ts">
	import { image } from '$states';
	import type { AnnotationLayer } from '$types';
	import {
		BufferGeometryLoader,
		Scene,
		MeshBasicMaterial,
		InstancedMesh,
		WebGLRenderer,
		type Camera,
		type BufferGeometry
	} from 'three';

	let {
		layer,
		layerIndex,
		camera
	}: { layer: AnnotationLayer; layerIndex: number; camera: Camera } = $props();

	const CANVAS_HEIGHT = 8000;
	let CANVAS_WIDTH = $derived.by(() => {
		if (image.width === undefined || image.height === undefined) return;
		return CANVAS_HEIGHT * (image.width / image.height);
	});

	let canvas: HTMLCanvasElement | undefined = $state();

	const scene = new Scene();
	const loader = new BufferGeometryLoader();
	let geometry: BufferGeometry = $derived(loader.parse(JSON.parse(layer.geometry)));

	// Create a renderer with a transparent canvas.
	const renderer = $derived(
		new WebGLRenderer({
			canvas,
			alpha: true,
			precision: 'highp',
			powerPreference: 'high-performance'
		})
	);

	// Materials for this annotation layer.
	let fillMaterial = $derived(
		new MeshBasicMaterial({
			color: layer.fill
		})
	);

	$effect(() => render());

	function render() {
		let start = performance.now();
		// Create a mesh with the geometries and materials.
		let mesh = new InstancedMesh(geometry, fillMaterial, 1);
		// Add the shapes to the scene.
		scene.add(mesh);
		// Render the scene.
		renderer.render(scene, camera);

		console.log('Rendering Layer', layerIndex, 'took', performance.now() - start, 'ms');
		console.log('Scene Polycount: ', renderer.info.render.triangles);
		console.log('Active Drawcalls: ', renderer.info.render.calls);
		console.log('Textures in Memory:', renderer.info.memory.textures);
		console.log('Geometries in Memory:', renderer.info.memory.geometries);
	}
</script>

<canvas
	bind:this={canvas}
	width={CANVAS_WIDTH}
	height={CANVAS_HEIGHT}
	id={'annotation-layer-' + layerIndex}
	class="absolute w-full"
	class:hidden={!layer.visible}
	style:z-index={100 + layerIndex}
	style:opacity={layer.opacity}
></canvas>
