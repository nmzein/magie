<script lang="ts">
	import { image } from '$states';
	import AnnotationLayerControls from '$ui/AnnotationLayerControls.svelte';
</script>

{#if image.initialised}
	<div class="panel">
		<div class="outer-container">
			<div class="inner-container" style="border-radius: 10px 10px 0 0;">
				<span class="grey-text"> ANNOTATIONS </span>
			</div>
			{#each image.annotations as layer}
				<AnnotationLayerControls {layer} />
			{/each}
		</div>
	</div>
{/if}
