<script lang="ts">
	import { defined } from '$helpers';
	import { image } from '$states';
	import AnnotationLayerControls from '$ui/AnnotationLayerControls.svelte';
</script>

<div class="panel">
	<div class="outer-container">
		<div class="inner-container" style="border-radius: 10px 10px 0 0;">
			<span class="text-secondary"> ANNOTATIONS </span>
		</div>
		{#if defined(image.properties)}
			{#each image.properties.annotations as layer}
				<AnnotationLayerControls {layer} />
			{/each}
		{/if}
	</div>
</div>

<style>
	.outer-container {
		backdrop-filter: blur(15px);
		background: rgba(255, 255, 255, 0.15);
		box-shadow: 0 15px 15px rgba(0, 0, 0, 0.1);

		font-size: 14px;
		border-radius: 10px;
	}

	.inner-container {
		background-color: rgba(0, 0, 0, 0.2);
		padding: 10px;
		border-radius: 10px;

		display: flex;
		flex-direction: column;
		gap: 10px;
	}
</style>
