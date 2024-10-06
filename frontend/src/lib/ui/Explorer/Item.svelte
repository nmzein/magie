<script lang="ts">
	import type { Bounds, Directory, Image } from '$types';
	import { image, explorer, type SelectionBox } from '$states';
	import { defined } from '$helpers';
	import Icon from '$icon';
	import { http } from '$api';
	import { onMount } from 'svelte';
	import { boundingclientrect } from '$actions';
	import { twMerge } from 'tailwind-merge';

	let { value, selectionBox }: { value: Directory | Image; selectionBox: SelectionBox } = $props();

	let thumbnail: HTMLImageElement | undefined = $state();

	onMount(async () => {
		if (value.type === 'image' && value.id !== 1) {
			thumbnail = await http.GetThumbnail(value.id);
		}
	});

	let itemBounds: Bounds | undefined = $state();
	let intersected = $derived(defined(itemBounds) && selectionBox.intersecting(itemBounds, value));
	let selected = $derived(explorer.isSelected(value));

	function handleMouseDown(event: MouseEvent) {
		// Do not deselect if right click using touchpad.
		if (event.buttons === 2) return;

		if (event.ctrlKey) {
			// Stop the mousedown event from
			// propagating to main panel which would
			// trigger a deselectAll()
			event.stopPropagation();

			// If ctrl key is pressed, the user wants
			// to select more than one item.
			// Toggle selection based on current value.
			if (explorer.isSelected(value)) {
				explorer.deselect(value);
			} else {
				explorer.select(value);
			}
		} else {
			// Else the user only wants to select this item
			// we deselect all other items and then only select
			// the one we want.
			explorer.deselectAll();
			explorer.select(value);
		}
	}

	function handleKeypress(event: KeyboardEvent) {
		if (event.key === 'Enter') {
			handleOpen();
		}
	}

	function handleOpen() {
		switch (value.type) {
			case 'directory':
				explorer.navigateTo(value.id);
				break;
			case 'image':
				image.load(value);
				break;
		}
	}
</script>

<div use:boundingclientrect={(v) => (itemBounds = v)}>
	<button
		class={twMerge(
			`hover:bg-primary/10 active:bg-primary/20 ${intersected ? 'bg-primary/10' : ''} ${selected ? 'bg-accent/20 hover:bg-accent/30 active:bg-accent/40' : ''} flex h-fit w-full flex-col items-center gap-3 rounded-lg p-3 text-sm`
		)}
		onmousedown={(e) => handleMouseDown(e)}
		ondblclick={() => handleOpen()}
		onkeypress={(e) => handleKeypress(e)}
		oncontextmenu={(e) => e.stopPropagation()}
	>
		{#if defined(thumbnail)}
			<img src={thumbnail.src} alt={value.name} class="h-16 rounded-md" />
		{:else}
			<Icon name={value.type} class="my-[-13px] h-[90px] w-[90px]" />
		{/if}
		{value.name}
	</button>
</div>
