<script lang="ts">
	import { explorer, SelectionBox } from '$states';
	import { DEFAULT_POINT, type Bounds, type Directory, type Image } from '$types';
	import Item from './Item.svelte';
	import DirectoryCreator from './DirectoryCreator.svelte';
	import { defined } from '$helpers';
	import { boundingclientrect } from '$actions';
	import * as Dropdown from '$components/dropdown';
	import ContextMenu from './ContextMenu.svelte';

	const selectionBox: SelectionBox<Directory | Image> = new SelectionBox();
	let selectionBoxElement: HTMLDivElement | undefined = $state();
	let mainPanelBounds: Bounds | undefined = $state();
	let contextMenu = $state({
		show: false,
		position: DEFAULT_POINT
	});

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

		// Return if not left click.
		if (!defined(selectionBox) || event.button !== 0) return;

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

{#if defined(mainPanelBounds)}
	<ContextMenu
		bind:show={contextMenu.show}
		position={contextMenu.position}
		parentBounds={mainPanelBounds}
	/>
{/if}

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div
	class="relative grid h-[400px] grid-cols-[repeat(4,calc(25%-7.5px))] grid-rows-[repeat(4,1fr)] gap-[10px] px-5 py-[10px]"
	use:boundingclientrect={(v) => (mainPanelBounds = v)}
	onmousedown={handleMouseDown}
	oncontextmenu={(e) => {
		e.preventDefault();
		contextMenu = {
			show: true,
			position: {
				x: e.clientX,
				y: e.clientY
			}
		};
	}}
>
	{#if defined(explorer.currentDirectory) && defined(selectionBox)}
		{#if explorer.showDirectoryCreator}
			<DirectoryCreator />
		{/if}
		{#each explorer.currentDirectory.data.subdirectories as subdirectory}
			<Item value={subdirectory} {selectionBox} />
		{/each}
		{#each explorer.currentDirectory.data.files as file}
			<Item value={file} {selectionBox} />
		{/each}
	{/if}

	<div
		bind:this={selectionBoxElement}
		class="border-accent bg-accent/20 absolute rounded-[10px] border"
		class:invisible={!selectionBox?.dragging}
	></div>
</div>
