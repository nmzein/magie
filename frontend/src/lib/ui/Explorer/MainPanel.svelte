<script lang="ts">
	import { clipboard, contextMenu, explorer, SelectionBoxState } from '$states';
	import Item from './Item.svelte';
	import DirectoryCreator from './DirectoryCreator.svelte';
	import { BoundingClientRect } from '$actions';
	import { defined } from '$helpers';
	import { BIN_ID } from '$lib/states/explorer.svelte';

	const selection: SelectionBoxState<number> = new SelectionBoxState();
	let mainPanel: HTMLElement | undefined = $state();

	function autoscroll(e: PointerEvent) {
		const SCROLL_SPEED = 10;
		const SCROLL_THRESHOLD = 50; // # of pixels from edge to trigger scroll.

		if (!selection.dragging || !defined(selection.parentBounds) || !defined(mainPanel)) return;

		// Scroll down.
		if (e.clientY > selection.parentBounds.bottom - SCROLL_THRESHOLD) {
			mainPanel.scrollTop += SCROLL_SPEED;
			selection.update({ x: e.clientX, y: e.clientY });
		}
		// Scroll up.
		else if (e.clientY < selection.parentBounds.top + SCROLL_THRESHOLD) {
			mainPanel.scrollTop -= SCROLL_SPEED;
			selection.update({ x: e.clientX, y: e.clientY });
		}
	}

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
		autoscroll(e);
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
			if (explorer!.inBin) {
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
			{ name: 'Paste', action: () => explorer!.paste(), disabled: clipboard.isEmpty },
			{ name: 'New Image', action: () => explorer!.uploader.open() },
			{ name: 'New Directory', action: () => explorer!.directoryCreator.open() }
		];
	}
</script>

<svelte:document {onkeydown} {onpointermove} {onpointerup} />

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
	bind:this={mainPanel}
	onscroll={(e) => {
		let target = e.target as HTMLElement;

		selection.parentScroll = {
			top: target.scrollTop,
			left: target.scrollLeft
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
		{#key explorer!.directoryId}
			{#each explorer!.directory.children as id}
				{@const item = explorer!.get(id)}
				{#if id !== BIN_ID && defined(item)}
					<Item {item} {selection} />
				{/if}
			{/each}
		{/key}

		<div class="pointer-events-none absolute inset-0 h-full overflow-clip">
			<div
				bind:this={selection.element}
				class="border-accent bg-accent/20 absolute rounded-md border"
				class:invisible={!selection.show}
			></div>
		</div>
	</div>
</div>
