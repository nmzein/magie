<script lang="ts">
	import { explorer } from '$states';
	import Icon from '$icon';

	import * as Dropdown from '$ui/dropdown/index.ts';

	const ICON_SIZE = 1.25;

	let showNew = $state(false);
	let showView = $state(false);
	let showSort = $state(false);
</script>

<div id="inner-bar" class="flex-row light-layer">
	<Dropdown.Root>
		<Dropdown.Trigger class="flex-row dropdown-trigger-button" bind:showContent={showNew}>
			<Icon variant="new" width={ICON_SIZE} height={ICON_SIZE} /> New
			<div class="down-arrow">
				<Icon variant="down-arrow" width={ICON_SIZE} height={ICON_SIZE} />
			</div>
		</Dropdown.Trigger>
		<Dropdown.Content class="flex-column dropdown-content" bind:showContent={showNew}>
			<button
				style="position: relative;"
				class="flex-row dropdown-content-button"
				onclick={() => {
					explorer.showUploader = true;
					showNew = false;
				}}
			>
				<Icon variant="new-image" width={ICON_SIZE} height={ICON_SIZE} /> Image
			</button>
			<button
				class="flex-row dropdown-content-button"
				onclick={() => {
					explorer.showDirectoryCreator = true;
					showNew = false;
				}}
			>
				<Icon variant="directory" width={ICON_SIZE} height={ICON_SIZE} /> Directory
			</button>
		</Dropdown.Content>
	</Dropdown.Root>

	<Dropdown.Root>
		<Dropdown.Trigger class="flex-row dropdown-trigger-button" bind:showContent={showView}>
			<Icon variant="view" width={ICON_SIZE} height={ICON_SIZE} /> View
			<div class="down-arrow">
				<Icon variant="down-arrow" width={ICON_SIZE} height={ICON_SIZE} />
			</div>
		</Dropdown.Trigger>
		<Dropdown.Content class="flex-column dropdown-content" bind:showContent={showView}>
			<button class="flex-row dropdown-content-button">
				<Icon variant="list-view" width={ICON_SIZE} height={ICON_SIZE} /> List
			</button>
			<button class="flex-row dropdown-content-button">
				<Icon variant="grid-view" width={ICON_SIZE} height={ICON_SIZE} /> Grid
			</button>
		</Dropdown.Content>
	</Dropdown.Root>

	<Dropdown.Root>
		<Dropdown.Trigger class="flex-row dropdown-trigger-button" bind:showContent={showSort}>
			<Icon variant="sort" width={ICON_SIZE} height={ICON_SIZE} /> Sort
			<div class="down-arrow">
				<Icon variant="down-arrow" width={ICON_SIZE} height={ICON_SIZE} />
			</div>
		</Dropdown.Trigger>
		<Dropdown.Content class="flex-column dropdown-content" bind:showContent={showSort}>
			<button class="flex-row dropdown-content-button">Name</button>
			<button class="flex-row dropdown-content-button">Date Created</button>
		</Dropdown.Content>
	</Dropdown.Root>
</div>

<style lang="scss">
	#inner-bar {
		z-index: 2;
		position: relative;
		padding: 0 1px;
	}

	.down-arrow {
		opacity: 40%;
		display: flex;
	}

	:global(.dropdown-trigger-button) {
		margin: 4px;
		padding: 7.5px 5px 7.5px 7.5px;
		border-radius: 5px;
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
		margin-left: 4px;
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
