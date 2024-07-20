<script lang="ts">
	import { explorer, SelectionBox } from '$states';
	import type { Directory, Image } from '$types';
	import Item from './Item.svelte';
	import DirectoryCreator from './DirectoryCreator.svelte';
	import { defined } from '$helpers';

	let mainPanel: HTMLDivElement | undefined = $state();
	let mainPanelBounds = $derived(mainPanel?.getBoundingClientRect());
	let selectionBoxElement: HTMLDivElement | undefined = $state();

	const selectionBox: SelectionBox<Directory | Image> | undefined = $derived.by(() => {
		if (!defined(selectionBoxElement) || !defined(mainPanelBounds)) return;
		return new SelectionBox(selectionBoxElement, mainPanelBounds);
	});

	$effect(() => {
		document.addEventListener('mousemove', handleMouseMove);
		document.addEventListener('mouseup', handleMouseUp);

		return () => {
			document.removeEventListener('mousemove', handleMouseMove);
			document.removeEventListener('mouseup', handleMouseUp);
		};
	});

	function handleMouseDown(event: MouseEvent) {
		explorer.deselectAll();

		if (!defined(selectionBox)) return;

		selectionBox.start({ x: event.clientX, y: event.clientY });
	}

	function handleMouseMove(event: MouseEvent) {
		if (!defined(selectionBox)) return;

		selectionBox.update({ x: event.clientX, y: event.clientY });
	}

	function handleMouseUp() {
		if (!defined(selectionBox) || !selectionBox.dragging) return;

		explorer.selected = selectionBox.stop();
	}
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div id="main-panel" bind:this={mainPanel} onmousedown={handleMouseDown}>
	{#if defined(explorer.currentDirectory) && defined(selectionBox)}
		{#each explorer.currentDirectory.directory.subdirectories as subdirectory, index}
			<Item variant="directory" value={subdirectory} {index} {selectionBox} />
		{/each}
		{#each explorer.currentDirectory.directory.files as file, index}
			<Item variant="image" value={file} {index} {selectionBox} />
		{/each}
		{#if explorer.showDirectoryCreator}
			<DirectoryCreator />
		{/if}
	{/if}

	<div
		id="selection-box"
		bind:this={selectionBoxElement}
		class:invisible={!selectionBox?.dragging && true}
	></div>
</div>

<style lang="scss">
	#main-panel {
		display: grid;
		grid-template-columns: repeat(4, calc(25% - 7.5px));
		grid-template-rows: repeat(4, 1fr);
		padding: 10px 20px;
		gap: 10px;
		z-index: 1;
		position: relative;

		height: 400px;
	}

	#selection-box {
		position: absolute;
		border: 1px solid rgb(51, 156, 255);
		border-radius: 10px;
		background-color: rgba(51, 156, 255, 0.2);
	}

	.invisible {
		visibility: hidden;
	}
</style>
