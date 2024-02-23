<script lang="ts">
	import FileExplorer from '$control/FileExplorer.svelte';
	import Uploader from '$control/Uploader.svelte';
	import ImageControls from '$control/ImageControls.svelte';

	let views = ['FILES', 'IMAGE'];
	let currentView = $state(views[0]);
	let showLargePanel = $state(true);

	let hidden = $derived(showLargePanel ? 'panel' : 'hidden');
	let rotation = $derived(showLargePanel ? '0deg' : '180deg');
</script>

<nav>
	<div id="container">
		<div class="large {hidden}">
			<div style="display: flex; flex-direction: column; gap: 8px;">
				<div style="display: flex; gap: 6px;">
					{#each views as view}
						<button class="panel-view-button" onclick={() => (currentView = view)}>{view}</button>
					{/each}
				</div>
				{#if currentView === views[0]}
					<FileExplorer />
					<Uploader />
				{:else if currentView === views[1]}
					<ImageControls />
				{/if}
			</div>
		</div>

		<div class="panel small">
			<button><img class="zoom" src="plus.svg" alt="Zoom in." /></button>
			<button><img class="zoom" src="minus.svg" alt="Zoom out." /></button>
			<div style="flex: 1;" />
			<button onclick={() => (showLargePanel = !showLargePanel)}
				><img
					id="arrow"
					src="arrow.png"
					alt="Show large panel."
					style="--rotation: {rotation}"
				/></button
			>
		</div>
	</div>
</nav>

<style lang="scss">
	.panel-view-button {
		height: 30px;
		border-radius: 10px;
		font-weight: 600;
		font-size: 14px;
	}

	nav {
		display: block;
		position: absolute;

		top: 0;
		bottom: 0;
		right: 0;

		width: 350px;
		margin: 10px;

		pointer-events: none;
	}

	button {
		border-radius: 30px;
		height: 40px;
		padding: 0 10px;
		backdrop-filter: blur(15px);
		background: rgba(255, 255, 255, 0.15);
		box-shadow: 0 15px 15px rgba(0, 0, 0, 0.1);

		&:hover {
			background-color: rgba(255, 255, 255, 0.1);
		}
	}

	// .small img {
	// }

	.zoom {
		width: 15px;
		height: 15px;
		padding: 2.5px;
		filter: invert(100%) sepia(100%) saturate(1%) hue-rotate(188deg) brightness(101%) contrast(101%);
	}

	#arrow {
		width: 20px;
		height: 20px;
		transform: rotate(var(--rotation));
	}

	#container {
		display: flex;
		flex-direction: column;

		height: 100%;
		gap: 10px;
	}

	.large {
		flex: 96;
		padding: 8px;
	}

	.small {
		display: flex;
		flex: 4;
		min-height: 40px;
		padding: 5px;
		gap: 5px;

		button {
			height: 100%;
			padding: 10px;
		}
	}
</style>
