<script lang="ts">
	let {
		annotationLayer,
		layerIndex,
		camera
	}: {
		annotationLayer: AnnotationLayer;
		layerIndex: number;
		camera: Camera;
	} = $props();

	import { image } from '$states';
	import type { AnnotationLayer } from '$types';
	import { untrack } from 'svelte';
	import {
		BufferGeometryLoader,
		Scene,
		MeshBasicMaterial,
		InstancedMesh,
		WebGLRenderer
	} from 'three';
	import type { Camera, BufferGeometry } from 'three';

	const CANVAS_HEIGHT = 8000;
	let CANVAS_WIDTH = $derived.by(() => {
		if (image.width === undefined || image.height === undefined) return;
		return CANVAS_HEIGHT * (image.width / image.height);
	});

	let canvas: HTMLCanvasElement | undefined = $state();

	const scene = new Scene();
	const loader = new BufferGeometryLoader();
	let geometry: BufferGeometry | undefined;

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
			color: annotationLayer.fill
		})
	);

	$effect(() => {
		untrack(() => (geometry = loader.parse(JSON.parse(annotationLayer.geometry))));
		untrack(() => (annotationLayer.geometry = ''));
	});

	$effect(() => render());

	function render() {
		if (!geometry) return;

		let start = performance.now();
		// renderer.clear();
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
	style:z-index={100 + layerIndex}
	style:display={annotationLayer.visible ? 'block' : 'none'}
	style:opacity={annotationLayer.opacity}
></canvas>
