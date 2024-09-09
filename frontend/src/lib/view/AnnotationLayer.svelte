<script lang="ts">
	import { IMAGE_ANNOTATIONS_URL } from '$api';
	import { defined } from '$helpers';
	import { image } from '$states';
	import type { AnnotationLayer } from '$types';
	import { onMount } from 'svelte';
	import {
		Scene,
		WebGLRenderer,
		MeshBasicMaterial,
		type Camera,
		type BufferGeometry,
		type Object3D,
		type Mesh
	} from 'three';
	import { GLTFLoader } from 'three/addons/loaders/GLTFLoader.js';

	let { imageId, layer, camera }: { imageId: number; layer: AnnotationLayer; camera: Camera } =
		$props();

	const CANVAS_HEIGHT = 8000;
	let CANVAS_WIDTH = $derived.by(() => {
		if (image.width === undefined || image.height === undefined) return;
		return CANVAS_HEIGHT * (image.width / image.height);
	});

	let canvas: HTMLCanvasElement | undefined = $state();

	const scene = new Scene();
	let mesh: Mesh | undefined;
	const loader = new GLTFLoader();
	let geometry: BufferGeometry | undefined;
	let firstRender = true;

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

	function isMesh(object: Object3D): object is Mesh {
		return (object as Mesh).material !== undefined;
	}

	onMount(async () => {
		// Load a glTF resource
		const data = await loader.loadAsync(
			// resource URL
			`${IMAGE_ANNOTATIONS_URL}?image_id=${imageId}&annotation_layer_id=${layer.id}`,
			// called while loading is progressing
			function (progress) {
				console.log((progress.loaded / progress.total) * 100 + '% loaded');
			}
		);

		const node = data.scene.children[0];
		mesh = isMesh(node) ? node : undefined;
		if (!defined(mesh)) return;
		render();

		firstRender = false;
	});

	$effect(() => {
		if (fillMaterial && !firstRender) {
			render();
		}
	});

	function render() {
		if (!defined(mesh)) return;

		let start = performance.now();

		// Clear scene;
		scene.clear();

		mesh.traverse(function (node) {
			if (isMesh(node)) {
				node.material = fillMaterial;
			}
		});

		// Add the mesh to the scene.
		scene.add(mesh);
		// Render the scene.
		renderer.render(scene, camera);

		console.log('Rendering Layer', layer.tag, 'took', performance.now() - start, 'ms');
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
	id={'annotation-layer-' + layer.id}
	class="absolute w-full"
	class:hidden={!layer.visible}
	style:z-index={100 + layer.id}
	style:opacity={layer.opacity}
></canvas>
