<script lang="ts">
	import { image, transformer } from '$states';
	import * as Tabs from '$components/tabs/index.ts';
	import { Explorer } from './Explorer';
	import Icon from '$icon';
	import AnnotationControls from '$ui/AnnotationControls.svelte';
	import { truncateNumber } from '$helpers';

	const ICON_SIZE = 2.3;
	const classes = {
		list: 'panel control-panel-tab-list',
		trigger: {
			regular: 'control-panel-tab-trigger',
			active: 'control-panel-tab-trigger-active',
			disabled: 'control-panel-tab-trigger-disabled'
		},
		content: 'control-panel-tab-content'
	};
</script>

<nav>
	<div class="groups">
		<Tabs.Root mode="0" {classes}>
			<Tabs.List>
				<Tabs.Trigger
					sideEffect={() => transformer.zoom(-100)}
					disabled={!image.initialised || transformer.atMaxScale()}
				>
					<Icon variant="zoom-in" width={ICON_SIZE} height={ICON_SIZE} />
				</Tabs.Trigger>
				<!-- svelte-ignore a11y_no_static_element_interactions -->
				<!-- svelte-ignore a11y_click_events_have_key_events -->
				<span
					onclick={() => transformer.resetScale()}
					style="text-align: center; margin: 5px 0; user-select: none;
						   {image.initialised ? 'cursor: pointer' : ''};"
					class:control-panel-tab-trigger-disabled={!image.initialised}
				>
					{truncateNumber(transformer.scale)}x
				</span>
				<Tabs.Trigger
					sideEffect={() => transformer.zoom(100)}
					disabled={!image.initialised || transformer.atMinScale()}
				>
					<Icon variant="zoom-out" width={ICON_SIZE} height={ICON_SIZE} />
				</Tabs.Trigger>
			</Tabs.List>
		</Tabs.Root>

		<Tabs.Root mode="<=1" {classes}>
			<Tabs.List>
				<Tabs.Trigger value="explorer">
					<Icon variant="explorer" width={ICON_SIZE} height={ICON_SIZE} />
				</Tabs.Trigger>
				<Tabs.Trigger value="control" disabled={!image.initialised}>
					<Icon variant="control" width={ICON_SIZE} height={ICON_SIZE} />
				</Tabs.Trigger>
				<Tabs.Trigger value="info" disabled={!image.initialised}>
					<Icon variant="info" width={ICON_SIZE} height={ICON_SIZE} />
				</Tabs.Trigger>
			</Tabs.List>
			<Tabs.Content value="explorer">
				<Explorer />
			</Tabs.Content>
			<Tabs.Content value="control" disabled={!image.initialised}>
				<AnnotationControls />
			</Tabs.Content>
		</Tabs.Root>

		<Tabs.Root mode="1" initialTab="move" {classes}>
			<Tabs.List>
				<Tabs.Trigger value="move" sideEffect={undefined} disabled={false}>
					<Icon variant="cursor" width={ICON_SIZE} height={ICON_SIZE} />
				</Tabs.Trigger>
				<Tabs.Trigger value="freehand-draw" sideEffect={undefined} disabled={true}>
					<Icon variant="freehand" width={ICON_SIZE} height={ICON_SIZE} />
				</Tabs.Trigger>
				<Tabs.Trigger value="square" sideEffect={undefined} disabled={true}>
					<Icon variant="square" width={ICON_SIZE} height={ICON_SIZE} />
				</Tabs.Trigger>
			</Tabs.List>
		</Tabs.Root>

		<div class="group panel anchor-bottom">
			<button>
				<Icon variant="settings" width={ICON_SIZE} height={ICON_SIZE} />
			</button>
		</div>
	</div>
</nav>

<style lang="scss">
	nav {
		display: block;
		position: absolute;

		top: 0;
		bottom: 0;
		right: 0;

		margin: 10px;

		pointer-events: none;
	}

	.groups {
		display: flex;
		flex-direction: column;

		height: 100%;
		gap: 10px;
	}

	.group {
		display: flex;
		flex-direction: column;
		gap: 2px;
		padding: 3px;
	}

	.anchor-bottom {
		margin-top: auto;
	}

	button,
	:global(.control-panel-tab-trigger) {
		border-radius: 7px;
		height: 3rem;
		height: 3rem;

		padding: 5px;

		&:hover {
			backdrop-filter: blur(15px);
			background-color: rgba(255, 255, 255, 0.1);
		}
	}

	:global(.control-panel-tab-trigger-active) {
		background-color: rgba(255, 255, 255, 0.1);
		&:hover {
			background-color: rgba(255, 255, 255, 0.1);
		}
	}

	:global(.control-panel-tab-trigger-disabled) {
		opacity: 0.3;
		cursor: default;
	}

	:global(.control-panel-tab-list) {
		display: flex;
		flex-direction: column;
		gap: 2px;
		padding: 3px;
	}

	:global(.control-panel-tab-content) {
		padding: 0;
		position: absolute;

		right: 0;
		margin-right: 65px;
		width: auto;
	}
</style>
