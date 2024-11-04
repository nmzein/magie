<script lang="ts">
	import type { Snippet } from 'svelte';
	import { setTabListState } from './context.svelte.js';

	let { id = '', children }: { id?: string; children: Snippet } = $props();

	setTabListState(id);
</script>

{@render children()}
