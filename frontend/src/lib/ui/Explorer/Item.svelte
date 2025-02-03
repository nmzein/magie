<script lang="ts">
	import type { Bounds, Directory, Image } from '$types';
	import { NewImageViewer, explorer, type SelectionBoxState, contextMenu } from '$states';
	import { defined } from '$helpers';
	import Icon from '$icon';
	import { http } from '$api';
	import { onMount } from 'svelte';
	import { twMerge } from 'tailwind-merge';
	import { BoundingClientRect } from '$actions';

	let { value, selection }: { value: Directory | Image; selection: SelectionBoxState } = $props();

	let thumbnail: HTMLImageElement | undefined = $state();

	onMount(async () => {
		if (value.type === 'image' && value.id !== 1) {
			thumbnail = await http.image.thumbnail(value.id);
		}
	});

	let itemBounds: Bounds | undefined = $state();
	let intersected = $state(false);

	$effect(() => {
		intersected = defined(itemBounds) && selection.intersecting(itemBounds, value);
	});

	const selected = $derived(explorer!.isSelected(value));

	function onpointerdown(e: PointerEvent) {
		// If control key is pressed, the user wants
		// to select more than one item.
		if (e.ctrlKey) {
			// Toggle selection based on current value.
			if (explorer!.isSelected(value)) {
				explorer!.deselect(value);
			} else {
				explorer!.select(value);
			}
		} else {
			// Only select if left click.
			if (e.button !== 0) return;
			// Else the user only wants to select this item
			// we deselect all other items and then only select
			// the one we want.
			explorer!.deselectAll();
			explorer!.select(value);
		}
	}

	function onkeypress(e: KeyboardEvent) {
		if (e.key === 'Enter') {
			handleOpen();
		}
	}

	function handleOpen() {
		switch (value.type) {
			case 'directory':
				explorer!.navigateTo(value.id);
				break;
			case 'image':
				NewImageViewer(value);
				break;
		}
	}
</script>

<div use:BoundingClientRect={(v) => (itemBounds = v)} class="h-fit">
	<button
		class={twMerge(
			`hover:bg-primary/10 active:bg-primary/20 ${intersected ? 'bg-primary/10' : ''} ${selected ? 'bg-accent/20 hover:bg-accent/30 active:bg-accent/40' : ''} flex h-fit w-full flex-col items-center gap-3 rounded-lg p-3 text-sm`
		)}
		{onpointerdown}
		ondblclick={handleOpen}
		{onkeypress}
		oncontextmenu={(e) => {
			e.preventDefault();

			if (!selected) {
				explorer!.deselectAll();
				explorer!.select(value);
			}

			contextMenu.show = true;
			contextMenu.position = { x: e.clientX, y: e.clientY };
			contextMenu.items = [
				{ name: 'Open', action: () => handleOpen(), hidden: explorer!.selected.length !== 1 },
				{
					name: 'Pin',
					action: () => explorer!.pinSelected(),
					hidden: explorer!.isPinned(value) && explorer!.selected.length === 1
				},
				{
					name: 'Unpin',
					action: () => explorer!.unpinSelected(),
					hidden: !explorer!.isPinned(value) || explorer!.selected.length !== 1
				},
				{ name: 'Copy', action: () => explorer!.clipSelected('copy'), disabled: true },
				{ name: 'Cut', action: () => explorer!.clipSelected('cut') },
				{
					name: 'Move to Bin',
					action: () => explorer!.deleteSelected('soft'),
					hidden: explorer!.directory?.data.id === 1
				},
				{
					name: 'Delete from Bin',
					action: () => explorer!.deleteSelected('hard'),
					hidden: explorer!.directory?.data.id !== 1
				},
				{
					name: 'Recover from Bin',
					action: () => {},
					disabled: true,
					hidden: explorer!.directory?.data.id !== 1
				}
			];
		}}
	>
		{#if defined(thumbnail)}
			<img src={thumbnail.src} alt={value.name} class="h-16 rounded-md" />
		{:else}
			<Icon name={value.type} class="my-[-13px] h-[90px] w-[90px]" />
		{/if}
		<span class="line-clamp-2 break-all">
			{value.name}
		</span>
	</button>
</div>
