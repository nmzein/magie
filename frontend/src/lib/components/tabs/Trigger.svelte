<script lang="ts">
	import type { Snippet } from 'svelte';
	import { getTabState } from './context.svelte.ts';

	let {
		value = '',
		sideEffect = undefined,
		disabled = false,
		children
	}: {
		value?: string;
		sideEffect?: () => void;
		disabled?: boolean;
		children: Snippet;
	} = $props();

	let tState = getTabState();
</script>

<button
	class="
		{tState.classes.trigger.regular}
		{!disabled && tState.currentTab === value ? tState.classes.trigger.active : ''}
		{disabled ? tState.classes.trigger.disabled : ''}
	"
	onclick={() => {
		if (disabled) return;
		if (sideEffect !== undefined) sideEffect();

		if (tState.mode !== '0' && tState.currentTab !== value) {
			// Only allow toggle if mode is '=1' or '<=1'.
			tState.currentTab = value;
		} else if (tState.mode === '<=1') {
			// Only allow untoggle if mode is '<=1'.
			tState.currentTab = undefined;
		}
	}}
>
	{@render children()}
</button>
