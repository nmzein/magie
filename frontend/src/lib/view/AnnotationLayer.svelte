<script lang="ts">
	let {
		annotationLayer,
		layerIndex,
		imageWidth,
		imageHeight,
		camera
	}: {
		annotationLayer: AnnotationLayer;
		layerIndex: number;
		imageWidth: number;
		imageHeight: number;
		camera: Camera;
	} = $props();

	import type { AnnotationLayer } from '$types';
	import {
		BufferGeometryLoader,
		Scene,
		MeshBasicMaterial,
		InstancedMesh,
		WebGLRenderer
	} from 'three';

	import type { Camera, BufferGeometry } from 'three';

	let canvas: HTMLCanvasElement | undefined = $state();

	const CANVAS_HEIGHT = 10000;
	const CANVAS_WIDTH = CANVAS_HEIGHT * (imageWidth / imageHeight);

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
		console.log(annotationLayer.geometry.length);
		geometry = loader.parse(JSON.parse(annotationLayer.geometry));
		annotationLayer.geometry = '';
		render();
	});

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
	style="z-index: {100 + layerIndex}; display: {annotationLayer.visible
		? 'block'
		: 'none'}; opacity: {annotationLayer.opacity};"
></canvas>

<style lang="scss">
	canvas {
		position: absolute;
		width: 100%;
	}
</style>
