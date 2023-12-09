<script lang="ts">
	import { sendProcessRequest } from '$lib/api';

	let showLargePanel: boolean = false;
</script>

<nav>
	<div id="container">
		{#if showLargePanel}
			<div class="panel large">
				<!-- dropdown input field with "image-1" option -->
				<div id="target">
					<select class="first">
						<option>image-1</option>
					</select>

					<div class="divider" />
					<div class="arrow"><img src="/icons8-chevron-26.png" alt="" /></div>

					<button class="last" id="upload-image" on:click={() => sendProcessRequest()}
						><p>+</p></button
					>
				</div>
			</div>
		{:else}
			<div class="hidden large" />
		{/if}
		<div class="panel small">
			<div class="placeholder" />
			<button
				id="show-panel"
				on:click={() => {
					showLargePanel = !showLargePanel;
					const arrowIcon = document.getElementById('arrow-icon');
					if (arrowIcon) {
						if (showLargePanel) {
							arrowIcon.style.transform = 'translateY(1px) rotate(0deg)';
						} else {
							arrowIcon.style.transform = 'translateY(1px) rotate(180deg)';
						}
					}
				}}><img id="arrow-icon" src="/icons8-chevron-26.png" alt="Show large panel." /></button
			>
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

	button > p {
		display: inline-block;
		transform: translateY(-18px);
	}

	.divider {
		position: absolute;
		height: 30px;
		top: 13px;
		left: 84.7%;
		z-index: 10;
		border-left: 2px solid rgba(255, 255, 255, 0.1);
		pointer-events: none;
	}

	.arrow {
		position: absolute;
		top: 18px;
		left: 78.3%;
		z-index: 10;
		pointer-events: none;

		& > img {
			width: 15px;
		}
	}

	#show-panel > img {
		width: 20px;
		height: 20px;
		transform: translateY(1px) rotate(180deg);
	}

	.rotate-arrow {
		transform: translateY(1px) rotate(180deg);
	}

	#container {
		display: flex;
		flex-direction: column;

		height: 100%;
		gap: 10px;
	}

	#target {
		display: flex;
		width: 100%;
		height: 40px;
		// gap: 2px;
	}

	#target > select {
		flex: 11;
		padding: 0 10px;

		backdrop-filter: blur(15px);
		background: rgba(255, 255, 255, 0.15);
		box-shadow: 0 15px 15px rgba(0, 0, 0, 0.1);

		&:hover {
			background-color: rgba(255, 255, 255, 0.1);
		}

		// -moz-appearance: none; /* Firefox */
		-webkit-appearance: none; /* Safari and Chrome */
		appearance: none;
	}

	#target > button {
		font-size: 24px;
		flex: 1;
	}

	.panel {
		background-color: rgba(0, 0, 0, 0.75);
		border: 1px solid rgba(255, 255, 255, 0.125);
		border-radius: 10px;
		backdrop-filter: blur(16px) saturate(180%);
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

		button {
			height: 100%;
		}
	}

	.placeholder {
		flex: 1;
	}

	.first {
		border-radius: 8px 0 0 8px;
	}

	.last {
		border-radius: 0 8px 8px 0;
	}
</style>
