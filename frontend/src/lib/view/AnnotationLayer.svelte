<script lang="ts">
	import { IMAGE_URL } from '$api';
	import { defined } from '$helpers';
	import type { AnnotationLayer } from '$types';
	import { onMount } from 'svelte';
	import { MeshBasicMaterial, type Mesh } from 'three';
	import { GLTFLoader } from 'three/addons/loaders/GLTFLoader.js';

	let {
		imageId,
		layer,
		render
	}: {
		imageId: number;
		layer: AnnotationLayer;
		render: (tag: string, mesh: Mesh) => void;
	} = $props();

	let mesh: Mesh | undefined = $state();
	const loader = new GLTFLoader();

	// Materials for this annotation layer.
	const fillMaterial = $derived(
		new MeshBasicMaterial({
			color: layer.fill,
			opacity: layer.opacity,
			transparent: true
		})
	);

	onMount(async () => {
		const data = await loader.loadAsync(`${IMAGE_URL}/${imageId}/annotations/${layer.id}`);

		const node = data.scene.children[0];

		if (node.type === 'Mesh') {
			mesh = node as Mesh;
			mesh.name = layer.tag;
		}
	});

	$effect(() => {
		if (defined(mesh)) {
			mesh.material = fillMaterial;
			mesh.visible = layer.visible;
			render(layer.tag, mesh);
		}
	});
</script>
