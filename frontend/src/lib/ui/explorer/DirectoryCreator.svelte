<script lang="ts">
	import { http } from '$api';
	import { explorer } from '$states';
	import Icon from '$icon';

	let button: HTMLButtonElement | undefined;
	let name = $state('');

	$effect(() => {
		setTimeout(() => {
			document.addEventListener('click', handleClick);
		}, 10);

		return () => {
			document.removeEventListener('click', handleClick);
		};
	});

	async function create(name: string) {
		if (explorer.currentDirectory === undefined) return;
		await http.CreateDirectory(explorer.currentDirectory.data.id, name);
		explorer.showDirectoryCreator = false;
	}

	function cancel() {
		explorer.showDirectoryCreator = false;
	}

	function handleClick(event: MouseEvent) {
		if (button === undefined) return;

		let clickedInside = button.contains(event.target as Node);

		if (clickedInside) {
			// Clicked inside and no name was set, do nothing.
			return;
		} else if (!clickedInside && name == '') {
			// Clicked outside and no name was set, cancel creation.
			cancel();
		} else {
			// Clicked anywhere and a name was set, create directory.
			create(name);
		}
	}

	function handleKeypress(event: KeyboardEvent) {
		if (event.key === 'Enter') {
			if (name == '') {
				return;
			} else {
				create(name);
			}
		}
	}
</script>

<button bind:this={button} class="flex-column" onkeypress={(e) => handleKeypress(e)}>
	<Icon variant="directory" width={5} height={5} />
	<!-- svelte-ignore a11y_autofocus -->
	<input autofocus type="text" class="light-layer" bind:value={name} placeholder="New Directory" />
</button>

<style lang="scss">
	input {
		flex-grow: 1;
		padding: 5px 10px;
		// height: 25px;
		width: 85%;
		border-radius: var(--border-radius);
	}

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

	// .selected {
	// 	background-color: rgba(51, 156, 255, 0.2) !important;

	// 	&:hover {
	// 		background-color: rgba(51, 156, 255, 0.3) !important;
	// 	}

	// 	&:active {
	// 		background-color: rgba(51, 156, 255, 0.4) !important;
	// 	}
	// }
</style>
