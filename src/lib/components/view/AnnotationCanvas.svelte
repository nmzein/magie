<script lang="ts">
	import { annotations, metadata } from '$lib/stores';
	import Annotation from '$lib/components/view/Annotation.svelte';
</script>

{#if $annotations && $metadata}
	<div id="annotation-canvas">
		{#if document.getElementsByClassName('image-grid')}
			{#each $annotations as layer, layer_index}
				<div id={'annotation-layer-' + layer.tag}>
					{#each layer.annotations as coordinates, annotation_index}
						<Annotation
							{coordinates}
							id={'annotation-' + layer_index + '-' + annotation_index}
							colours={layer.colours}
						/>
					{/each}
				</div>
			{/each}
		{/if}
	</div>
{/if}

<style lang="scss">
	div {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
	}
</style>
