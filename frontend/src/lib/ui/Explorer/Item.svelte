<script lang="ts">
	import type { Directory, Image } from '$types';
	import { image, explorer, type SelectionBox } from '$states';
	import { defined } from '$helpers';
	import Icon from '$icon';

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

<button
	bind:this={item}
	class="flex-column"
	class:intersected
	class:selected={explorer.isSelected(value)}
	onmousedown={(e) => handleMouseDown(e)}
	ondblclick={() => handleOpen()}
	onkeypress={(e) => handleKeypress(e)}
>
	<Icon {variant} width={5} height={5} />
	{value.name}
</button>

<style lang="scss">
	button {
		align-items: center;
		border-radius: 10px;
		padding: 0 10px 10px 10px;
		z-index: 0;

		&:hover,
		&.intersected {
			background-color: rgba(255, 255, 255, 0.1);
			backdrop-filter: blur(15px);
		}

		&:active {
			background-color: rgba(255, 255, 255, 0.2);
		}

		&.selected {
			background-color: rgba(51, 156, 255, 0.2) !important;

			&:hover {
				background-color: rgba(51, 156, 255, 0.3) !important;
			}

			&:active {
				background-color: rgba(51, 156, 255, 0.4) !important;
			}
		}
	}
</style>
