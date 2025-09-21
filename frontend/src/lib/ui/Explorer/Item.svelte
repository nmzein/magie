<script lang="ts">
	import type { Bounds, Directory, Asset } from '$types';
	import { type SelectionBoxState, contextMenu } from '$states';
	import { defined } from '$helpers';
	import Icon from '$icon';
	import { http } from '$api';
	import { twMerge } from 'tailwind-merge';
	import { BoundingClientRect } from '$actions';
	import { context } from './context.svelte.ts';

	const explorer = context.get();

	let { item, selection }: { item: Directory | Asset; selection: SelectionBoxState } = $props();

	let itemBounds: Bounds | undefined = $state();
	let intersected = $state(false);

	$effect(() => {
		intersected = defined(itemBounds) && selection.intersecting(itemBounds, item.id);
	});

	let selected = $derived(explorer.isSelected(item.id));

	function onpointerdown(e: PointerEvent) {
		// If control key is pressed, the user wants
		// to select more than one item.
		if (e.ctrlKey) {
			// Toggle selection based on current item.
			if (explorer.isSelected(item.id)) {
				explorer.deselect(item.id);
			} else {
				explorer.select(item.id);
			}
		} else {
			// Only select if left click.
			if (e.button !== 0) return;
			// Else the user only wants to select this item
			// we deselect all other items and then only select
			// the one we want.
			explorer.deselectAll();
			explorer.select(item.id);
		}
	}

	function onkeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') {
			explorer.open(item);
		} else if (e.key === 'Tab') {
			explorer.deselect(item.id);
		}
	}

	function oncontextmenu(e: MouseEvent) {
		e.preventDefault();
		e.stopPropagation();

		if (!selected) {
			explorer.deselectAll();
			explorer.select(item.id);
		}

		contextMenu.open({ x: e.clientX, y: e.clientY }, [
			{
				name: 'Open',
				action: () => explorer.open(item),
				hidden: explorer.selected.size !== 1,
				shortcut: 'Enter'
			},
			'separator',
			{ name: 'Cut', action: () => explorer.clipSelected('cut'), shortcut: 'Ctrl+X' },
			{
				name: 'Copy',
				action: () => explorer.clipSelected('copy'),
				disabled: true,
				shortcut: 'Ctrl+C'
			},
			{ name: 'Rename', action: () => {}, disabled: true },
			'separator',
			{
				name: 'Pin',
				action: () => explorer.pinSelected(),
				// TODO: Should also hide when all selected are pinned.
				hidden: explorer.inBin || (explorer.isPinned(item.id) && explorer.selected.size === 1),
				shortcut: 'Ctrl+P'
			},
			{
				name: 'Unpin',
				action: () => explorer.unpinSelected(),
				hidden: !explorer.isPinned(item.id) || explorer.selected.size !== 1,
				shortcut: 'Ctrl+U'
			},
			'separator',
			{
				name: 'Move to Bin',
				action: () => explorer.deleteSelected('soft'),
				hidden: explorer.inBin,
				shortcut: 'Del'
			},
			{
				name: 'Permanently Delete',
				action: () => explorer.deleteSelected('hard'),
				hidden: explorer.inBin,
				shortcut: 'Shift+Del'
			},
			{
				name: 'Delete from Bin',
				action: () => explorer.deleteSelected('hard'),
				hidden: !explorer.inBin,
				shortcut: 'Del'
			},
			{
				name: 'Recover from Bin',
				disabled: true,
				hidden: !explorer.inBin
			}
		]);
	}
</script>

<div use:BoundingClientRect={(v) => (itemBounds = v)} class="h-fit">
	<button
		class={twMerge(
			`hover:bg-primary/10 active:bg-primary/20 focus:bg-primary/10 focus:outline-none ${intersected ? 'bg-primary/10' : ''} ${selected ? 'bg-accent/20 hover:bg-accent/30 active:bg-accent/40 focus:bg-accent/30' : ''} flex h-fit w-full flex-col items-center gap-3 rounded-lg p-3 text-sm`
		)}
		{onpointerdown}
		ondblclick={() => explorer.open(item)}
		{onkeydown}
		{oncontextmenu}
	>
		{#if item.type === 'Asset'}
			{#await http.asset.thumbnail(explorer.storeId, item.id)}
				<div class="h-16"></div>
			{:then thumbnail}
				{#if thumbnail}
					<!-- svelte-ignore a11y_missing_attribute -->
					<img src={thumbnail.src} class="h-16 rounded-md" />
				{:else}
					<Icon name="image" class="my-[-13px] h-[90px] w-[90px]" />
				{/if}
			{/await}
		{:else}
			<Icon name="directory" class="my-[-13px] h-[90px] w-[90px]" />
		{/if}
		<span class="line-clamp-2 break-all">
			{item.name}
		</span>
	</button>
</div>
