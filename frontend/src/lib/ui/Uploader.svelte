<!-- <script lang="ts">
	import { SendUploadAssets } from '$api';
	import {
		imageUpload,
		annotationsUpload,
		generators,
		selectedGenerator,
		autogenerateAnnotations
	} from '$states';
	import Switch from '$ui/Switch.svelte';
	import UploadAsset from '$ui/UploadAsset.svelte';

	// TODO: Get path from file explorer.
	let parent_directory_id = 1;

	function handleUpload() {
		if (imageUpload.value === undefined || selectedGenerator.value === undefined) {
			alert('Please provide an image and/or select an annotation generator.');
			return;
		}

		// Still need to do this check as the user may have
		// uploaded an annotation file earlier and then
		// switched to autogeneration.
		if (autogenerateAnnotations.value) {
			SendUploadAssets(parent_directory_id, imageUpload.value, undefined, selectedGenerator.value);
			imageUpload.value = undefined;
		} else if (annotationsUpload) {
			SendUploadAssets(
				parent_directory_id,
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
			<span class="grey-text"> ANNOTATION GENERATION </span>
			<div style="display: flex;">
				<div style="flex: 1; display: flex; gap: 5px; padding-top: 3px;">
					AUTOGENERATE
					<Switch bind:checked={autogenerateAnnotations.value} />
				</div>
				<select style="flex: 1;" bind:value={selectedGenerator.value}>
					{#if generators.value}
						{#each generators.value as generator}
							<option value={generator}>{generator}</option>
						{/each}
					{/if}
				</select>
			</div>
		</div>

		<div style="display: flex;">
			<UploadAsset bind:assetUpload={imageUpload.value} placeholder="Image" />

			{#if !autogenerateAnnotations.value}
				<div class="divider"></div>
				<UploadAsset bind:assetUpload={annotationsUpload.value} placeholder="Annotations" />
			{/if}
		</div>

		<div
			class="inner-container grey-text"
			style="padding: 5px 10px; font-size: 12px; border-radius: 0 0 10px 10px;"
		>
			Drag file onto/click on icon to browse fs.
		</div>
	</div>

	<div style="display: flex;">
		<div style="flex: 1;"></div>
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
</style> -->
