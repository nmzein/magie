<script lang="ts">
	import { onMount } from 'svelte';
	import FileExplorer from './FileExplorer.svelte';
	import FinetuneControls from './FinetuneControls.svelte';
	import type { ImageSelection } from '$lib/types';
	import { WebSocketStore } from '$lib/stores';
	import { GetAnnotations } from '$lib/api';

	let View = 'FileExplorer';
	let ShowLargePanel = true;
	let socket: WebSocket;
	let selection: ImageSelection = { start: { x: 0, y: 0 }, end: { x: 2, y: 2 } };
	$: rotation = ShowLargePanel ? '0deg' : '180deg';

	onMount(() => {
		const UnsubscribeWebSocketStore = WebSocketStore.subscribe((value) => {
			socket = value as WebSocket;
		});

		return () => {
			UnsubscribeWebSocketStore();
		};
	});
</script>

<nav>
	<div id="container">
		{#if ShowLargePanel}
			<div class="panel large">
				<div style="margin-bottom: 8px;">
					<button class="panel-page-button" on:click={() => (View = 'FileExplorer')}>FILES</button>
					<button class="panel-page-button" on:click={() => (View = 'FinetuneControls')}
						>IMAGE</button
					>
				</div>

				{#if View === 'FileExplorer'}
					<FileExplorer />
				{:else if View === 'FinetuneControls'}
					<FinetuneControls />
				{/if}
			</div>
		{:else}
			<div class="hidden large" />
		{/if}

		<div class="panel small">
			<div style="flex: 1;" />
			<button on:click={() => socket.send(JSON.stringify(selection))}>IMG</button>
			<button on:click={() => GetAnnotations()}>ANNO</button>
			<button id="show-panel" on:click={() => (ShowLargePanel = !ShowLargePanel)}
				><img
					id="arrow-icon"
					src="/icons8-chevron-26.png"
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

		// color: black;
		// width: 70px;
		// letter-spacing: -0.01rem;
		// background: rgba(255, 255, 255, 1);
		// backdrop-filter: blur(15px);

		// &:hover {
		// 	background-color: rgba(245, 245, 245, 1);
		// }
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

		// & > p {
		// 	display: inline-block;
		// 	transform: translateY(-18.75px);
		// }
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

	// dialog {
	// 	min-width: 400px;
	// 	min-height: 200px;
	// 	padding: 20px;
	// }

	// form {
	// 	display: flex;
	// 	flex-direction: column;
	// 	gap: 20px;
	// 	text-align: right;
	// }

	// h1 {
	// 	text-align: left;
	// }

	// input[type='file'] {
	// 	display: none;
	// }

	// input[type='text'] {
	// 	width: 100%;
	// 	height: 40px;
	// 	padding: 0 10px;

	// 	color: white;
	// 	border: none;
	// 	font-size: 15px;
	// 	// backdrop-filter: blur(15px);
	// 	// background: rgba(255, 255, 255, 0.15);
	// 	// box-shadow: 0 15px 15px rgba(0, 0, 0, 0.1);
	// 	background: transparent;

	// 	&:focus {
	// 		outline: none;
	// 	}
	// }

	// .input {
	// 	width: 100%;
	// 	height: 40px;

	// 	backdrop-filter: blur(15px);
	// 	background: rgba(255, 255, 255, 0.15);
	// 	box-shadow: 0 15px 15px rgba(0, 0, 0, 0.1);
	// }

	// .browser-cont {
	// 	display: flex;
	// }

	// .browse {
	// 	border-radius: 15px !important;
	// 	padding: 4px 10px;
	// 	// margin: 10px 0;
	// 	// width: 90px !important;
	// 	// height: 30px !important;
	// 	// backdrop-filter: blur(15px);
	// 	background: rgba(20, 20, 20, 1);
	// 	// box-shadow: 0 15px 15px rgba(0, 0, 0, 0.1);
	// 	border: none;
	// 	cursor: pointer;

	// 	color: white;
	// 	font-weight: 500;
	// 	font-size: 14px;

	// 	&:hover {
	// 		background-color: rgba(10, 10, 10, 1);
	// 	}
	// }

	// #upload-dialog-submit {
	// 	width: 100px;
	// 	align-self: flex-end;
	// }

	// .divider {
	// 	position: absolute;
	// 	height: 30px;
	// 	top: 13px;
	// 	left: 84.7%;
	// 	z-index: 1;
	// 	border-left: 2px solid rgba(255, 255, 255, 0.1);
	// 	pointer-events: none;
	// }

	// .arrow {
	// 	position: absolute;
	// 	top: 18px;
	// 	left: 78.3%;
	// 	z-index: 10;
	// 	pointer-events: none;

	// 	& > img {
	// 		width: 15px;
	// 	}
	// }

	// #target {
	// 	display: flex;
	// 	width: 100%;
	// 	height: 40px;
	// }

	// #target > select {
	// 	flex: 11;
	// 	padding: 0 10px;

	// 	backdrop-filter: blur(15px);
	// 	background: rgba(255, 255, 255, 0.15);
	// 	box-shadow: 0 15px 15px rgba(0, 0, 0, 0.1);

	// 	&:hover {
	// 		background-color: rgba(255, 255, 255, 0.1);
	// 	}

	// 	// -moz-appearance: none; /* Firefox */
	// 	-webkit-appearance: none; /* Safari and Chrome */
	// 	appearance: none;
	// }

	// #target > button {
	// 	font-size: 24px;
	// 	flex: 1;
	// }

	// .first {
	// 	border-radius: 8px 0 0 8px;
	// }

	// .last {
	// 	border-radius: 0 8px 8px 0;
	// }
</style>
