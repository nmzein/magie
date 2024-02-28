<script lang="ts">
	let { layer, layerIndex, imageWidth, imageHeight } = $props<{
		layer: AnnotationLayer;
		layerIndex: number;
		imageWidth: number;
		imageHeight: number;
	}>();

	import type { AnnotationLayer } from '$types';
	import { untrack } from 'svelte';
	import * as THREE from 'three';

	let canvas: HTMLCanvasElement | undefined = $state();

	const CANVAS_WIDTH = 2000;
	const CANVAS_HEIGHT = CANVAS_WIDTH * (imageHeight / imageWidth);

	$effect(() => untrack(() => draw()));

	function draw() {
		if (!canvas) return;

		let start = performance.now();

		const scene = new THREE.Scene();

		const camera = new THREE.OrthographicCamera(0, imageWidth, 0, -1 * imageHeight, 0.1, 10);
		camera.position.z = 1;

		// Create a renderer with a transparent canvas.
		const renderer = new THREE.WebGLRenderer({ canvas, alpha: true });

		// Materials for this layer.
		const fill_material = new THREE.MeshBasicMaterial({
			color: layer.fill,
			transparent: true,
			opacity: layer.opacity
		});

		console.log('Creating annotation layer', layerIndex, 'took', performance.now() - start, 'ms');

		start = performance.now();

		layer.annotations.forEach((annotation, index) => {
			const shape = new THREE.Shape();

			shape.moveTo(annotation[0][0], -1 * annotation[0][1]);
			annotation[0] = [];
			for (let i = 1; i < annotation.length; i++) {
				shape.lineTo(annotation[i][0], -1 * annotation[i][1]);
				annotation[i] = [];
			}
			shape.closePath();

			const geometry = new THREE.ShapeGeometry(shape);
			// Create a mesh with the geometry and materials.
			const mesh = new THREE.Mesh(geometry, fill_material);
			// Add the shape to the scene.
			scene.add(mesh);
		});

		console.log('Drawing annotation layer', layerIndex, 'took', performance.now() - start, 'ms');
		// Clear annotations.
		layer.annotations = [];

		start = performance.now();

		// Render the scene.
		renderer.render(scene, camera);

		console.log('Rendering annotation layer', layerIndex, 'took', performance.now() - start, 'ms');
	}
</script>

<canvas
	bind:this={canvas}
	width={CANVAS_WIDTH}
	height={CANVAS_HEIGHT}
	id={'annotation-layer-' + layerIndex}
	style="display: {layer.visible ? 'block' : 'none'}; opacity: {layer.opacity};"
/>

<style lang="scss">
	canvas {
		position: absolute;
		z-index: 1;
		width: 100%;
	}
</style>
