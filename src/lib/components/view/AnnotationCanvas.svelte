<script lang="ts">
	import { onMount } from 'svelte';
	import Annotation from '$lib/components/view/Annotation.svelte';
	import { metadata, annotations } from '$lib/stores';

	export let scale: number;

	$: image_grid_width = 0;
	$: image_grid_height = 0;
	$: scale_x = 1;
	$: scale_y = 1;

	onMount(() => {
		window.addEventListener('resize', ResizeAnnotations);

		ResizeAnnotations();

		return () => {
			window.removeEventListener('resize', ResizeAnnotations);
		};
	});

	function ResizeAnnotations() {
		const ImageGridDimensions = document.getElementById('image-grid')?.getClientRects();

		image_grid_width = ImageGridDimensions?.[0].width || 1000;
		image_grid_height = ImageGridDimensions?.[0].height || 1000;

		// coordinate * (image grid width (variable) / actual image width (constant))
		if (image_grid_width && image_grid_height && $metadata && $metadata.width && $metadata.height) {
			scale_x = image_grid_width / ($metadata.width * scale);
			scale_y = image_grid_height / ($metadata.height * scale);
		}
	}
</script>

{#if $annotations}
	<div id="annotation-canvas">
		{#each $annotations as layer}
			<div id={'annotation-layer-' + layer.tag}>
				{#each layer.annotations as coordinates}
					<Annotation
						{coordinates}
						colours={layer.colours}
						{scale_x}
						{scale_y}
						{image_grid_width}
						{image_grid_height}
					/>
				{/each}
			</div>
		{/each}
	</div>
{/if}

<style lang="scss">
	#annotation-canvas {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
	}
</style>
