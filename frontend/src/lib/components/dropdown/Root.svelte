<script lang="ts">
	import type { Snippet } from 'svelte';
	import { setDropdownState, type DropdownClasses } from './context.svelte';

	let { classes, children }: { classes: DropdownClasses; children: Snippet } = $props();

	setDropdownState(classes);
</script>

<div>
	{@render children()}
</div>
