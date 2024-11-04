<script lang="ts">
	import { untrack, type Snippet } from 'svelte';
	import { getTabState, setTabState, type Modes, type TabClasses } from './context.svelte.ts';

	let {
		id = '',
		currentTab = $bindable(),
		mode,
		classes,
		children
	}: {
		id?: string;
		currentTab?: string;
		mode?: Modes;
		classes?: TabClasses;
		children: Snippet;
	} = $props();

	setTabState(id, mode, currentTab, classes);

	let tState = getTabState(id);

	$effect(() => {
		tState.currentTab; // Triggers the effect when tState.currentTab changes.
		untrack(() => {
			currentTab = tState.currentTab;
		});
	});

	$effect(() => {
		classes;
		untrack(() => {
			tState.classes = classes;
		});
	});
</script>

{@render children()}
