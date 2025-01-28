<script lang="ts">
	import NavigationButtons from './NavigationButtons.svelte';
	import AddressBar from './AddressBar.svelte';
	import Search from './Search.svelte';
	import { defined } from '$helpers';
	import type { Bounds } from '$types';

	let {
		offsetX = $bindable(),
		offsetY = $bindable(),
		contentSpaceBounds,
		explorerBounds
	}: {
		offsetX: number;
		offsetY: number;
		contentSpaceBounds: Bounds | undefined;
		explorerBounds: Bounds | undefined;
	} = $props();

	let isDragging = false;
	let startX: number | undefined = $state();
	let startY: number | undefined = $state();

	function onmousedown(event: MouseEvent) {
		isDragging = true;
		startX = event.clientX - offsetX;
		startY = event.clientY - offsetY;
	}

	function onmousemove(event: MouseEvent) {
		if (
			!isDragging ||
			!defined(startX) ||
			!defined(startY) ||
			!defined(contentSpaceBounds) ||
			!defined(explorerBounds)
		)
			return;

		// Calculate the new position
		let newOffsetX = event.clientX - startX;
		let newOffsetY = event.clientY - startY;

		// Clamp X position
		newOffsetX = Math.max(0, Math.min(newOffsetX, contentSpaceBounds.width - explorerBounds.width));

		// Clamp Y position
		newOffsetY = Math.max(
			0,
			Math.min(newOffsetY, contentSpaceBounds.height - explorerBounds.height)
		);

		// Update the offsets
		offsetX = newOffsetX;
		offsetY = newOffsetY;
	}

	function onmouseup() {
		isDragging = false;
	}
</script>

<svelte:window {onmousemove} {onmouseup} />

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
	class="border-b-secondary/20 bg-primary/15 flex flex-row items-center gap-[10px] rounded-t-[inherit] border-b p-[10px] active:cursor-grabbing"
	{onmousedown}
>
	<NavigationButtons />
	<AddressBar />
	<Search />
</div>
