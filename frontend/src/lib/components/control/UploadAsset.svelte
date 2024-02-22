<script lang="ts">
	let { assetUpload, inputStyle, labelStyle, accept, handleDrop } = $props<{
		assetUpload: File | undefined;
		inputStyle: string;
		labelStyle: string;
		accept: string;
		handleDrop: (e: DragEvent) => void;
	}>();
	// TODO: Add length limit to name before truncating.
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div style="display: flex;" ondrop={(e) => handleDrop(e)}>
	<input
		type="text"
		placeholder={assetUpload ? '' : 'Drop file here or browse your filesystem.'}
		style={inputStyle}
		ondragover={(e) => e.preventDefault()}
		readonly
	/>
	{#if assetUpload}
		<div id="wrapper">
			<!-- svelte-ignore a11y-click-events-have-key-events -->
			<!-- svelte-ignore a11y-no-static-element-interactions -->
			<div id="asset" onclick={() => (assetUpload = undefined)}>{assetUpload.name} X</div>
		</div>
	{/if}
	<label style={labelStyle}>
		<div class="divider" />
		<img src="/folder.svg" alt="Browse filesystem." />
		<input type="file" {accept} />
	</label>
</div>

<style lang="scss">
	#wrapper {
		position: absolute;
		width: 100%;
	}

	#asset {
		position: absolute;
		background-color: #5c5c5c;
		border-radius: 10px;
		padding: 5px;
		margin-top: 5px;
		margin-left: 5px;
		font-family: 'JetBrains Mono', monospace;
		cursor: pointer;
	}

	input[type='text'] {
		flex: 1;
		height: 40px;
		padding: 0 10px;

		color: white;
		border: none;
		font-size: 13px;
		background: transparent;

		&:focus {
			outline: none;
		}
	}

	.divider {
		position: absolute;
		height: 30px;
		top: 10%;
		left: -2%;
		z-index: 1;
		border-left: 2px solid rgba(255, 255, 255, 0.2);
		pointer-events: none;
	}

	label {
		cursor: pointer;

		height: 40px;
		width: 40px;
		position: absolute;
		right: 0;

		&:hover {
			background-color: rgba(0, 0, 0, 0.2);
		}
	}

	input[type='file'] {
		display: none;
	}

	img {
		width: 20px;
		height: 20px;
		margin: 10px;
	}
</style>
