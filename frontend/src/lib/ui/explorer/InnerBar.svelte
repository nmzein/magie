<script lang="ts">
	import New from '~icons/ant-design/plus-circle-outlined';
	import View from '~icons/material-symbols/view-day-outline';
	import Sort from '~icons/fluent/arrow-sort-28-filled';
	import DownArrow from '~icons/mdi/chevron-down';

	import NewDirectory from '~icons/mdi/folder-add';
	import NewImage from '~icons/uis/upload-alt';

	import ListView from '~icons/ic/round-format-list-bulleted';
	import GridView from '~icons/material-symbols/grid-view-outline-rounded';

	import * as Dropdown from '$ui/dropdown/index.ts';

	const ICON_SIZE = '1.25em';

	let showNew = $state(false);
	let showView = $state(false);
	let showSort = $state(false);
</script>

<div class="flex-row light-layer">
	<Dropdown.Root>
		<Dropdown.Trigger class="flex-row dropdown-trigger-button" bind:showContent={showNew}>
			<New /> New <DownArrow />
		</Dropdown.Trigger>
		<Dropdown.Content class="flex-column dropdown-content" bind:showContent={showNew}>
			<button class="flex-row dropdown-content-button">
				<NewDirectory width={ICON_SIZE} height={ICON_SIZE} /> New Directory
			</button>
			<button class="flex-row dropdown-content-button">
				<NewImage width={ICON_SIZE} height={ICON_SIZE} /> New Image
			</button>
		</Dropdown.Content>
	</Dropdown.Root>
	<Dropdown.Root>
		<Dropdown.Trigger class="flex-row dropdown-trigger-button" bind:showContent={showView}>
			<View /> View <DownArrow />
		</Dropdown.Trigger>
		<Dropdown.Content class="flex-column dropdown-content" bind:showContent={showView}>
			<button class="flex-row dropdown-content-button">
				<ListView width={ICON_SIZE} height={ICON_SIZE} /> List
			</button>
			<button class="flex-row dropdown-content-button">
				<GridView width={ICON_SIZE} height={ICON_SIZE} /> Grid
			</button>
		</Dropdown.Content>
	</Dropdown.Root>
	<Dropdown.Root>
		<Dropdown.Trigger class="flex-row dropdown-trigger-button" bind:showContent={showSort}>
			<Sort /> Sort <DownArrow />
		</Dropdown.Trigger>
		<Dropdown.Content class="flex-column dropdown-content" bind:showContent={showSort}>
			<button class="flex-row dropdown-content-button">Name</button>
			<button class="flex-row dropdown-content-button">Date Created</button>
		</Dropdown.Content>
	</Dropdown.Root>
</div>

<style lang="scss">
	div {
		padding: 0px 5px;
		z-index: 2;
		position: relative;
	}

	:global(.dropdown-trigger-button) {
		padding: 10px 5px 10px 10px;
		gap: 7px;
		align-items: center;
		z-index: 3;

		&:hover {
			background-color: rgba(255, 255, 255, 0.1);
		}
	}

	:global(.dropdown-content) {
		position: absolute;
		margin-top: 4px;
		z-index: 3;

		border-radius: 5px;
		border: 1px solid rgba(255, 255, 255, 0.1);
		background-color: rgba(56, 56, 56, 0.9);
		backdrop-filter: blur(45px);
	}

	:global(.dropdown-content-button) {
		gap: 10px;
		align-items: center;
		margin: 2px;
		padding: 7.5px 10px;
		border-radius: 5px;
		z-index: 4;

		&:hover {
			background-color: rgba(255, 255, 255, 0.1);
		}
	}
</style>
