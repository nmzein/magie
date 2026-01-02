<script lang="ts" generics="T">
	import { defined } from '$helpers';
	import { SelectionBoxState } from './state.svelte.ts';

	type Props = {
		selection?: SelectionBoxState<T>;
		element: HTMLElement;
		onboundschange?: (e: PointerEvent) => void;
		onselected?: (selected: T[]) => void;
	};

	let { selection = $bindable(), element, onboundschange, onselected }: Props = $props();

	selection = new SelectionBoxState();

	function autoscroll(e: PointerEvent) {
		if (
			!defined(selection) ||
			!selection.dragging ||
			!defined(selection.parentBounds) ||
			!defined(element)
		)
			return;

		const SCROLL_SPEED = 10;
		const SCROLL_THRESHOLD = 50; // # of pixels from edge to trigger scroll.

		// Scroll down.
		if (e.clientY > selection.parentBounds.bottom - SCROLL_THRESHOLD) {
			element.scrollTop += SCROLL_SPEED;
			selection.update({ x: e.clientX, y: e.clientY });
		}
		// Scroll up.
		else if (e.clientY < selection.parentBounds.top + SCROLL_THRESHOLD) {
			element.scrollTop -= SCROLL_SPEED;
			selection.update({ x: e.clientX, y: e.clientY });
		}
	}

	function onpointermove(e: PointerEvent) {
		if (!defined(selection)) return;
		selection.update({ x: e.clientX, y: e.clientY });
		autoscroll(e);
		onboundschange?.(e);
	}

	function onpointerup() {
		if (!selection?.dragging) return;
		const selected = selection.finish();
		onselected?.(selected);
	}
</script>

<svelte:document {onkeydown} {onpointermove} {onpointerup} />

<div
	bind:this={selection.element}
	class="border-accent bg-accent/20 absolute rounded-md border"
	class:invisible={!selection.show}
></div>
