<script lang="ts">
	import Back from '~icons/material-symbols-light/chevron-left-rounded';
	import Forward from '~icons/material-symbols-light/chevron-right-rounded';
	import Up from '~icons/material-symbols-light/keyboard-arrow-up-rounded';
	import { directoryStack } from './state.svelte';

	const ICON_SIZE = '2.5em';
</script>

<div class="flex-row">
	<button>
		<Back width={ICON_SIZE} height={ICON_SIZE} />
	</button>
	<button>
		<Forward width={ICON_SIZE} height={ICON_SIZE} />
	</button>
	<button
		onclick={() => {
			if (directoryStack.value.length > 1) {
				directoryStack.value.pop();
			}
		}}
	>
		<Up width={ICON_SIZE} height={ICON_SIZE} />
	</button>
</div>

<style lang="scss">
	button {
		height: 2.5em;
		width: 2.5em;
	}
</style>
