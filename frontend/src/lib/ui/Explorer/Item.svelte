<script lang="ts">
	import type { Bounds, Directory, Image } from '$types';
	import { image, explorer, type SelectionBox } from '$states';
	import { defined } from '$helpers';
	import Icon from '$icon';
	import { http } from '$api';
	import { onMount } from 'svelte';

	let { value, selectionBox }: { value: Directory | Image; selectionBox: SelectionBox } = $props();

	let thumbnail: HTMLImageElement | undefined = $state();

	onMount(async () => {
		if (value.type === 'image' && value.id !== 1) {
			thumbnail = await http.GetThumbnail(value.id);
		}
		console.log(thumbnail);
	});

	let itemBounds: Bounds | undefined = $state();

	function resizeobserver(element: HTMLElement) {
		function update() {
			itemBounds = element.getBoundingClientRect();
		}

		const observer = new ResizeObserver(update);
		observer.observe(element);

		update();

		return {
			destroy() {
				observer.unobserve(element);
			}
		};
	}

	function positionobserver(element: HTMLElement) {
		let lastPosition = { x: 0, y: 0 };

		function update() {
			const rect = element.getBoundingClientRect();
			const newPosition = { x: rect.x, y: rect.y };

			// Check if the position has changed
			if (newPosition.x !== lastPosition.x || newPosition.y !== lastPosition.y) {
				lastPosition = newPosition;
				itemBounds = rect; // Update your reactive value
			}
		}

		// Use requestAnimationFrame to continuously check for changes
		function loop() {
			update();
			requestAnimationFrame(loop); // Keep checking on each frame
		}

		loop(); // Start the loop on mount

		return {
			destroy() {
				// Cleanup if necessary
			}
		};
	}

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

<button
	use:resizeobserver
	use:positionobserver
	class="hover:bg-primary/10 active:bg-primary/20 flex flex-col items-center gap-3 rounded-lg p-3 text-sm hover:backdrop-blur-[15px]
		   {intersected ? '!bg-primary/10 !backdrop-blur-[15px]' : ''}
		   {selected ? '!bg-accent/20 hover:!bg-accent/30 active:!bg-accent/40' : ''}"
	onmousedown={(e) => handleMouseDown(e)}
	ondblclick={() => handleOpen()}
	onkeypress={(e) => handleKeypress(e)}
>
	{#if defined(thumbnail)}
		<img src={thumbnail.src} alt={value.name} class="h-16 rounded-md" />
	{:else}
		<Icon name={value.type} class="my-[-13px] h-[90px] w-[90px]" />
	{/if}
	{value.name}
</button>
