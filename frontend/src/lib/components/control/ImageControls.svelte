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

<style lang="scss">
	.grey-heading {
		color: rgba(255, 255, 255, 0.6);
	}

	.outer-container {
		backdrop-filter: blur(15px);
		background: rgba(255, 255, 255, 0.15);
		box-shadow: 0 15px 15px rgba(0, 0, 0, 0.1);
		font-size: 14px;
		border-radius: 10px;
		font-family: 'JetBrains Mono', monospace;
	}

	.inner-container {
		background-color: rgba(0, 0, 0, 0.2);
		padding: 10px;
		border-radius: 10px;
		display: flex;
		flex-direction: column;
		gap: 10px;
	}

	// button {
	// 	font-family: 'JetBrains Mono', monospace;
	// 	font-size: 14px;
	// 	background-color: rgba(255, 255, 255, 0.2);
	// 	border-radius: 8px;
	// 	margin-top: 8px;
	// 	width: 70px;

	// 	&:hover {
	// 		background-color: rgba(255, 255, 255, 0.15);
	// 	}
	// }

	// .divider {
	// 	position: absolute;
	// 	height: 30px;
	// 	top: 10%;
	// 	left: -2%;
	// 	z-index: 1;
	// 	border-left: 2px solid rgba(255, 255, 255, 0.2);
	// 	pointer-events: none;
	// }
</style>
