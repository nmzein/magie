<script lang="ts">
	import { explorer } from '$states';
	import Icon from '$icon';

	const ICON_SIZE = 2;
</script>

<div class="light-layer">
	Uploader
	<button onclick={() => (explorer.showUploader = false)}>
		<Icon variant="close" width={ICON_SIZE} height={ICON_SIZE} />
	</button>
</div>

<style lang="scss">
	div {
		display: flex;
		justify-content: center;
		align-items: center;
		font-weight: 600;
		font-size: 17px;
		padding: 15px;
		border-top-left-radius: 8px;
		border-top-right-radius: 8px;
		user-select: none;
	}

	button {
		border-radius: 50%;
		height: 2rem;
		width: 2rem;
		position: absolute;
		right: 10px;
	}
</style>
