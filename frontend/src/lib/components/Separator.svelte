<script lang="ts">
	let { margin = 7.5 }: { margin?: number } = $props();
</script>

<div style:--margin={`${margin}px`}></div>

<style lang="scss">
	div {
		width: 100%;
		height: 1px;
		background-color: rgba(168, 168, 168, 0.2);
		margin: var(--margin) 0;
	}
</style>
