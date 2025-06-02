<script lang="ts">
	import { MeshBasicMaterial, type Mesh } from 'three';
	import type { GLTF } from 'three/examples/jsm/loaders/GLTFLoader.js';
	import type { Geometry2DLayer } from './types.ts';
	import { defined } from '$helpers';
	import { onMount } from 'svelte';

	type Props = {
		layer: Geometry2DLayer;
		fetch: (id: number) => Promise<GLTF>;
		render: (tag: string, mesh: Mesh) => void;
	};

	let { layer, fetch, render }: Props = $props();

	let mesh: Mesh | undefined = $state();

	// Materials for this annotation layer.
	const fillMaterial = $derived(
		new MeshBasicMaterial({
			color: layer.fill,
			opacity: layer.opacity,
			transparent: true
		})
	);

	onMount(async () => {
		const data = await fetch(layer.id);
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
