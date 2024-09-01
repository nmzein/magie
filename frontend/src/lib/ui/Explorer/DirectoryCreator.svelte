<script lang="ts">
	import { http } from '$api';
	import { explorer } from '$states';
	import Icon from '$icon';
	import Button from '$components/Button.svelte';

	let button: HTMLButtonElement | undefined = $state();
	let name = $state('');

	$effect(() => {
		let timeout = setTimeout(() => {
			document.addEventListener('click', handleClick);
		}, 10);

		return () => {
			document.removeEventListener('click', handleClick);
			clearTimeout(timeout);
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

		// Clicked inside, do nothing.
		if (clickedInside) return;

		if (!clickedInside && name == '') {
			// Clicked outside and no name was set, cancel creation.
			cancel();
		} else {
			// Clicked anywhere and a name was set, create directory.
			create(name);
		}
	}

	function handleKeypress(event: KeyboardEvent) {
		if (event.key === 'Enter' && name !== '') {
			create(name);
		}
	}
</script>

<Button
	bind:component={button}
	class="hover:bg-primary/10 active:bg-primary/20 flex flex-col items-center rounded-lg px-[10px] pb-[10px] text-xs hover:backdrop-blur-[15px]"
	onkeypress={(e) => handleKeypress(e)}
>
	<Icon name="directory" class="h-20 w-20" />
	<!-- svelte-ignore a11y_autofocus -->
	<input
		autofocus
		type="text"
		class="bg-primary/15 h-7 w-full grow rounded-[inherit] px-[10px] py-[5px] focus:outline-none"
		bind:value={name}
		placeholder="New Directory"
	/>
</Button>
