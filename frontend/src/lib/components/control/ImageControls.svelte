<script lang="ts">
	import { annotations } from '$stores';
	import AnnotationVisibilityControls from '$control/AnnotationVisibilityControls.svelte';
</script>

<div class="outer-container">
	<div class="inner-container">
		<div style="display: flex; flex-direction: column; gap: 10px;">
			<span class="secondary-text"> ANNOTATIONS </span>
			{#if annotations.value}
				{#each annotations.value as annotation}
					<AnnotationVisibilityControls {annotation} />
				{/each}
			{:else}
				Load an image to control annotations.
			{/if}
		</div>
	</div>
</div>
