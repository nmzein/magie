<script lang="ts">
	import { image, transformer } from '$states';
	import * as Tabs from '$components/tabs/index.ts';
	import { Explorer } from './Explorer';
	import Icon from '$icon';
	import AnnotationControls from '$ui/AnnotationControls.svelte';
	import { truncateNumber } from '$helpers';
	import Button from '$components/Button.svelte';

	const classes = {
		list: 'panel flex flex-col gap-[2px] p-[3px]',
		trigger: {
			base: 'rounded-[7px] h-12 w-12 p-[5px] hover:backdrop-blur-[15px] hover:bg-primary/10',
			active: 'bg-primary/10',
			disabled: 'opacity-30'
		},
		content: 'p-0 absolute right-0 mr-[65px] w-auto'
	};
</script>

<nav class="pointer-events-none absolute bottom-0 right-0 top-0 m-[10px] block">
	<div class="flex h-full flex-col gap-[10px]">
		<Tabs.Root mode="buttons" {classes}>
			<Tabs.List>
				<Tabs.Trigger
					sideEffect={() => transformer.zoom(-100)}
					disabled={!image.initialised || transformer.atMaxScale()}
				>
					<Icon name="zoom-in" class="h-9 w-9" />
				</Tabs.Trigger>
				<button
					onclick={() => transformer.resetScale()}
					class="my-[5px] select-none text-center"
					class:cursor-pointer={image.initialised}
					class:opacity-30={!image.initialised}
				>
					{truncateNumber(transformer.scale)}x
				</button>
				<Tabs.Trigger
					sideEffect={() => transformer.zoom(100)}
					disabled={!image.initialised || transformer.atMinScale()}
				>
					<Icon name="zoom-out" class="h-9 w-9" />
				</Tabs.Trigger>
			</Tabs.List>
		</Tabs.Root>

		<Tabs.Root mode="collapsible-tab" {classes}>
			<Tabs.List>
				<Tabs.Trigger value="explorer">
					<Icon name="explorer" class="h-9 w-9" />
				</Tabs.Trigger>
				<Tabs.Trigger value="control" disabled={!image.initialised}>
					<Icon name="control" class="h-9 w-9" />
				</Tabs.Trigger>
				<Tabs.Trigger value="info" disabled={!image.initialised}>
					<Icon name="info" class="h-9 w-9" />
				</Tabs.Trigger>
			</Tabs.List>
			<Tabs.Content value="explorer">
				<Explorer />
			</Tabs.Content>
			<Tabs.Content value="control" disabled={!image.initialised}>
				<AnnotationControls />
			</Tabs.Content>
		</Tabs.Root>

		<Tabs.Root mode="tab" currentTab="move" {classes}>
			<Tabs.List>
				<Tabs.Trigger value="move" sideEffect={undefined} disabled={false}>
					<Icon name="cursor" class="h-9 w-9" />
				</Tabs.Trigger>
				<Tabs.Trigger value="freehand-draw" sideEffect={undefined} disabled={true}>
					<Icon name="freehand" class="h-9 w-9" />
				</Tabs.Trigger>
				<Tabs.Trigger value="square" sideEffect={undefined} disabled={true}>
					<Icon name="square" class="h-9 w-9" />
				</Tabs.Trigger>
			</Tabs.List>
		</Tabs.Root>

		<div class="panel mt-auto flex flex-col gap-[2px] p-[3px]">
			<Button
				class="hover:bg-primary/10 h-12 w-12 rounded-[7px] p-[5px] opacity-30 hover:backdrop-blur-[15px]"
				disabled
			>
				<Icon name="settings" class="h-9 w-9" />
			</Button>
		</div>
	</div>
</nav>
