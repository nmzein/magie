<script lang="ts">
	import type { Directory, Image } from '$types';
	import { image, explorer, type SelectionBox } from '$states';
	import { defined } from '$helpers';
	import Icon from '$icon';
	import Button from '$components/Button.svelte';

	let {
		variant,
		value,
		selectionBox
	}: {
		variant: string;
		value: Directory | Image;
		selectionBox: SelectionBox;
	} = $props();

	let item: HTMLButtonElement | undefined = $state();
	let itemBounds = $derived(item?.getBoundingClientRect());
	let intersected = $derived(defined(itemBounds) && selectionBox.intersecting(itemBounds, value));
	let selected = $derived(explorer.isSelected(value));

	function handleMouseDown(event: MouseEvent) {
		// Stop the mousedown event from
		// propagating to main panel which would
		// trigger a deselectAll()
		event.stopPropagation();

		if (event.ctrlKey) {
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
		if (variant === 'directory') {
			explorer.navigateTo(value.id);
		} else if (variant === 'file') {
			image.load(value);
		}
	}
</script>

<Button
	bind:component={item}
	class="hover:bg-primary/10 active:bg-primary/20 flex flex-col items-center rounded-lg px-[10px] pb-[10px] text-sm hover:backdrop-blur-[15px]
		   {intersected ? 'bg-primary/10 backdrop-blur-[15px]' : ''}
		   {selected ? 'bg-accent/20 hover:bg-accent/30 active:bg-accent/40' : ''}"
	onmousedown={(e) => handleMouseDown(e)}
	ondblclick={() => handleOpen()}
	onkeypress={(e) => handleKeypress(e)}
>
	<Icon name={variant} class="h-20 w-20" />
	{value.name}
</Button>
