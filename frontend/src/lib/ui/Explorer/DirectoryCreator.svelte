<script lang="ts">
	import { explorer } from '$states';
	import Icon from '$icon';
	import Button from '$components/Button.svelte';
	import { defined } from '$helpers';

	let button: HTMLButtonElement | undefined = $state();
	let name = $state('');

	$effect(() => {
		explorer!.deselectAll();

		let timeout = setTimeout(() => {
			document.addEventListener('click', handleClick);
		}, 10);

		return () => {
			document.removeEventListener('click', handleClick);
			clearTimeout(timeout);
		};
	});

	function handleClick(event: MouseEvent) {
		if (!defined(button) || !defined(explorer!.directory)) return;

		let clickedInside = button.contains(event.target as Node);

		// Clicked inside, do nothing.
		if (clickedInside) return;

		if (!clickedInside && name == '') {
			// Clicked outside and no name was set, cancel creation.
			explorer!.directoryCreator.close();
		} else {
			// Clicked anywhere and a name was set, create directory.
			explorer!.directoryCreator.create(explorer!.directory.data.id, name);
		}
	}

	function onkeypress(event: KeyboardEvent) {
		if (event.key === 'Enter' && name !== '' && defined(explorer!.directory)) {
			explorer!.directoryCreator.create(explorer!.directory.data.id, name);
		}
	}
</script>

<Button
	bind:component={button}
	class="hover:bg-primary/10 active:bg-primary/20 flex h-fit flex-col items-center rounded-lg px-[10px] pb-[7.5px] hover:backdrop-blur-[15px]"
	{onkeypress}
	onkeydown={(e) => e.stopPropagation()}
>
	<Icon name="directory" class="h-[90px] w-[90px]" />
	<!-- svelte-ignore a11y_autofocus -->
	<input
		autofocus
		type="text"
		class="bg-primary/15 mt-[-5px] h-7 w-full grow rounded-[inherit] px-[10px] py-[5px] text-center focus:outline-none"
		bind:value={name}
		placeholder=""
	/>
</Button>
