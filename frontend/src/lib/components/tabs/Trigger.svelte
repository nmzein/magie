<script lang="ts">
	import type { Snippet } from 'svelte';
	import { getTabState, getTabListState } from './context.svelte.ts';
	import Button from '$components/Button.svelte';
	import { twMerge } from 'tailwind-merge';

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

	let data = getTabListState();
	let tState = getTabState(data.id);
</script>

<Button
	class="
		{tState.classes?.trigger?.base}
		{tState.currentTab !== value ? tState.classes?.trigger?.inactive : ''}
		{!disabled && tState.currentTab === value && tState.mode !== 'buttons'
		? tState.classes?.trigger?.active
		: ''}
		{disabled ? twMerge(tState.classes?.trigger?.disabled, 'opacity-30 grayscale') : ''}
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
