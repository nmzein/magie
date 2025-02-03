<script lang="ts">
	import { contextMenu, explorer, SelectionBoxState } from '$states';
	import { type Directory, type Image } from '$types';
	import Item from './Item.svelte';
	import DirectoryCreator from './DirectoryCreator.svelte';
	import { BoundingClientRect } from '$actions';

	const selection: SelectionBoxState<Directory | Image> = new SelectionBoxState();

	function onpointerdown(e: PointerEvent) {
		// If main panel not clicked directly, return.
		if ((e.target as HTMLElement)?.id !== 'main') return;

		explorer!.deselectAll();

		// Return if not left click.
		if (e.button !== 0) return;

		selection.start({ x: e.clientX, y: e.clientY });
	}

	function onpointermove(e: PointerEvent) {
		selection.update({ x: e.clientX, y: e.clientY });
	}

	function onpointerup() {
		if (!selection.dragging) return;

		const selected = selection.finish();
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
		selection.parentScroll = {
			// @ts-ignore
			top: e.target?.scrollTop,
			// @ts-ignore
			left: e.target?.scrollLeft
		};
		selection.update();
	}}
	use:BoundingClientRect={(v) => (selection.parentBounds = v)}
	class="@container h-[408px] select-none rounded-br-[10px]
		   {contextMenu.show ? 'overflow-hidden' : 'overflow-auto'}"
	{onpointerdown}
	{oncontextmenu}
>
	<div
		id="main"
		class="@sm:grid-cols-2 @md:grid-cols-3 @lg:grid-cols-4 relative grid min-h-full grid-cols-1 gap-3 p-3"
	>
		{#if explorer!.directoryCreator.show}
			<DirectoryCreator />
		{/if}
		{#each explorer!.directory.data.subdirectories as subdirectory}
			<Item value={subdirectory} {selection} />
		{/each}
		{#each explorer!.directory.data.files as file}
			<Item value={file} {selection} />
		{/each}

		<div class="pointer-events-none absolute inset-0 h-full overflow-clip">
			<div
				bind:this={selection.element}
				class="border-accent bg-accent/20 absolute rounded-md border"
				class:invisible={!selection.show}
			></div>
		</div>
	</div>
</div>
