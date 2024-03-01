<script lang="ts">
	let { annotationLayer, layerIndex, imageWidth, imageHeight } = $props<{
		annotationLayer: AnnotationLayer;
		layerIndex: number;
		imageWidth: number;
		imageHeight: number;
	}>();

	import type { AnnotationLayer } from '$types';
	import * as THREE from 'three';
	import * as BufferGeometryUtils from 'three/addons/utils/BufferGeometryUtils.js';

	let canvases: HTMLCanvasElement[] = $state([]);

	const CANVAS_WIDTH = 2000;
	const CANVAS_HEIGHT = CANVAS_WIDTH * (imageHeight / imageWidth);

	// Start off with highest area annotations.
	let currentResolutionLayer = $state(0);

	// Materials for this annotation layer.
	const fillMaterial = new THREE.MeshBasicMaterial({
		color: annotationLayer.fill,
		transparent: true,
		opacity: annotationLayer.opacity
	});

	let drawn = Array(annotationLayer.resolutions.length).fill(false);

	function draw(resolutionIndex: number) {
		let canvas = canvases[resolutionIndex];
		if (canvas === undefined) return;

		let start = performance.now();

		const scene = new THREE.Scene();

		const camera = new THREE.OrthographicCamera(0, imageWidth, 0, -1 * imageHeight, 0.1, 10);
		camera.position.z = 1;

		// Create a renderer with a transparent canvas.
		const renderer = new THREE.WebGLRenderer({
			canvas,
			alpha: true,
			precision: 'lowp',
			powerPreference: 'high-performance'
		});

		console.log(
			'Creating annotation layer',
			layerIndex,
			'resolution',
			resolutionIndex,
			'took',
			performance.now() - start,
			'ms'
		);

		start = performance.now();

		let geometries: THREE.BufferGeometry[] = [];

		annotationLayer.resolutions[resolutionIndex].forEach((annotation, _) => {
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

		// Merge the geometries into a single geometry to minimise draw calls.
		const geometry = BufferGeometryUtils.mergeGeometries(geometries);
		// Create a mesh with the geometries and materials.
		const mesh = new THREE.Mesh(geometry, fillMaterial);
		// Add the shapes to the scene.
		scene.add(mesh);

		console.log(
			'Drawing annotation layer',
			layerIndex,
			'resolution',
			resolutionIndex,
			'took',
			performance.now() - start,
			'ms'
		);
		// Clear annotations for given resolution.
		annotationLayer.resolutions[resolutionIndex] = [];

		start = performance.now();

		// Render the scene.
		renderer.render(scene, camera);

		console.log(
			'Rendering annotation layer',
			layerIndex,
			'resolution',
			resolutionIndex,
			'took',
			performance.now() - start,
			'ms'
		);

		console.log('Scene polycount: ', renderer.info.render.triangles);
		console.log('Active Drawcalls: ', renderer.info.render.calls);
		console.log('Textures in Memory:', renderer.info.memory.textures);
		console.log('Geometries in Memory:', renderer.info.memory.geometries);

		// Set to drawn to prevent redraw.
		drawn[resolutionIndex] = true;
	}
</script>

{#each annotationLayer.resolutions as _, resolutionIndex}
	<canvas
		bind:this={canvases[resolutionIndex]}
		width={CANVAS_WIDTH}
		height={CANVAS_HEIGHT}
		id={'annotation-layer-' + layerIndex + '-resolution-' + resolutionIndex}
		style="z-index: {1 + layerIndex}; display: {annotationLayer.visible &&
		resolutionIndex <= currentResolutionLayer
			? 'block'
			: 'none'}; opacity: {annotationLayer.opacity};"
	>
		{!drawn[resolutionIndex] && resolutionIndex == currentResolutionLayer && draw(resolutionIndex)}
	</canvas>
{/each}

<style lang="scss">
	canvas {
		position: absolute;
		width: 100%;
	}
</style>
