<script lang="ts">
	import { image } from '$lib/stores';
	import ImageLayer from '$lib/components/view/ImageLayer.svelte';

	export let currentLevel: number;
</script>

<div id="image-canvas">
	{#each $image as layer, layerIndex}
		<ImageLayer {layer} {layerIndex} display={layerIndex === currentLevel} />
	{/each}
</div>
