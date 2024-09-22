<script lang="ts">
	import type { Snippet } from 'svelte';
	import { getTabState, setTabListState } from './context.svelte.ts';

	let { id = '', children }: { id?: string; children: Snippet } = $props();

	let tState = getTabState(id);
	setTabListState(id);
</script>

<div class={tState.classes?.list}>
	{@render children()}
</div>
