<script lang="ts">
	export let name: string;
	// TODO: Add length limit to name before truncating.
</script>

<div>{name} X</div>

<style lang="scss">
	div {
		position: absolute;
		background-color: #5c5c5c;
		border-radius: 10px;
		padding: 5px;
		margin-top: 5px;
		margin-left: 5px;
		font-family: 'JetBrains Mono', monospace;
		cursor: pointer;
	}
</style>
