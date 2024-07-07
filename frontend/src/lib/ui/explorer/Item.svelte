<script lang="ts">
	import Folder from '~icons/material-symbols-light/folder';
	import File from '~icons/ph/image-light';

	import type { Directory, Image } from '$types';
	import { directoryStack } from './state.svelte';
	import { image } from '$states';

	let { type, value, index }: { type: string; value: Directory | Image; index: number } = $props();
	let selected = $state(false);

	function handleKeypress(event: KeyboardEvent) {
		if (event.key === 'Enter') {
			handleAction();
		}
	}

	function handleAction() {
		if (type === 'directory') {
			directoryStack.value.push(index);
		} else if (type === 'file') {
			image.load(value);
		}
	}
</script>

<button
	class="flex-column"
	onclick={() => (selected = !selected)}
	ondblclick={() => handleAction()}
	onkeypress={(e) => handleKeypress(e)}
	class:selected
>
	{#if type === 'directory'}
		<Folder width="5em" height="5em" />
	{:else if type === 'file'}
		<File width="5em" height="5em" />
	{/if}
	{value.name}
</button>

<style lang="scss">
	button {
		align-items: center;
		border-radius: 10px;
		padding: 0 10px 10px 10px;
		z-index: 0;

		&:hover {
			backdrop-filter: blur(15px);
			background-color: rgba(255, 255, 255, 0.1);
		}

		&:active {
			background-color: rgba(255, 255, 255, 0.2);
		}
	}

	.selected {
		background-color: rgba(51, 156, 255, 0.2) !important;

		&:hover {
			background-color: rgba(51, 156, 255, 0.3) !important;
		}

		&:active {
			background-color: rgba(51, 156, 255, 0.4) !important;
		}
	}
</style>
