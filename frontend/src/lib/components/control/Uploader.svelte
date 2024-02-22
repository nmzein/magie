<script lang="ts">
	import { SendUploadAssets } from '$api';
	import { imageUpload, annotationsUpload, generators } from '$stores';
	import Switch from '$control/Switch.svelte';
	import UploadAsset from '$control/UploadAsset.svelte';

	// TODO: Cache settings choices in stores.
	let selectedGenerator = $state<string>();

	$effect(() => {
		selectedGenerator = generators.value?.[0];
	});

	let autogenerateAnnotations = $state(true);
	// TODO: Get path from file explorer.
	let directory_path = '';

	function handleImage(event: DragEvent) {
		event.preventDefault();

		const file = event.dataTransfer?.files[0];
		imageUpload.value = file;
	}

	function handleAnnotationFile(event: DragEvent) {
		event.preventDefault();

		const file = event.dataTransfer?.files[0];
		annotationsUpload.value = file;
	}

	function handleUpload() {
		if (!imageUpload.value || !selectedGenerator) {
			alert('Please provide an image and/or select an annotation generator.');
			return;
		}

		// Still need to do this check as the user may have
		// uploaded an annotation file earlier and then
		// switched to autogeneration.
		if (autogenerateAnnotations) {
			SendUploadAssets(directory_path, imageUpload.value, undefined, selectedGenerator);
			imageUpload.value = undefined;
		} else if (annotationsUpload) {
			SendUploadAssets(
				directory_path,
				imageUpload.value,
				annotationsUpload.value,
				selectedGenerator
			);
			imageUpload.value = undefined;
			annotationsUpload.value = undefined;
		}
	}
</script>

<div>
	<div class="outer-container">
		<div class="inner-container" style="border-radius: 10px 10px 0 0;">
			<span class="grey-heading"> ANNOTATIONS </span>
			<div style="display: flex;">
				<div style="flex: 1; display: flex; gap: 5px; padding-top: 3px;">
					AUTOGENERATE
					<Switch bind:checked={autogenerateAnnotations} onclick={undefined} />
				</div>
				<select style="flex: 1;" bind:value={selectedGenerator}>
					{#if generators.value}
						{#each generators.value as annotation_generator}
							<option value={annotation_generator}>{annotation_generator}</option>
						{/each}
					{/if}
				</select>
			</div>
		</div>

		<UploadAsset
			bind:assetUpload={imageUpload.value}
			inputStyle={autogenerateAnnotations ? 'border-radius: 0 0 10px 10px' : ''}
			labelStyle={autogenerateAnnotations ? 'border-radius: 0 0 10px 0' : ''}
			accept="image/*"
			handleDrop={handleImage}
		/>

		{#if !autogenerateAnnotations}
			<UploadAsset
				bind:assetUpload={annotationsUpload.value}
				inputStyle="border-radius: 0 0 10px 10px;"
				labelStyle="border-radius: 0 0 10px 0;"
				accept="json/*"
				handleDrop={handleAnnotationFile}
			/>
		{/if}
	</div>

	<div style="display: flex;">
		<div style="flex: 1;" />
		<button type="submit" onclick={() => handleUpload()}>UPLOAD</button>
	</div>
</div>

<style lang="scss">
	select {
		width: 130px;
		border-radius: 5px;
		font-family: 'JetBrains Mono', monospace;
		letter-spacing: -0.01rem;
		font-size: 13px;
		padding: 3px 7px;
		background-color: rgba(255, 255, 255, 0.2);
	}

	button {
		font-family: 'JetBrains Mono', monospace;
		font-size: 14px;
		background-color: rgba(255, 255, 255, 0.2);
		border-radius: 8px;
		margin-top: 8px;
		width: 70px;

		&:hover {
			background-color: rgba(255, 255, 255, 0.15);
		}
	}
</style>
