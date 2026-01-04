<script lang="ts">
	import type { Bounds } from '$types';
	import { defined, truncateNumber } from '$helpers';
	import { registry, viewerManager } from '$states';
	import * as Tabs from '$components/tabs/index.ts';
	import Button from '$components/Button.svelte';
	import Window from '$components/window/Window.svelte';
	import { Explorer } from '$ui/Explorer';
	import { ContextMenu } from '$ui/ContextMenu/index.ts';
	// import Geometry2DControls from '$view/Geometry2D/Control.svelte';
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
</script>

<div class="pointer-events-none absolute z-100 flex w-full flex-row gap-2 overflow-hidden">
	<Tabs.Root id="zooming" mode="buttons" {classes}>
		<Tabs.Root id="applets" mode="collapsible-tab" {classes}>
			<Tabs.Root id="drawing" mode="tab" currentTab="move" {classes}>
				<ContextMenu />

				<Tabs.ContentSpace id="applets">
					<div
						class="h-screen w-full flex-1 shrink-0 overflow-hidden p-2 pr-1"
						bind:contentRect={contentSpaceBounds}
					>
						<Tabs.Content value="explorer">
							<Window {contentSpaceBounds}>
								<Explorer />
							</Window>
						</Tabs.Content>
						<!-- <Tabs.Content value="control" disabled={!defined(viewerManager.activeViewer)}>
							<Window {contentSpaceBounds}>
								<Geometry2DControls
									bind:geometries={viewerManager.activeViewer!.state.geometries}
								/>
							</Window>
						</Tabs.Content> -->
					</div>
				</Tabs.ContentSpace>

				<div class="pointer-events-none p-2.5 pl-0">
					<div class="flex h-full flex-col gap-2.5">
						<Tabs.TriggerList id="zooming">
							<Tabs.Trigger
								sideEffect={() => {
									viewerManager.activeViewer?.zoom(-100);
								}}
								disabled={!defined(viewerManager.activeViewer) ||
									viewerManager.activeViewer?.atMaxScale()}
							>
								<Icon name="zoom-in" class="h-9 w-9" />
							</Tabs.Trigger>
							<button
								onclick={() => {
									viewerManager.activeViewer?.reset();
								}}
								disabled={!defined(viewerManager.activeViewer)}
								class="my-1.25 text-center select-none"
								class:cursor-pointer={defined(viewerManager.activeViewer)}
								class:opacity-30={!defined(viewerManager.activeViewer)}
							>
								{defined(viewerManager.activeViewer)
									? truncateNumber(viewerManager.activeViewer.scale)
									: '1.0'}x
							</button>
							<Tabs.Trigger
								sideEffect={() => {
									viewerManager.activeViewer?.zoom(100);
								}}
								disabled={!defined(viewerManager.activeViewer) ||
									viewerManager.activeViewer?.atMinScale()}
							>
								<Icon name="zoom-out" class="h-9 w-9" />
							</Tabs.Trigger>
						</Tabs.TriggerList>

						<Tabs.TriggerList id="applets">
							<Tabs.Trigger
								value="explorer"
								disabled={!defined(contentSpaceBounds) || !registry.loaded}
							>
								<Icon name="explorer" class="h-9 w-9" />
							</Tabs.Trigger>
							<!-- <Tabs.Trigger
								value="control"
								disabled={!defined(viewerManager.activeViewer) ||
									viewerManager.activeViewer?.asset.geometries.length === 0}
							>
								<Icon name="control" class="h-9 w-9" />
							</Tabs.Trigger> -->
							<Tabs.Trigger value="info" disabled={true}>
								<Icon name="info" class="h-9 w-9" />
							</Tabs.Trigger>

							<Tabs.Trigger value="info" disabled={true}>
								<Icon name="shapes" class="h-9 w-9" />
							</Tabs.Trigger>
						</Tabs.TriggerList>

						<div class="panel mt-auto flex flex-col gap-0.5 p-0.75">
							<Button
								class="hover:bg-primary/10 h-12 w-12 rounded-[7px] p-1.25 opacity-30 hover:backdrop-blur-[15px]"
								disabled
							>
								<Icon name="settings" class="h-9 w-9" />
							</Button>
						</div>
					</div>
				</div>
			</Tabs.Root>
		</Tabs.Root>
	</Tabs.Root>
</div>
