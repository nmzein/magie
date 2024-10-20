<script lang="ts">
	import { untrack, type Snippet } from 'svelte';
	import { getDropdownState, setDropdownState, type DropdownClasses } from './context.svelte';

	let {
		show = $bindable(false),
		classes,
		children
	}: { show?: boolean; classes: DropdownClasses; children: Snippet } = $props();

	setDropdownState(classes);
	let dState = getDropdownState();

	$effect(() => {
		show;
		untrack(() => {
			dState.show = show;
		});
	});

	$effect(() => {
		dState.show;
		untrack(() => {
			show = dState.show;
		});
	});
</script>

<div>
	{@render children()}
</div>
