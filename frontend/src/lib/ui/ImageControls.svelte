<script lang="ts">
	import { image } from '$states';
	import AnnotationLayerControls from '$ui/AnnotationLayerControls.svelte';
</script>

<div class="panel">
	<div class="outer-container">
		<div class="inner-container" style="border-radius: 10px 10px 0 0;">
			<span class="grey-text"> ANNOTATIONS </span>
		</div>
		{#if image.initialised}
			{#each image.annotations as layer}
				<AnnotationLayerControls {layer} />
			{/each}
		{:else}
			<div style="padding: 10px;">Load an image to control annotations.</div>
		{/if}
	</div>
</div>
