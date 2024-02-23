<script lang="ts">
	import { SendUploadAssets } from '$api';
	import {
		imageUpload,
		annotationsUpload,
		generators,
		selectedGenerator,
		autogenerateAnnotations
	} from '$stores';
	import Switch from '$control/Switch.svelte';
	import UploadAsset from '$control/UploadAsset.svelte';

	// TODO: Get path from file explorer.
	let directory_path = '';

	function handleUpload() {
		if (!imageUpload.value || !selectedGenerator.value) {
			alert('Please provide an image and/or select an annotation generator.');
			return;
		}

		// Still need to do this check as the user may have
		// uploaded an annotation file earlier and then
		// switched to autogeneration.
		if (autogenerateAnnotations.value) {
			SendUploadAssets(directory_path, imageUpload.value, undefined, selectedGenerator.value);
			imageUpload.value = undefined;
		} else if (annotationsUpload) {
			SendUploadAssets(
				directory_path,
				imageUpload.value,
				annotationsUpload.value,
				selectedGenerator.value
			);
			imageUpload.value = undefined;
			annotationsUpload.value = undefined;
		}
	}
</script>

<div>
	<div class="outer-container">
		<div class="inner-container" style="border-radius: 10px 10px 0 0;">
			<span class="secondary-text"> ANNOTATION GENERATION </span>
			<div style="display: flex;">
				<div style="flex: 1; display: flex; gap: 5px; padding-top: 3px;">
					AUTOGENERATE
					<Switch bind:checked={autogenerateAnnotations.value} onclick={undefined} />
				</div>
				<select style="flex: 1;" bind:value={selectedGenerator.value}>
					{#if generators.value}
						{#each generators.value as annotation_generator}
							<option value={annotation_generator}>{annotation_generator}</option>
						{/each}
					{/if}
				</select>
			</div>
		</div>

		<div style="display: flex;">
			<UploadAsset bind:assetUpload={imageUpload.value} placeholder="Image" />

			{#if !autogenerateAnnotations.value}
				<div class="divider" />
				<UploadAsset bind:assetUpload={annotationsUpload.value} placeholder="Annotations" />
			{/if}
		</div>

		<div
			class="inner-container secondary-text"
			style="padding: 5px 10px; font-size: 12px; border-radius: 0 0 10px 10px;"
		>
			Drag file onto/click on icon to browse fs.
		</div>
	</div>

	<div style="display: flex;">
		<div style="flex: 1;" />
		<button type="submit" onclick={() => handleUpload()}>UPLOAD</button>
	</div>
</div>

<style lang="scss">
	.divider {
		margin: 15px 2px;
		border-left: 1px solid rgba(255, 255, 255, 0.2);
		pointer-events: none;
	}

	select {
		width: 130px;
		border-radius: 5px;
		letter-spacing: -0.01rem;
		font-size: 13px;
		padding: 3px 7px;
		background-color: rgba(255, 255, 255, 0.2);
	}

	button {
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
