<script lang="ts">
	import type { Snippet } from 'svelte';
	import { setTabState, type Modes, type TabClasses } from './context.svelte.ts';

	let {
		mode,
		classes,
		initialTab,
		children
	}: { mode?: Modes; classes?: TabClasses; initialTab?: string; children: Snippet } = $props();

	setTabState(mode, initialTab, classes);
</script>

{@render children()}
