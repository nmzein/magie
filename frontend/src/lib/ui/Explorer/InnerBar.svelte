<script lang="ts">
	import Icon from '$icon';
	import * as Dropdown from '$components/dropdown/index.ts';
	import { context } from './context.svelte.ts';

	const explorer = context.get();

	const classes = {
		trigger:
			'flex flex-row m-1 p-[7.5px] pr-[5px] rounded-[5px] gap-[7px] items-center hover:bg-primary/10 text-sm',
		list: 'flex flex-col absolute ml-1 mt-1 bg-tertiary/90 rounded-[5px] border border-primary/10 backdrop-blur-[45px] z-10 text-sm',
		item: 'flex flex-row gap-2.5 text-left items-center m-0.5 px-2.5 py-[7.5px] rounded-[5px] hover:bg-primary/10'
	};
</script>

{#snippet DownArrow()}
	<Icon name="down-arrow" class="flex h-[1.15rem] w-[1.15rem] opacity-40" />
{/snippet}

<div class="bg-primary/15 relative flex flex-row px-[1px]">
	<Dropdown.Root {classes}>
		<Dropdown.Trigger>
			<Icon name="new" class="h-[1.15rem] w-[1.15rem]" /> New
			{@render DownArrow()}
		</Dropdown.Trigger>
		<Dropdown.List>
			<Dropdown.Item onclick={() => explorer.uploader.open()}>
				<Icon name="new-image" class="h-[1.15rem] w-[1.15rem]" /> Image
			</Dropdown.Item>
			<Dropdown.Item onclick={() => explorer.directoryCreator.open()}>
				<Icon name="directory" class="h-[1.15rem] w-[1.15rem]" /> Directory
			</Dropdown.Item>
		</Dropdown.List>
	</Dropdown.Root>

	<Dropdown.Root {classes}>
		<Dropdown.Trigger>
			<Icon name="view" class="h-[1.15rem] w-[1.15rem]" /> View
			{@render DownArrow()}
		</Dropdown.Trigger>
		<Dropdown.List>
			<Dropdown.Item>
				<Icon name="list-view" class="h-[1.15rem] w-[1.15rem]" /> List
			</Dropdown.Item>
			<Dropdown.Item>
				<Icon name="grid-view" class="h-[1.15rem] w-[1.15rem]" /> Grid
			</Dropdown.Item>
		</Dropdown.List>
	</Dropdown.Root>

	<Dropdown.Root {classes}>
		<Dropdown.Trigger>
			<Icon name="sort" class="h-[1.15rem] w-[1.15rem]" /> Sort
			{@render DownArrow()}
		</Dropdown.Trigger>
		<Dropdown.List>
			<Dropdown.Item>Name</Dropdown.Item>
			<Dropdown.Item>Date Created</Dropdown.Item>
		</Dropdown.List>
	</Dropdown.Root>
</div>
