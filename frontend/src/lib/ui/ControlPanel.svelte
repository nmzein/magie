<script lang="ts">
	import ZoomIn from '~icons/mdi/plus';
	import ZoomOut from '~icons/mdi/minus';

	import Cursor from '~icons/material-symbols/point-scan-rounded';
	import Freehand from '~icons/material-symbols-light/draw';
	import Square from '~icons/material-symbols-light/square-outline';

	import Folder from '~icons/material-symbols-light/folder-rounded';
	import AnnotationControl from '~icons/codicon/settings';
	import Info from '~icons/material-symbols-light/info-rounded';

	import Settings from '~icons/material-symbols-light/settings-outline-rounded';

	import * as Tabs from '$ui/tabs/index.ts';

	import { Explorer } from './explorer';
	import ImageControls from '$ui/ImageControls.svelte';
	// import Uploader from '$ui/Uploader.svelte';

	const ICON_SIZE = '2.3em';

	let currentTab = $state(undefined);
</script>

<nav>
	<div class="groups">
		<div class="group panel">
			<button>
				<ZoomIn width={ICON_SIZE} height={ICON_SIZE} />
			</button>
			<span style="text-align: center; margin: 5px 0;"> 1x </span>
			<button>
				<ZoomOut width={ICON_SIZE} height={ICON_SIZE} />
			</button>
		</div>
		<Tabs.Root>
			<Tabs.List class="panel control-panel-tab-list">
				<Tabs.Trigger
					value="explorer"
					bind:currentTab
					class="control-panel-tab-trigger"
					activeClass="control-panel-tab-trigger-active"
				>
					<Folder width={ICON_SIZE} height={ICON_SIZE} />
				</Tabs.Trigger>
				<Tabs.Trigger
					value="control"
					bind:currentTab
					class="control-panel-tab-trigger"
					activeClass="control-panel-tab-trigger-active"
				>
					<AnnotationControl width={ICON_SIZE} height={ICON_SIZE} />
				</Tabs.Trigger>
				<Tabs.Trigger
					value=""
					bind:currentTab
					class="control-panel-tab-trigger"
					activeClass="control-panel-tab-trigger-active"
				>
					<Info width={ICON_SIZE} height={ICON_SIZE} />
				</Tabs.Trigger>
			</Tabs.List>
			<Tabs.Content value="explorer" {currentTab} class="control-panel-tab-content">
				<Explorer />
			</Tabs.Content>
			<Tabs.Content value="control" {currentTab} class="control-panel-tab-content">
				<!-- <Uploader /> -->
				<ImageControls />
			</Tabs.Content>
		</Tabs.Root>
		<div class="group panel">
			<button>
				<Cursor width={ICON_SIZE} height={ICON_SIZE} />
			</button>
			<button>
				<Freehand width={ICON_SIZE} height={ICON_SIZE} />
			</button>
			<button>
				<Square width={ICON_SIZE} height={ICON_SIZE} />
			</button>
		</div>

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
