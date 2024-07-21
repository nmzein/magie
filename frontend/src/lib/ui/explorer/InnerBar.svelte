<script lang="ts">
	import { explorer } from '$states';
	import Icon from '$icon';

	import * as Dropdown from '$components/dropdown/index.ts';

	const ICON_SIZE = 1.15;
	const classes = {
		trigger: 'flex-row dropdown-trigger-button',
		list: 'flex-column dropdown-content',
		item: 'flex-row dropdown-content-button'
	};
</script>

{#snippet DownArrow()}
	<div class="down-arrow">
		<Icon variant="down-arrow" width={ICON_SIZE} height={ICON_SIZE} />
	</div>
{/snippet}

<div id="inner-bar" class="flex-row light-layer">
	<Dropdown.Root {classes}>
		<Dropdown.Trigger>
			<Icon variant="new" width={ICON_SIZE} height={ICON_SIZE} /> New
			{@render DownArrow()}
		</Dropdown.Trigger>
		<Dropdown.List>
			<Dropdown.Item onclick={() => (explorer.showUploader = true)}>
				<Icon variant="new-image" width={ICON_SIZE} height={ICON_SIZE} /> Image
			</Dropdown.Item>
			<Dropdown.Item onclick={() => (explorer.showDirectoryCreator = true)}>
				<Icon variant="directory" width={ICON_SIZE} height={ICON_SIZE} /> Directory
			</Dropdown.Item>
		</Dropdown.List>
	</Dropdown.Root>

	<Dropdown.Root {classes}>
		<Dropdown.Trigger>
			<Icon variant="view" width={ICON_SIZE} height={ICON_SIZE} /> View
			{@render DownArrow()}
		</Dropdown.Trigger>
		<Dropdown.List>
			<Dropdown.Item onclick={() => {}}>
				<Icon variant="list-view" width={ICON_SIZE} height={ICON_SIZE} /> List
			</Dropdown.Item>
			<Dropdown.Item onclick={() => {}}>
				<Icon variant="grid-view" width={ICON_SIZE} height={ICON_SIZE} /> Grid
			</Dropdown.Item>
		</Dropdown.List>
	</Dropdown.Root>

	<Dropdown.Root {classes}>
		<Dropdown.Trigger>
			<Icon variant="sort" width={ICON_SIZE} height={ICON_SIZE} /> Sort
			{@render DownArrow()}
		</Dropdown.Trigger>
		<Dropdown.List>
			<Dropdown.Item onclick={() => {}}>Name</Dropdown.Item>
			<Dropdown.Item onclick={() => {}}>Date Created</Dropdown.Item>
		</Dropdown.List>
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
