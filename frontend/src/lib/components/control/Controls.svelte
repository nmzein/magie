<script lang="ts">
	import FileExplorer from '$control/FileExplorer.svelte';
	import Uploader from '$control/Uploader.svelte';
	import ImageControls from '$control/ImageControls.svelte';

	let pages = ['FILES', 'IMAGE'];
	let currentView = $state(pages[0]);
	let showLargePanel = $state(true);
</script>

<nav>
	<div id="container">
		<div class="{showLargePanel ? 'panel' : 'hidden'} large">
			<div style="display: flex; flex-direction: column; gap: 8px;">
				<div style="display: flex; gap: 6px;">
					{#each pages as page}
						<button class="panel-page-button" onclick={() => (currentView = page)}>{page}</button>
					{/each}
				</div>
				{#if currentView === pages[0]}
					<FileExplorer />
					<Uploader />
				{:else if currentView === pages[1]}
					<ImageControls />
				{/if}
			</div>
		</div>

		<div class="panel small">
			<div style="flex: 1;" />
			<button onclick={() => (showLargePanel = !showLargePanel)}
				><img
					id="arrow"
					src="arrow.png"
					alt="Show large panel."
					style="--rotation:{showLargePanel ? '0deg' : '180deg'}"
				/></button
			>
		</div>
	</div>
</nav>

<style lang="scss">
	.panel-page-button {
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

	.small img {
		width: 20px;
		height: 20px;
	}

	#arrow {
		transform: translateY(2px) rotate(var(--rotation));
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
		flex: 4;
		min-height: 40px;
		padding: 5px;
		display: flex;
		gap: 5px;

		button {
			height: 100%;
		}
	}
</style>
