<script lang="ts">
	import type { Bounds } from '$types';
	import type { View } from '$lib/types/views';
	import { defined, truncateNumber } from '$helpers';
	import { registry, views } from '$states';
	import * as Tabs from '$components/tabs/index.ts';
	import Button from '$components/Button.svelte';
	import { Explorer } from '$ui/Explorer';
	import ContextMenu from '$ui/ContextMenu.svelte';
	import Geometry2DControls from '$view/Geometry2D/Control.svelte';
	import Icon from '$icon';

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

	const activeView: View | undefined = $derived(views[0]);
</script>

<Tabs.Root id="zooming" mode="buttons" {classes}>
	<Tabs.Root id="applets" mode="collapsible-tab" {classes}>
		<Tabs.Root id="drawing" mode="tab" currentTab="move" {classes}>
			<div class="pointer-events-none absolute flex w-full flex-row gap-[10px] overflow-hidden">
				<ContextMenu />

				<Tabs.ContentSpace id="applets">
					<div
						class="h-screen w-full flex-1 shrink-0 overflow-hidden p-[10px] pr-0"
						bind:contentRect={contentSpaceBounds}
					>
						<Tabs.Content value="explorer">
							<Explorer {contentSpaceBounds} />
						</Tabs.Content>
						<Tabs.Content value="control" disabled={!defined(activeView)}>
							<Geometry2DControls bind:geometries={activeView!.state.geometries} />
						</Tabs.Content>
					</div>
				</Tabs.ContentSpace>

				<div class="pointer-events-none p-[10px] pl-0">
					<div class="flex h-full flex-col gap-[10px]">
						<Tabs.TriggerList id="zooming">
							<Tabs.Trigger
								sideEffect={() => {
									activeView?.state.transformer.zoom(-100);
								}}
								disabled={!defined(activeView) || activeView?.state.transformer.atMaxScale}
							>
								<Icon name="zoom-in" class="h-9 w-9" />
							</Tabs.Trigger>
							<button
								onclick={() => {
									activeView?.state.transformer.resetScale();
								}}
								class="my-[5px] text-center select-none"
								class:cursor-pointer={defined(activeView)}
								class:opacity-30={!defined(activeView)}
							>
								{defined(activeView) ? truncateNumber(activeView.state.transformer.scale) : '1.0'}x
							</button>
							<Tabs.Trigger
								sideEffect={() => {
									activeView?.state.transformer.zoom(100);
								}}
								disabled={!defined(activeView) || activeView?.state.transformer.atMinScale}
							>
								<Icon name="zoom-out" class="h-9 w-9" />
							</Tabs.Trigger>
						</Tabs.TriggerList>

						<Tabs.TriggerList id="applets">
							<Tabs.Trigger
								value="explorer"
								disabled={!defined(contentSpaceBounds) || !defined(registry.registry)}
							>
								<Icon name="explorer" class="h-9 w-9" />
							</Tabs.Trigger>
							<Tabs.Trigger
								value="control"
								disabled={!defined(activeView) || activeView?.state.geometries.length === 0}
							>
								<Icon name="control" class="h-9 w-9" />
							</Tabs.Trigger>
							<Tabs.Trigger value="info" disabled={true}>
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
				</div>
			</div>
		</Tabs.Root>
	</Tabs.Root>
</Tabs.Root>
