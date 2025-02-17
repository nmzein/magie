<script lang="ts">
	import Icon from '$icon';
	import Button from '$components/Button.svelte';
	import { defined } from '$helpers';
	import { onClickOutside } from 'runed';
	import { context } from './context.svelte.ts';

	const explorer = context.get();

	let button: HTMLButtonElement | undefined = $state();
	let name = $state('');

	explorer.deselectAll();

	onClickOutside(
		() => button,
		() => {
			if (name == '') {
				// Clicked outside and no name was set, cancel creation.
				explorer.directoryCreator.close();
			} else {
				// Clicked anywhere and a name was set, create directory.
				// FIX: I don't like this.
				explorer.directoryCreator.create(explorer.storeId, explorer.directory.id, name);
			}
		}
	);

	function onkeypress(event: KeyboardEvent) {
		if (event.key === 'Enter' && name !== '' && defined(explorer.directory)) {
			// FIX: I don't like this.
			explorer.directoryCreator.create(explorer.storeId, explorer.directory.id, name);
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
