<script lang="ts">
	import { annotations } from '$states';
	import AnnotationLayerControls from '$ui/AnnotationLayerControls.svelte';
</script>

<div class="panel">
	<div class="outer-container">
		<div class="inner-container" style="border-radius: 10px 10px 0 0;">
			<span class="grey-text"> ANNOTATIONS </span>
		</div>
		{#if annotations.value}
			{#each annotations.value as layer}
				<AnnotationLayerControls {layer} />
			{/each}
		{:else}
			<div style="padding: 10px;">Load an image to control annotations.</div>
		{/if}
	</div>
</div>
