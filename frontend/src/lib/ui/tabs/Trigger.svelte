<script lang="ts">
	import type { Snippet } from 'svelte';
	import { getTabState } from './context.svelte.ts';

	let {
		value = '',
		sideEffect = undefined,
		disabled = false,
		children
	}: {
		value: string;
		sideEffect: (() => void) | undefined;
		disabled: boolean;
		children: Snippet;
	} = $props();

	let state = getTabState();
</script>

<button
	class="
		{state.classes.trigger.regular}
		{!disabled && state.currentTab === value ? state.classes.trigger.active : ''}
		{disabled ? state.classes.trigger.disabled : ''}
	"
	onclick={() => {
		if (disabled) return;
		if (sideEffect !== undefined) sideEffect();

		if (state.mode !== '0' && state.currentTab !== value) {
			// Only allow toggle if mode is '=1' or '<=1'.
			state.currentTab = value;
		} else if (state.mode === '<=1') {
			// Only allow untoggle if mode is '<=1'.
			state.currentTab = undefined;
		}
	}}
>
	{@render children()}
</button>
