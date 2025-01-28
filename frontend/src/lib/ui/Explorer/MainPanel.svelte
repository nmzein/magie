<script lang="ts">
	import { contextMenu, explorer, SelectionBoxState } from '$states';
	import { type Directory, type Image } from '$types';
	import Item from './Item.svelte';
	import DirectoryCreator from './DirectoryCreator.svelte';
	import { BoundingClientRect } from '$actions';

	let selectionBoxState: SelectionBoxState<Directory | Image> = new SelectionBoxState();

	function onpointerdown(e: PointerEvent) {
		explorer!.deselectAll();

		// Return if not left click.
		if (e.button !== 0) return;

		selectionBoxState.start({ x: e.clientX, y: e.clientY });
	}

	function onpointermove(e: PointerEvent) {
		selectionBoxState.update({ x: e.clientX, y: e.clientY });
	}

	function onpointerup() {
		if (!selectionBoxState.dragging) return;

		const selected = selectionBoxState.stop();
		explorer!.selectGroup(selected);
	}

	function onkeydown(e: KeyboardEvent) {
		if (e.ctrlKey) {
			switch (e.key) {
				case 'a':
					e.preventDefault();
					explorer!.selectAll();
					break;
				case 'p':
					e.preventDefault();
					explorer!.pinSelected();
					break;
				case 'x':
					e.preventDefault();
					explorer!.clipSelected('cut');
					break;
				case 'c':
					e.preventDefault();
					explorer!.clipSelected('copy');
					break;
				case 'v':
					e.preventDefault();
					explorer!.paste();
					break;
			}
		} else if (!e.shiftKey && e.key === 'Delete') {
			e.preventDefault();
			// If delete in bin then hard delete.
			if (explorer!.directory.data.id === 1) {
				explorer!.deleteSelected('hard');
			} else {
				explorer!.deleteSelected('soft');
			}
		} else if (e.shiftKey && e.key === 'Delete') {
			e.preventDefault();
			explorer!.deleteSelected('hard');
		}
	}

	function oncontextmenu(e: MouseEvent) {
		e.preventDefault();
		contextMenu.show = true;
		contextMenu.position = { x: e.clientX, y: e.clientY };
		contextMenu.items = [
			{ name: 'Select All', action: () => explorer!.selectAll() },
			{ name: 'Paste', action: () => explorer!.paste(), disabled: explorer!.clipboardIsEmpty },
			{ name: 'New Image', action: () => explorer!.uploader.open() },
			{ name: 'New Directory', action: () => explorer!.directoryCreator.open() }
		];
	}
</script>

<svelte:document {onkeydown} {onpointermove} {onpointerup} />

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
	onscroll={(e) => {
		selectionBoxState.parentScroll = {
			// @ts-ignore
			top: e.target?.scrollTop,
			// @ts-ignore
			left: e.target?.scrollLeft
		};
		selectionBoxState.update();
	}}
	use:BoundingClientRect={(v) => (selectionBoxState.parentBounds = v)}
	class="@container relative h-[400px] select-none p-3 {contextMenu.show
		? 'overflow-hidden'
		: 'overflow-auto'}"
	{onpointerdown}
	{oncontextmenu}
>
	<div class="@sm:grid-cols-2 @md:grid-cols-3 @lg:grid-cols-4 grid grid-cols-1 gap-3">
		{#if explorer!.directoryCreator.show}
			<DirectoryCreator />
		{/if}
		{#each explorer!.directory.data.subdirectories as subdirectory}
			<Item value={subdirectory} {selectionBoxState} />
		{/each}
		{#each explorer!.directory.data.files as file}
			<Item value={file} {selectionBoxState} />
		{/each}
	</div>

	<div
		bind:this={selectionBoxState.element}
		class="border-accent bg-accent/20 absolute rounded-md border"
		class:invisible={!selectionBoxState.show}
	></div>
</div>
