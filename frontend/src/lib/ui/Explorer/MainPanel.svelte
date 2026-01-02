<script lang="ts">
	import { clipboard, contextMenu } from '$states';
	import Item from './Item.svelte';
	import DirectoryCreator from './DirectoryCreator.svelte';
	import { boundingClientRect } from '$attachments';
	import { defined } from '$helpers';
	import { context, BIN_ID } from './context.svelte.ts';
	import SelectionBox from '$ui/SelectionBox/SelectionBox.svelte';
	import type { SelectionBoxState } from '$ui/SelectionBox/state.svelte.ts';

	const explorer = context.get();
	let mainPanel: HTMLDivElement | undefined = $state();
	let selection: SelectionBoxState<number> | undefined = $state();

	function onpointerdown(e: PointerEvent) {
		// If main panel not clicked directly, return.
		if ((e.target as HTMLDivElement)?.id !== 'main') return;

		explorer.deselectAll();

		// Return if not left click.
		if (e.button !== 0) return;

		selection?.start({ x: e.clientX, y: e.clientY });
	}

	function onkeydown(e: KeyboardEvent) {
		if (e.ctrlKey) {
			switch (e.key) {
				case 'l':
					e.preventDefault();
					explorer.uploader.open();
					break;
				case 'L':
					e.preventDefault();
					explorer.directoryCreator.open();
					break;
				case 'a':
					e.preventDefault();
					explorer.selectAll();
					break;
				case 'p':
					e.preventDefault();
					explorer.pinSelected();
					break;
				case 'u':
					e.preventDefault();
					explorer.unpinSelected();
					break;
				case 'x':
					e.preventDefault();
					explorer.clipSelected('cut');
					break;
				case 'c':
					e.preventDefault();
					explorer.clipSelected('copy');
					break;
				case 'v':
					e.preventDefault();
					if (!explorer.inBin) {
						explorer.paste();
					} else {
						// Display error notification.
					}
					break;
			}
		} else if (!e.shiftKey && e.key === 'Delete') {
			e.preventDefault();
			// If delete in bin then hard delete.
			if (explorer.inBin) {
				explorer.deleteSelected('hard');
			} else {
				explorer.deleteSelected('soft');
			}
		} else if (e.shiftKey && e.key === 'Delete') {
			e.preventDefault();
			explorer.deleteSelected('hard');
		}
	}

	function oncontextmenu(e: MouseEvent) {
		e.preventDefault();
		contextMenu.open({ x: e.clientX, y: e.clientY }, [
			{
				name: 'New Asset',
				action: () => explorer.uploader.open(),
				hidden: explorer.inBin,
				shortcut: 'Ctrl+L'
			},
			{
				name: 'New Folder',
				action: () => explorer.directoryCreator.open(),
				hidden: explorer.inBin,
				shortcut: 'Shift+Ctrl+L'
			},
			'separator',
			{
				name: 'Paste',
				action: () => explorer.paste(),
				disabled: clipboard.isEmpty,
				hidden: explorer.inBin,
				shortcut: 'Ctrl+V'
			},
			{ name: 'Select All', action: () => explorer.selectAll(), shortcut: 'Ctrl+A' }
		]);
	}
</script>

<svelte:document {onkeydown} />

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
	bind:this={mainPanel}
	onscroll={(e) => {
		if (!selection) return;
		let target = e.target as HTMLElement;

		selection.parentScroll = {
			top: target.scrollTop,
			left: target.scrollLeft
		};
		selection.update();
	}}
	{@attach boundingClientRect((rect) => selection && (selection.parentBounds = rect))}
	class="@container h-102 rounded-br-[10px] select-none
           {contextMenu.show ? 'overflow-hidden' : 'overflow-auto'}"
	{onpointerdown}
	{oncontextmenu}
>
	<div
		id="main"
		class="relative grid min-h-full w-full grid-cols-1 content-start gap-3 p-3 @sm:grid-cols-2 @md:grid-cols-3 @lg:grid-cols-4"
	>
		{#if explorer.directoryCreator.show}
			<DirectoryCreator />
		{/if}
		{#each explorer.items as id (id)}
			{@const item = explorer.get(id)}
			{#if id !== BIN_ID && defined(item) && defined(selection)}
				<Item {item} {selection} />
			{/if}
		{/each}

		<div class="pointer-events-none absolute inset-0 h-full overflow-clip">
			<SelectionBox
				bind:selection
				element={mainPanel}
				onselected={(selected: number[]) => explorer.selectGroup(selected)}
			/>
		</div>
	</div>
</div>
