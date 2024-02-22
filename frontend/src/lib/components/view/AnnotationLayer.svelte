<script lang="ts">
	import Annotation from '$view/Annotation.svelte';
	import type { AnnotationLayer } from '$types';

	let { layer, layerIndex, imageWidth, imageHeight } = $props<{
		layer: AnnotationLayer;
		layerIndex: number;
		imageWidth: number;
		imageHeight: number;
	}>();
</script>

<div id={'annotation-layer-' + layer.tag}>
	{#each layer.annotations as coordinates, annotationIndex}
		<Annotation
			{coordinates}
			{imageWidth}
			{imageHeight}
			id={'annotation-' + layerIndex + '-' + annotationIndex}
			colours={layer.colours}
		/>
	{/each}
</div>

<style lang="scss">
	div {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
	}
</style>
