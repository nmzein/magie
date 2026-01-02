<script lang="ts">
	import Icon from '$icon';
	import Button from '$components/Button.svelte';
	import { onClickOutside } from 'runed';
	import { context } from './context.svelte.ts';

	const explorer = context.get();

	let button: HTMLButtonElement | undefined = $state();
	let name = $state('');

	explorer.deselectAll();

	onClickOutside(
		() => button,
		() => {
			if (name === '') {
				// Clicked outside and no name was set, cancel creation.
				explorer.directoryCreator.close();
			} else {
				// Clicked anywhere and a name was set, create directory.
				explorer.createDirectory(name);
			}
		}
	);

	function onkeydown(event: KeyboardEvent) {
		event.stopPropagation();
		if (event.key === 'Enter' && name !== '') {
			explorer.createDirectory(name);
		}
	}
</script>

<Button
	bind:component={button}
	class="hover:bg-primary/10 active:bg-primary/20 flex h-fit flex-col items-center rounded-lg px-2.5 pb-[7.5px] hover:backdrop-blur-[15px]"
	{onkeydown}
>
	<Icon name="directory" class="h-22.5 w-22.5" />
	<!-- svelte-ignore a11y_autofocus -->
	<input
		autofocus
		type="text"
		class="bg-primary/15 -mt-1.25 h-7 w-full grow rounded-[inherit] px-2.5 py-1.25 text-center focus:outline-none"
		bind:value={name}
		placeholder=""
	/>
</Button>
