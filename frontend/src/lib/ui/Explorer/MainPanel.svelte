<script lang="ts">
	import { explorer, SelectionBox } from '$states';
	import type { Bounds, Directory, Image } from '$types';
	import Item from './Item.svelte';
	import DirectoryCreator from './DirectoryCreator.svelte';
	import { defined } from '$helpers';

	let selectionBoxElement: HTMLDivElement | undefined = $state();

	let mainPanelBounds: Bounds | undefined = $state();
	function resizeobserver(element: HTMLElement) {
		function update() {
			mainPanelBounds = element.getBoundingClientRect();
		}

		const observer = new ResizeObserver(update);

		observer.observe(element);

		update();

		return {
			destroy() {
				observer.unobserve(element);
			}
		};
	}

	function positionobserver(element: HTMLElement) {
		let lastPosition = { x: 0, y: 0 };

		function update() {
			const rect = element.getBoundingClientRect();
			const newPosition = { x: rect.x, y: rect.y };

			// Check if the position has changed
			if (newPosition.x !== lastPosition.x || newPosition.y !== lastPosition.y) {
				lastPosition = newPosition;
				mainPanelBounds = rect; // Update your reactive value
			}
		}

		// Use requestAnimationFrame to continuously check for changes
		function loop() {
			update();
			requestAnimationFrame(loop); // Keep checking on each frame
		}

		loop(); // Start the loop on mount

		return {
			destroy() {
				// Cleanup if necessary
			}
		};
	}

	const selectionBox: SelectionBox<Directory | Image> = new SelectionBox();

	$effect(() => {
		if (mainPanelBounds || selectionBoxElement) {
			selectionBox.parentBounds = mainPanelBounds;
			selectionBox.selectionBox = selectionBoxElement;
		}
	});

	$effect(() => {
		document.addEventListener('keydown', handleKeyDown);
		document.addEventListener('mousemove', handleMouseMove);
		document.addEventListener('mouseup', handleMouseUp);

		return () => {
			document.removeEventListener('keydown', handleKeyDown);
			document.removeEventListener('mousemove', handleMouseMove);
			document.removeEventListener('mouseup', handleMouseUp);
		};
	});

	function handleMouseDown(event: MouseEvent) {
		explorer.deselectAll();

		if (!defined(selectionBox)) return;

		selectionBox.start({ x: event.clientX, y: event.clientY });
	}

	function handleMouseMove(event: MouseEvent) {
		if (!defined(selectionBox)) return;

		selectionBox.update({ x: event.clientX, y: event.clientY });
	}

	function handleMouseUp() {
		if (!defined(selectionBox) || !selectionBox.dragging) return;

		explorer.selected = selectionBox.stop();
	}

	function handleKeyDown(event: KeyboardEvent) {
		if (event.ctrlKey && event.key === 'p') {
			event.preventDefault();
			explorer.pinSelected();
		} else if (event.ctrlKey && event.key === 'x') {
			event.preventDefault();
			explorer.clipSelected('cut');
		} else if (event.ctrlKey && event.key === 'c') {
			event.preventDefault();
			explorer.clipSelected('copy');
		} else if (event.ctrlKey && event.key === 'v') {
			event.preventDefault();
			explorer.paste();
		} else if (!event.shiftKey && event.key === 'Delete') {
			event.preventDefault();
			// If delete in bin then hard delete.
			if (explorer.currentRoute[0] === 1) {
				explorer.deleteSelected('hard');
			} else {
				explorer.deleteSelected('soft');
			}
		} else if (event.shiftKey && event.key === 'Delete') {
			event.preventDefault();
			explorer.deleteSelected('hard');
		}
	}
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
	class="relative grid h-[400px] grid-cols-[repeat(4,calc(25%-7.5px))] grid-rows-[repeat(4,1fr)] gap-[10px] px-5 py-[10px]"
	use:resizeobserver
	use:positionobserver
	onmousedown={handleMouseDown}
>
	{#if defined(explorer.currentDirectory) && defined(selectionBox)}
		{#each explorer.currentDirectory.data.subdirectories as subdirectory}
			<Item value={subdirectory} {selectionBox} />
		{/each}
		{#each explorer.currentDirectory.data.files as file}
			<Item value={file} {selectionBox} />
		{/each}
		{#if explorer.showDirectoryCreator}
			<DirectoryCreator />
		{/if}
	{/if}

	<div
		bind:this={selectionBoxElement}
		class="border-accent bg-accent/20 absolute rounded-[10px] border"
		class:invisible={!selectionBox?.dragging}
	></div>
</div>
