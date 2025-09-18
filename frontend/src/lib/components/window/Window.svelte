<script lang="ts">
	import type { Bounds, Point } from '$types';
	import { defined } from '$helpers';
	import { untrack, type Snippet } from 'svelte';
	import { ResizeObserver } from '$actions';
	import { context } from './context.svelte.ts';

	let { contentSpaceBounds, children }: { contentSpaceBounds: Bounds; children: Snippet } =
		$props();

	$effect(() => {
		if (!defined(bounds)) return;

		contentSpaceBounds.width;
		contentSpaceBounds.height;

		untrack(() => {
			position = {
				x: 0.5 * (contentSpaceBounds.width - bounds!.width),
				y: 0.3 * (contentSpaceBounds.height - bounds!.height)
			};
		});
	});

	let bounds: Bounds | undefined = $state();
	let position: Point = $state({ x: -1, y: -1 });

	let isDragging = false;
	let startX: number | undefined = $state();
	let startY: number | undefined = $state();

	function onmousedown(event: MouseEvent) {
		isDragging = true;
		startX = event.clientX - position.x;
		startY = event.clientY - position.y;
	}

	function onmousemove(event: MouseEvent) {
		if (
			!isDragging ||
			!defined(startX) ||
			!defined(startY) ||
			!defined(contentSpaceBounds) ||
			!defined(bounds)
		)
			return;

		// Calculate the new position
		let newOffsetX = event.clientX - startX;
		let newOffsetY = event.clientY - startY;

		// Clamp X position
		newOffsetX = Math.max(0, Math.min(newOffsetX, contentSpaceBounds.width - bounds.width));

		// Clamp Y position
		newOffsetY = Math.max(0, Math.min(newOffsetY, contentSpaceBounds.height - bounds.height));

		// Update the offsets
		position.x = newOffsetX;
		position.y = newOffsetY;
	}

	function onmouseup() {
		isDragging = false;
	}

	let ctx = $state({ drag: onmousedown });
	context.set(ctx);
</script>

<svelte:window {onmousemove} {onmouseup} />

<div
	class="h-fit max-h-full w-fit max-w-full"
	style="transform: translate({position.x}px, {position.y}px);"
	use:ResizeObserver={(v) => (bounds = v)}
>
	{@render children()}
</div>
