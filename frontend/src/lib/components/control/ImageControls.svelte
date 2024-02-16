<script lang="ts">
	import { annotations } from '$lib/stores';
	import AnnotationVisibilityControls from '$lib/components/control/AnnotationVisibilityControls.svelte';
</script>

<div style="display: flex; flex-direction: column; gap: 15px;">
	<div class="outer-container">
		<div class="inner-container">
			<div style="display: flex; flex-direction: column; gap: 10px;">
				<span class="grey-heading"> ANNOTATIONS </span>
				{#if $annotations}
					{#each $annotations as annotation}
						<AnnotationVisibilityControls {annotation} />
					{/each}
				{:else}
					<!-- Turn page controller to store so link here can auto switch pages. -->
					Load an image to control annotations.
				{/if}
			</div>
		</div>
	</div>
</div>
