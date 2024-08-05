<script lang="ts">
	import { explorer } from '$states';
	import Icon from '$icon';

	const ICON_SIZE = 2.5;
</script>

<div class="flex-row">
	<Icon variant="back" width={ICON_SIZE} height={ICON_SIZE} onclick={() => explorer.backward()} />
	<Icon variant="forward" width={ICON_SIZE} height={ICON_SIZE} onclick={() => explorer.forward()} />
	<Icon variant="up" width={ICON_SIZE} height={ICON_SIZE} onclick={() => explorer.up()} />
</div>
