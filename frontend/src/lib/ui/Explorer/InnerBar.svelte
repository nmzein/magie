<script lang="ts">
	import { explorer, uploader } from '$states';
	import Icon from '$icon';
	import * as Dropdown from '$components/dropdown/index.ts';
	import { defined } from '$helpers';

	const classes = {
		trigger:
			'flex flex-row m-[4px] p-[7.5px] pr-[5px] rounded-[5px] gap-[7px] items-center hover:bg-primary/10 text-sm',
		list: 'flex flex-col absolute mt-[4px] ml-[4px] bg-tertiary/90 rounded-[5px] border border-primary/10 backdrop-blur-[45px] z-10 text-sm',
		item: 'flex flex-row gap-[10px] items-center m-[2px] px-[10px] py-[7.5px] rounded-[5px] hover:bg-primary/10'
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
			<Dropdown.Item
				onclick={() => {
					if (!defined(explorer.currentDirectory)) return;
					explorer.showUploader = true;
					uploader.parentDirectoryId = explorer.currentDirectory.data.id;
				}}
			>
				<Icon name="new-image" class="h-[1.15rem] w-[1.15rem]" /> Image
			</Dropdown.Item>
			<Dropdown.Item onclick={() => (explorer.showDirectoryCreator = true)}>
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
