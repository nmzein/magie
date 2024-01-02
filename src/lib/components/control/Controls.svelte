<script lang="ts">
	import FileControls from '$lib/components/control/FileControls.svelte';
	import ImageControls from '$lib/components/control/ImageControls.svelte';
	import { GetAnnotations } from '$lib/api';

	let view = 'FileControls';
	let show_large_panel = true;
	$: rotation = show_large_panel ? '0deg' : '180deg';
</script>

<nav>
	<div id="container">
		{#if show_large_panel}
			<div class="panel large">
				<div style="margin-bottom: 15px;">
					<button class="panel-page-button" on:click={() => (view = 'FileControls')}>FILES</button>
					<button class="panel-page-button" on:click={() => (view = 'ImageControls')}>IMAGE</button>
				</div>

				{#if view === 'FileControls'}
					<FileControls />
				{:else if view === 'ImageControls'}
					<ImageControls />
				{/if}
			</div>
		{:else}
			<div class="hidden large" />
		{/if}

		<div class="panel small">
			<div style="flex: 1;" />
			<!-- TODO: Actually fix annotation rendering. -->
			<button on:click={() => GetAnnotations('image-test')}>ANNO</button>
			<button id="show-panel" on:click={() => (show_large_panel = !show_large_panel)}
				><img
					id="arrow-icon"
					src="/arrow.png"
					alt="Show large panel."
					style="--rotation:{rotation}"
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

	.panel {
		color: white;
		border: 1px solid rgba(255, 255, 255, 0.125);
		border-radius: 10px;
		backdrop-filter: blur(15px);
		background: rgba(0, 0, 0, 0.75);
		box-shadow: 0 15px 15px rgba(0, 0, 0, 0.1);
		-webkit-backdrop-filter: blur(16px) saturate(180%);
		pointer-events: all;
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
