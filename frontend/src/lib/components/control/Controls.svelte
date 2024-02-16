<script lang="ts">
	import FileControls from '$lib/components/control/FileControls.svelte';
	import ImageControls from '$lib/components/control/ImageControls.svelte';

	let pages = ['FILES', 'IMAGE'];
	let currentView = pages[0];
	let showLargePanel = true;
</script>

<nav>
	<div id="container">
		<div class="{showLargePanel ? 'panel' : 'hidden'} large">
			<div style="display: flex; gap: 6px; margin-bottom: 15px;">
				{#each pages as page}
					<button class="panel-page-button" on:click={() => (currentView = page)}>{page}</button>
				{/each}
			</div>

			{#if currentView === 'FILES'}
				<FileControls />
			{:else if currentView === 'IMAGE'}
				<ImageControls />
			{/if}
		</div>

		<div class="panel small">
			<div style="flex: 1;" />
			<button id="show-panel" on:click={() => (showLargePanel = !showLargePanel)}
				><img
					id="arrow-icon"
					src="/arrow.png"
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
		font-family: 'JetBrains Mono', monospace;
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
		font-family: 'JetBrains Mono', monospace;
		&:hover {
			background-color: rgba(255, 255, 255, 0.1);
		}
	}

	#show-panel > img {
		width: 20px;
		height: 20px;
		transform: translateY(2px) rotate(var(--rotation));
	}

	#container {
		display: flex;
		flex-direction: column;

		height: 100%;
		gap: 10px;
	}

	.hidden {
		visibility: hidden;
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
