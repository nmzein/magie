<script lang="ts">
	import type { Snippet } from 'svelte';
	import { getTabState } from './context.svelte.ts';
	import Button from '$components/Button.svelte';

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

<Button
	class="
		{tState.classes.trigger.regular}
		{!disabled && tState.currentTab === value ? tState.classes.trigger.active : ''}
		{disabled ? tState.classes.trigger.disabled : ''}
	"
	{disabled}
	onclick={() => {
		if (disabled) return;
		if (sideEffect !== undefined) sideEffect();

		if (tState.mode !== 'buttons' && tState.currentTab !== value) {
			// Only allow enable if mode is 'tab', 'collapsible-tab', or 'toggle'.
			tState.currentTab = value;
		} else if (tState.mode === 'collapsible-tab') {
			// Only allow disable if mode is 'collapsible-tab' ('buttons' auto-disables).
			tState.currentTab = undefined;
		}
	}}
>
	{@render children()}
</Button>
