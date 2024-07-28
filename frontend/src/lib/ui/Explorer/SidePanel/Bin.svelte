<script lang="ts">
	import Icon from '$components/Icon.svelte';
	import { explorer } from '$states';
</script>

<!-- Directory with id of 1 is always set to the global bin. -->
<button onclick={() => explorer.routeTo([1])}>
	<Icon variant="trash" width={1.5} height={1.5} />
	Bin
</button>

<style lang="scss">
	button {
		background-color: rgba(255, 255, 255, 0.1);
		width: 100%;
		gap: 10px;
		display: flex;
		align-items: center;
		text-align: left;
		padding: 7.5px 10px;
		border-radius: 7px;

		&:hover {
			background-color: rgba(255, 255, 255, 0.15);
		}
	}
</style>
