<script lang="ts">
	import { image, transformer } from '$states';
	import * as Tabs from '$components/tabs/index.ts';
	import { Explorer } from './Explorer';
	import Icon from '$icon';
	import AnnotationControls from '$ui/AnnotationControls.svelte';
	import { defined, truncateNumber } from '$helpers';
	import Button from '$components/Button.svelte';
	import type { Bounds } from '$types';
	import ContextMenu from './ContextMenu.svelte';

	const classes = {
		list: 'panel flex flex-col gap-[2px] p-[3px]',
		trigger: {
			base: 'rounded-[7px] h-12 w-12 p-[5px] hover:backdrop-blur-[15px] hover:bg-primary/10',
			active: 'bg-primary/10',
			disabled: 'opacity-30'
		},
		content: 'h-full'
	};

	let contentSpaceBounds: Bounds | undefined = $state();
</script>

<Tabs.Root id="zooming" mode="buttons" {classes}>
	<Tabs.Root id="applets" mode="collapsible-tab" {classes}>
		<Tabs.Root id="drawing" mode="tab" currentTab="move" {classes}>
			{@render UILayer()}
		</Tabs.Root>
	</Tabs.Root>
</Tabs.Root>

{#snippet UILayer()}
	<div class="pointer-events-none absolute flex w-full flex-row gap-[10px] overflow-hidden">
		<ContextMenu />

		<Tabs.ContentSpace id="applets">
			<div
				class="h-screen w-full flex-1 shrink-0 overflow-hidden p-[10px] pr-0"
				bind:contentRect={contentSpaceBounds}
			>
				<Tabs.Content value="explorer">
					{#if defined(contentSpaceBounds)}
						<Explorer {contentSpaceBounds} />
					{/if}
				</Tabs.Content>
				<Tabs.Content value="control" disabled={!image.initialised}>
					<AnnotationControls />
				</Tabs.Content>
			</div>
		</Tabs.ContentSpace>

		<nav class="pointer-events-none p-[10px] pl-0">
			<div class="flex h-full flex-col gap-[10px]">
				<Tabs.TriggerList id="zooming">
					<Tabs.Trigger
						sideEffect={() => transformer.zoom(-100)}
						disabled={!image.initialised || transformer.atMaxScale()}
					>
						<Icon name="zoom-in" class="h-9 w-9" />
					</Tabs.Trigger>
					<button
						onclick={() => {
							if (image.initialised) transformer.resetScale();
						}}
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
				</Tabs.TriggerList>

				<Tabs.TriggerList id="applets">
					<Tabs.Trigger value="explorer">
						<Icon name="explorer" class="h-9 w-9" />
					</Tabs.Trigger>
					<Tabs.Trigger
						value="control"
						disabled={!defined(image.properties) || image.properties.annotations.length === 0}
					>
						<Icon name="control" class="h-9 w-9" />
					</Tabs.Trigger>
					<Tabs.Trigger value="info" disabled={!image.initialised}>
						<Icon name="info" class="h-9 w-9" />
					</Tabs.Trigger>
				</Tabs.TriggerList>

				<Tabs.TriggerList id="drawing">
					<Tabs.Trigger value="move">
						<Icon name="cursor" class="h-9 w-9" />
					</Tabs.Trigger>
					<Tabs.Trigger value="freehand-draw" disabled>
						<Icon name="freehand" class="h-9 w-9" />
					</Tabs.Trigger>
					<Tabs.Trigger value="square" disabled>
						<Icon name="square" class="h-9 w-9" />
					</Tabs.Trigger>
				</Tabs.TriggerList>

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
	</div>
{/snippet}
