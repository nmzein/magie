<script lang="ts">
	import { untrack, type Snippet } from 'svelte';
	import { context, type DropdownClasses, DropdownState } from './context.svelte';

	let {
		show = $bindable(false),
		classes,
		children
	}: { show?: boolean; classes: DropdownClasses; children: Snippet } = $props();

	const ctx = context.set(new DropdownState(classes));

	$effect(() => {
		show;
		untrack(() => {
			ctx.show = show;
		});
	});

	$effect(() => {
		ctx.show;
		untrack(() => {
			show = ctx.show;
		});
	});
</script>

<div>
	{@render children()}
</div>
