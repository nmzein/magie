<script lang="ts">
	import { image, metadata } from '$lib/stores';
</script>

<div id="image-grid" style="--no-of-columns: {$metadata?.cols};">
	{#each $image as tile, index (tile.src)}
		<img src={tile.src} alt="Image {index}" />
	{/each}
</div>

<style lang="scss">
	#image-grid {
		display: grid;
		grid-template-columns: repeat(var(--no-of-columns), 1fr);
	}

	img {
		width: 100%;
		height: auto;
		// Prevent image selection when dragging.
		user-select: none;
		object-fit: cover;
	}
</style>
