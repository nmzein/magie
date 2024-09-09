<script lang="ts">
	import { http } from '$api';
	import { defined } from '$helpers';
	import { image } from '$states';
	import type { AnnotationLayer } from '$types';
	import { onMount } from 'svelte';
	import {
		BufferGeometryLoader,
		Scene,
		MeshBasicMaterial,
		InstancedMesh,
		WebGLRenderer,
		type Camera,
		type BufferGeometry
	} from 'three';

	let { imageId, layer, camera }: { imageId: number; layer: AnnotationLayer; camera: Camera } =
		$props();

	const CANVAS_HEIGHT = 8000;
	let CANVAS_WIDTH = $derived.by(() => {
		if (image.width === undefined || image.height === undefined) return;
		return CANVAS_HEIGHT * (image.width / image.height);
	});

	let canvas: HTMLCanvasElement | undefined = $state();

	const scene = new Scene();
	const loader = new BufferGeometryLoader();
	let geometry: BufferGeometry | undefined;
	let firstRender = true;

	onMount(async () => {
		await http.GetAnnotations(imageId, layer.id).then((geom) => {
			if (!defined(geom)) return;
			geometry = loader.parse(JSON.parse(geom));
			render();
			firstRender = false;
		});
	});

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

	$effect(() => {
		if (fillMaterial && !firstRender) {
			render();
		}
	});

	function render() {
		let start = performance.now();
		// Create a mesh with the geometries and materials.
		let mesh = new InstancedMesh(geometry, fillMaterial, 1);
		// Add the shapes to the scene.
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
