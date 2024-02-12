<script lang="ts">
	export let name: string;
	export let showFiles = false;
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<span class="grey-heading" on:click={() => (showFiles = !showFiles)}>{name}</span>

<div>
	{#if showFiles}
		<slot />
	{/if}
</div>

<style lang="scss">
	.grey-heading {
		color: rgba(255, 255, 255, 0.6);
		cursor: pointer;
		user-select: none;
	}

	div {
		padding-left: 10px;
		display: flex;
		flex-direction: column;
	}
</style>
