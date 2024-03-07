<script lang="ts">
	let { annotationLayer, layerIndex, imageWidth, imageHeight, camera } = $props<{
		annotationLayer: AnnotationLayer;
		layerIndex: number;
		imageWidth: number;
		imageHeight: number;
		camera: THREE.Camera;
	}>();

	import type { AnnotationLayer } from '$types';
	import { untrack } from 'svelte';
	import * as THREE from 'three';
	import * as BufferGeometryUtils from 'three/addons/utils/BufferGeometryUtils.js';

	let canvas: HTMLCanvasElement | undefined = $state();

	const CANVAS_HEIGHT = 8000;
	const CANVAS_WIDTH = CANVAS_HEIGHT * (imageWidth / imageHeight);

	const scene = new THREE.Scene();

	// Create a renderer with a transparent canvas.
	const renderer = $derived(
		new THREE.WebGLRenderer({
			canvas,
			alpha: true,
			precision: 'lowp',
			powerPreference: 'high-performance'
		})
	);

	// Materials for this annotation layer.
	let fillMaterial = $derived(
		new THREE.MeshBasicMaterial({
			color: annotationLayer.fill
		})
	);

	let geometry: THREE.BufferGeometry | undefined = $state();

	$effect(() => untrack(() => draw()));
	$effect(() => render());

	function draw() {
		let start = performance.now();

		let geometries: THREE.BufferGeometry[] = [];

		annotationLayer.annotations.forEach((annotation, _) => {
			const shape = new THREE.Shape();
			shape.moveTo(annotation[0][0], -1 * annotation[0][1]);
			annotation[0] = [];
			for (let i = 1; i < annotation.length; i++) {
				shape.lineTo(annotation[i][0], -1 * annotation[i][1]);
				annotation[i] = [];
			}
			shape.closePath();
			geometries.push(new THREE.ShapeGeometry(shape));
		});
		// Clear annotations for given resolution.
		annotationLayer.annotations = [];

		console.log('Drawing annotation layer', layerIndex, 'took', performance.now() - start, 'ms');
		start = performance.now();

		// Merge the geometries into a single geometry to minimise draw calls.
		geometry = BufferGeometryUtils.mergeGeometries(geometries);
	}

	function render(material: THREE.Material = fillMaterial) {
		let start = performance.now();

		renderer.clear();
		// Create a mesh with the geometries and materials.
		const mesh = new THREE.InstancedMesh(geometry, material, 1);
		// Add the shapes to the scene.
		scene.add(mesh);

		console.log('Mesh creation for layer ', layerIndex, 'took', performance.now() - start, 'ms');
		start = performance.now();

		// Render the scene.
		renderer.render(scene, camera);

		console.log('Rendering annotation layer', layerIndex, 'took', performance.now() - start, 'ms');

		console.log('Scene polycount: ', renderer.info.render.triangles);
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
/>

<style lang="scss">
	canvas {
		position: absolute;
		width: 100%;
	}
</style>
