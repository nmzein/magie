<script lang="ts">
	import { contextMenu, explorer, SelectionBoxState } from '$states';
	import { type Bounds, type Directory, type Image } from '$types';
	import Item from './Item.svelte';
	import DirectoryCreator from './DirectoryCreator.svelte';
	import { defined } from '$helpers';
	import { boundingclientrect } from '$actions';

	let selectionBoxState: SelectionBoxState<Directory | Image> = new SelectionBoxState();
	let selectionBoxElement: HTMLDivElement | undefined = $state();
	let mainPanelBounds: Bounds | undefined = $state();

	$effect(() => {
		if (mainPanelBounds && selectionBoxElement) {
			selectionBoxState.parentBounds = mainPanelBounds;
			selectionBoxState.element = selectionBoxElement;
		}
	});

	function onpointerdown(event: PointerEvent) {
		explorer.deselectAll();

		// Return if not left click.
		if (event.button !== 0) return;

		selectionBoxState.start({ x: event.clientX, y: event.clientY });
	}

	function onpointermove(event: PointerEvent) {
		selectionBoxState.update({ x: event.clientX, y: event.clientY });
	}

	function onpointerup() {
		if (!selectionBoxState.dragging) return;

		explorer.selected = selectionBoxState.stop();
	}

	function onkeydown(event: KeyboardEvent) {
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
			if (explorer.currentDirectory?.data.id === 1) {
				explorer.deleteSelected('hard');
			} else {
				explorer.deleteSelected('soft');
			}
		} else if (event.shiftKey && event.key === 'Delete') {
			event.preventDefault();
			explorer.deleteSelected('hard');
		}
	}

	function oncontextmenu(event: MouseEvent) {
		event.preventDefault();
		contextMenu.show = true;
		contextMenu.position = { x: event.clientX, y: event.clientY };
		contextMenu.items = [
			{ name: 'Select All', action: () => explorer.selectAll() },
			{ name: 'Paste', action: () => explorer.paste(), disabled: explorer.emptyClipboard },
			{ name: 'New Image', action: () => (explorer.showUploader = true) },
			{ name: 'New Directory', action: () => (explorer.showDirectoryCreator = true) }
		];
	}
</script>

<svelte:document {onkeydown} {onpointermove} {onpointerup} />

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div
	class="relative grid h-[400px] select-none grid-cols-[repeat(4,calc(25%-7.5px))] grid-rows-[repeat(4,1fr)] gap-[10px] px-5 py-[10px] {contextMenu.show
		? 'overflow-hidden'
		: 'overflow-auto'}"
	use:boundingclientrect={(v) => (mainPanelBounds = v)}
	{onpointerdown}
	{oncontextmenu}
>
	{#if defined(explorer.currentDirectory) && defined(selectionBoxState)}
		{#if explorer.showDirectoryCreator}
			<DirectoryCreator />
		{/if}
		{#each explorer.currentDirectory.data.subdirectories as subdirectory}
			<Item value={subdirectory} {selectionBoxState} />
		{/each}
		{#each explorer.currentDirectory.data.files as file}
			<Item value={file} {selectionBoxState} />
		{/each}
	{/if}

	<div
		bind:this={selectionBoxElement}
		class="border-accent bg-accent/20 absolute rounded-[10px] border"
		class:invisible={!selectionBoxState.show}
	></div>
</div>
