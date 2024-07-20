<script lang="ts">
	import ZoomIn from '~icons/mdi/plus';
	import ZoomOut from '~icons/mdi/minus';

	import Cursor from '~icons/material-symbols/point-scan-rounded';
	import Freehand from '~icons/material-symbols-light/draw';
	import Square from '~icons/material-symbols-light/square-outline';

	import Folder from '~icons/material-symbols-light/folder-rounded';
	import Control from '~icons/codicon/settings';
	import Info from '~icons/material-symbols-light/info-rounded';

	import Settings from '~icons/material-symbols-light/settings-outline-rounded';

	import * as Tabs from '$ui/tabs/index.ts';

	import { Explorer } from './explorer';
	import AnnotationControls from '$ui/AnnotationControls.svelte';
	import { image, transformer } from '$states';

	const ICON_SIZE = '2.3em';

	let panelCurrentTab = $state(undefined);
	let cursorCurrentTab = $state('move');
	let fakeState = $state(undefined);

	function formatNumber(num: number, digits: number = 2) {
		// Convert the number to a string with 2 decimal places
		let number = num.toString();

		if (number.includes('.')) {
			let [integer, decimal] = number.split('.');

			if (integer.length >= digits) {
				// For when scale == 101.2
				return integer;
			} else if (integer.length + decimal.length >= digits) {
				// Most cases.
				return integer + '.' + decimal.slice(0, digits - integer.length);
			} else {
				// For when scale is number like 0.1, 6.0, etc.
				return integer + '.' + decimal + '0'.repeat(digits - integer.length - decimal.length);
			}
		} else {
			let integer = number;

			if (integer.length >= digits) {
				return integer;
			} else {
				// For when scale is integer like 1.
				return integer + '.' + '0'.repeat(digits - integer.length);
			}
		}
	}
</script>

<nav>
	<div class="groups">
		<Tabs.Root>
			<Tabs.List class="panel control-panel-tab-list">
				<Tabs.Trigger
					mode="0"
					value=""
					sideEffect={() => transformer.zoom(-100)}
					bind:currentTab={fakeState}
					disabled={!image.initialised || transformer.atMaxScale()}
					class="control-panel-tab-trigger"
					activeClass="control-panel-tab-trigger-active"
					disabledClass="control-panel-tab-trigger-disabled"
				>
					<ZoomIn width={ICON_SIZE} height={ICON_SIZE} />
				</Tabs.Trigger>
				<!-- svelte-ignore a11y_no_static_element_interactions -->
				<!-- svelte-ignore a11y_click_events_have_key_events -->
				<span
					onclick={() => {
						if (!image.initialised) return;
						transformer.resetScale();
					}}
					style="text-align: center; margin: 5px 0; user-select: none;
						   {image.initialised ? 'cursor: pointer' : ''};"
					class:control-panel-tab-trigger-disabled={!image.initialised}
				>
					{formatNumber(transformer.scale)}x
				</span>
				<Tabs.Trigger
					mode="0"
					value=""
					sideEffect={() => transformer.zoom(100)}
					bind:currentTab={fakeState}
					disabled={!image.initialised || transformer.atMinScale()}
					class="control-panel-tab-trigger"
					activeClass="control-panel-tab-trigger-active"
					disabledClass="control-panel-tab-trigger-disabled"
				>
					<ZoomOut width={ICON_SIZE} height={ICON_SIZE} />
				</Tabs.Trigger>
			</Tabs.List>
		</Tabs.Root>

		<Tabs.Root>
			<Tabs.List class="panel control-panel-tab-list">
				<Tabs.Trigger
					mode="<=1"
					value="explorer"
					sideEffect={undefined}
					bind:currentTab={panelCurrentTab}
					disabled={false}
					class="control-panel-tab-trigger"
					activeClass="control-panel-tab-trigger-active"
					disabledClass="control-panel-tab-trigger-disabled"
				>
					<Folder width={ICON_SIZE} height={ICON_SIZE} />
				</Tabs.Trigger>
				<Tabs.Trigger
					mode="<=1"
					value="control"
					sideEffect={undefined}
					bind:currentTab={panelCurrentTab}
					disabled={!image.initialised}
					class="control-panel-tab-trigger"
					activeClass="control-panel-tab-trigger-active"
					disabledClass="control-panel-tab-trigger-disabled"
				>
					<Control width={ICON_SIZE} height={ICON_SIZE} />
				</Tabs.Trigger>
				<Tabs.Trigger
					mode="<=1"
					value="info"
					sideEffect={undefined}
					bind:currentTab={panelCurrentTab}
					disabled={!image.initialised}
					class="control-panel-tab-trigger"
					activeClass="control-panel-tab-trigger-active"
					disabledClass="control-panel-tab-trigger-disabled"
				>
					<Info width={ICON_SIZE} height={ICON_SIZE} />
				</Tabs.Trigger>
			</Tabs.List>
			<Tabs.Content
				value="explorer"
				currentTab={panelCurrentTab}
				disabled={false}
				class="control-panel-tab-content"
			>
				<Explorer />
			</Tabs.Content>
			<Tabs.Content
				value="control"
				currentTab={panelCurrentTab}
				disabled={!image.initialised}
				class="control-panel-tab-content"
			>
				<AnnotationControls />
			</Tabs.Content>
		</Tabs.Root>

		<Tabs.Root>
			<Tabs.List class="panel control-panel-tab-list">
				<Tabs.Trigger
					mode="=1"
					value="move"
					sideEffect={undefined}
					bind:currentTab={cursorCurrentTab}
					disabled={false}
					class="control-panel-tab-trigger"
					activeClass="control-panel-tab-trigger-active"
					disabledClass="control-panel-tab-trigger-disabled"
				>
					<Cursor width={ICON_SIZE} height={ICON_SIZE} />
				</Tabs.Trigger>
				<Tabs.Trigger
					mode="=1"
					value="freehand-draw"
					sideEffect={undefined}
					bind:currentTab={cursorCurrentTab}
					disabled={true}
					class="control-panel-tab-trigger"
					activeClass="control-panel-tab-trigger-active"
					disabledClass="control-panel-tab-trigger-disabled"
				>
					<Freehand width={ICON_SIZE} height={ICON_SIZE} />
				</Tabs.Trigger>
				<Tabs.Trigger
					mode="=1"
					value="square"
					sideEffect={undefined}
					bind:currentTab={cursorCurrentTab}
					disabled={true}
					class="control-panel-tab-trigger"
					activeClass="control-panel-tab-trigger-active"
					disabledClass="control-panel-tab-trigger-disabled"
				>
					<Square width={ICON_SIZE} height={ICON_SIZE} />
				</Tabs.Trigger>
			</Tabs.List>
		</Tabs.Root>

		<div class="group panel anchor-bottom">
			<button>
				<Settings width={ICON_SIZE} height={ICON_SIZE} />
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

	button {
		border-radius: 8px;
		height: 3em;

		padding: 5px;

		&:hover {
			backdrop-filter: blur(15px);
			background-color: rgba(255, 255, 255, 0.1);
		}
	}

	:global(.control-panel-tab-trigger) {
		border-radius: 8px;
		height: 3em;

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
		margin-right: 57px;
		width: auto;
	}
</style>
