<script lang="ts">
	import Icon from '$icon';

	let {
		asset = $bindable(),
		placeholder
	}: {
		asset: File | undefined;
		placeholder: string;
	} = $props();

	function handleDrop(event: DragEvent) {
		event.preventDefault();
		setAsset(event.dataTransfer?.files);
	}

	function handleBrowse(event: Event) {
		setAsset((event.target as HTMLInputElement).files);
	}

	function setAsset(files: FileList | undefined | null) {
		if (files && files.length > 0) {
			asset = files[0];
		}
	}
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div ondrop={(e) => handleDrop(e)} ondragover={(e) => e.preventDefault()}>
	{#if asset}
		<button id="asset" class="populated" onclick={() => (asset = undefined)}>
			<Icon variant="image" width={5} height={5} />
			<span>{asset.name}</span>
		</button>
	{:else}
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<label
			id="asset"
			class="unpopulated"
			for={'browse-input-' + placeholder}
			onclick={(e) => handleBrowse(e)}
		>
			<Icon variant="image" width={5} height={5} />
			<span class="grey-text">{placeholder}</span>
		</label>

		<input
			id={'browse-input-' + placeholder}
			type="file"
			style="display: none;"
			onchange={(e) => handleBrowse(e)}
		/>
	{/if}
</div>

<style lang="scss">
	div {
		display: flex;
		flex: 1;
		justify-content: center;
		align-items: center;
		height: 100%;
		width: 100%;

		color: white;
		background: transparent;
		border-radius: 5px;

		&:focus {
			outline: none;
		}
	}

	#asset {
		display: flex;
		flex: 1;
		flex-direction: column;
		justify-content: center;
		align-items: center;

		border-radius: 5px;
		gap: 10px;
		height: 100%;

		cursor: pointer;
	}

	.populated {
		background-color: rgba(255, 255, 255, 0.1);
	}

	.unpopulated {
		border: 2px dashed rgba(255, 255, 255, 0.7);
	}

	// img {
	// 	width: 60px;
	// 	height: 60px;
	// 	margin-bottom: 10px;
	// 	user-select: none;
	// }

	span {
		// width: 130px;
		white-space: nowrap;
		overflow: hidden;
		user-select: none;
		text-overflow: ellipsis;
		text-align: center;
		font-size: 14px;
	}
</style>
