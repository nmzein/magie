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

	const CANVAS_WIDTH = 2000;
	const CANVAS_HEIGHT = CANVAS_WIDTH * (imageHeight / imageWidth);

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
	const fillMaterial = new THREE.MeshBasicMaterial({
		color: annotationLayer.fill
	});

	$effect(() => untrack(() => draw()));

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
		const geometry = BufferGeometryUtils.mergeGeometries(geometries);
		// Create a mesh with the geometries and materials.
		const mesh = new THREE.InstancedMesh(geometry, fillMaterial, 1);
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
	style="z-index: {1 + layerIndex}; display: {annotationLayer.visible
		? 'block'
		: 'none'}; opacity: {annotationLayer.opacity};"
/>

<style lang="scss">
	canvas {
		position: absolute;
		width: 100%;
	}
</style>
