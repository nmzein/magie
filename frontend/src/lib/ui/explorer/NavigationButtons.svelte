<script lang="ts">
	import { explorer } from '$states';

	import Back from '~icons/material-symbols-light/chevron-left-rounded';
	import Forward from '~icons/material-symbols-light/chevron-right-rounded';
	import Up from '~icons/material-symbols-light/keyboard-arrow-up-rounded';

	const ICON_SIZE = '2.5em';
</script>

<div class="flex-row">
	<button onclick={() => explorer.backward()}>
		<Back width={ICON_SIZE} height={ICON_SIZE} />
	</button>
	<button onclick={() => explorer.forward()}>
		<Forward width={ICON_SIZE} height={ICON_SIZE} />
	</button>
	<button onclick={() => explorer.up()}>
		<Up width={ICON_SIZE} height={ICON_SIZE} />
	</button>
</div>

<style lang="scss">
	button {
		height: 2.5em;
		width: 2.5em;
	}
</style>
