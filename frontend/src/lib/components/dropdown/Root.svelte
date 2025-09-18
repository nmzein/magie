<script lang="ts">
	import { untrack, type Snippet } from 'svelte';
	import { context, type DropdownClasses, DropdownState } from './context.svelte';
	import { onClickOutside } from 'runed';

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

	onClickOutside(
		() => ctx.listElement,
		() => ctx.close()
	);
</script>

<svelte:window
	onpointerdown={(e) => {
		if (e.button === 2) {
			show = false;
		}
	}}
	onkeydown={(event) => {
		if (event.key === 'Escape' || event.key === 'Enter') {
			show = false;
		}
	}}
/>

<div>
	{@render children()}
</div>
