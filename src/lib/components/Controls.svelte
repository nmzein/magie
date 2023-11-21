<script lang="ts">
	import { sendProcessRequest } from '$lib/api';

	let showLargePanel: boolean = false;
</script>

<nav>
	<div id="container">
		{#if showLargePanel}
			<div class="panel large">
				<button on:click={() => sendProcessRequest()}>Process</button>
			</div>
		{:else}
			<div class="hidden-panel large" />
		{/if}
		<div class="panel small">
			<button id="first" on:click={() => (showLargePanel = !showLargePanel)}>Show</button>
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

		width: 350px;
		margin: 10px;

		pointer-events: none;
	}

	#container {
		display: flex;
		flex-direction: column;

		height: 100%;
		gap: 10px;
	}

	.panel {
		background-color: rgba(0, 0, 0, 0.75);
		border-radius: 10px;
	}

	.hidden-panel {
		visibility: hidden;
	}

	.large {
		flex: 96;
		padding: 10px;
		pointer-events: all;
	}

	.small {
		flex: 4;
		min-height: 40px;
		padding: 3px;
		pointer-events: all;

		button {
			height: 100%;
		}
	}

	button {
		border-radius: 8px;
		height: 40px;
		padding: 0 10px;

		&#first {
			border-radius: 8px 0 0 8px;
		}

		&#last {
			border-radius: 0 8px 8px 0;
		}
	}
</style>
