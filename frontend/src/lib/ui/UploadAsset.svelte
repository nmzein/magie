<script lang="ts">
	let {
		assetUpload = $bindable(),
		placeholder
	}: {
		assetUpload: File | undefined;
		placeholder: string;
	} = $props();

	function handleDrop(event: DragEvent) {
		event.preventDefault();

		const files = event.dataTransfer?.files;
		if (files && files.length > 0) {
			assetUpload = files[0];
		}
	}

	function handleBrowse(event: Event) {
		const files = (event.target as HTMLInputElement).files;
		if (files && files.length > 0) {
			assetUpload = files[0];
		}
	}
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div ondrop={(e) => handleDrop(e)} ondragover={(e) => e.preventDefault()}>
	{#if assetUpload}
		<button
			id="asset"
			style="background-color: rgba(255, 255, 255, 0.1);"
			onclick={() => (assetUpload = undefined)}
		>
			<img src="default_file.svg" alt="" />
			<span>{assetUpload.name}</span>
		</button>
	{:else}
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<label id="asset" for={'browse-input-' + placeholder} onclick={(e) => handleBrowse(e)}>
			<img src="default_file.svg" alt="" />
			<span class="grey-text">{placeholder}</span>
		</label>

		<input
			id={'browse-input-' + placeholder}
			type="file"
			style="display: none"
			onchange={(e) => handleBrowse(e)}
		/>
	{/if}
</div>

<style lang="scss">
	div {
		display: flex;
		flex: 1;
		align-items: center;

		color: white;
		border: none;
		font-size: 13px;
		background: transparent;

		&:focus {
			outline: none;
		}
	}

	#asset {
		display: flex;
		flex: 1;
		flex-direction: column;
		align-items: center;

		padding: 10px;
		border-radius: 5px;
		margin: 5px;

		cursor: pointer;
	}

	img {
		width: 60px;
		height: 60px;
		margin-bottom: 10px;
		user-select: none;
	}

	span {
		width: 130px;
		white-space: nowrap;
		overflow: hidden;
		user-select: none;
		text-overflow: ellipsis;
		text-align: center;
		font-size: 13px;
	}
</style>
