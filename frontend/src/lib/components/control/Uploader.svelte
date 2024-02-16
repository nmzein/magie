<script lang="ts">
	import { SendUploadAssets } from '$lib/api';
	import { image_upload, annotations_upload, annotation_generator_list } from '$lib/stores';
	import Switch from '$lib/components/control/Switch.svelte';
	import UploadAsset from '$lib/components/view/UploadAsset.svelte';

	// TODO: Cache settings choices locally.
	let selectedAnnotationGenerator = $annotation_generator_list?.[0];
	let autogenerateAnnotations = true;

	function handleImage(event: DragEvent) {
		event.preventDefault();

		let file = event.dataTransfer?.files[0];
		$image_upload = file;

		let imageInput = document.getElementById('image-input');
		// imageInput?.setAttribute('readonly', 'true');
		imageInput?.setAttribute('placeholder', ' ');
	}

	function handleAnnotationFile(event: DragEvent) {
		event.preventDefault();

		let file = event.dataTransfer?.files[0];
		$annotations_upload = file;

		let annotationInput = document.getElementById('annotation-input');
		// annotationInput?.setAttribute('readonly', 'true');
		annotationInput?.setAttribute('placeholder', ' ');
	}

	function handleUpload() {
		if ($image_upload && selectedAnnotationGenerator) {
			// Still need to do this check as the user may have
			// uploaded an annotation file earlier and then
			// switched to autogeneration.
			if (autogenerateAnnotations) {
				SendUploadAssets($image_upload, undefined, selectedAnnotationGenerator);
				resetImageField();
			} else if (annotations_upload) {
				console.log('got here');
				SendUploadAssets($image_upload, $annotations_upload, selectedAnnotationGenerator);
				resetImageField();
				resetAnnotationField();
			}
		}
		// TODO: Output error message for other cases.
	}

	function resetImageField() {
		$image_upload = undefined;
		(document.getElementById('image-input') as HTMLInputElement).placeholder =
			'Drop image here or browse your filesystem.';
	}

	function resetAnnotationField() {
		$annotations_upload = undefined;
		(document.getElementById('annotation-input') as HTMLInputElement).placeholder =
			'Drop annotation file here or browse your fs.';
	}
</script>

<div>
	<div class="outer-container">
		<div class="inner-container" style="border-radius: 10px 10px 0 0;">
			<span class="grey-heading"> ANNOTATIONS </span>
			<div style="display: flex;">
				<div style="flex: 1; display: flex; gap: 5px; padding-top: 3px;">
					AUTOGENERATE
					<Switch bind:checked={autogenerateAnnotations} />
				</div>
				{#if $annotation_generator_list}
					<select style="flex: 1;" bind:value={selectedAnnotationGenerator}>
						{#each $annotation_generator_list as annotation_generator}
							<option value={annotation_generator}>{annotation_generator}</option>
						{/each}
					</select>
				{/if}
			</div>
		</div>

		<!-- svelte-ignore a11y-no-static-element-interactions -->
		<div class="input" style="display: flex;" on:drop={(e) => handleImage(e)}>
			<input
				id="image-input"
				type="text"
				placeholder="Drop image here or browse your filesystem."
				style={autogenerateAnnotations ? 'border-radius: 0 0 10px 10px' : ''}
				on:dragover={(e) => e.preventDefault()}
				readonly
			/>
			{#if $image_upload}
				<!-- svelte-ignore a11y-click-events-have-key-events -->
				<div on:click={() => resetImageField()} style="position: absolute; width: 100%;">
					<UploadAsset name={$image_upload?.name} />
				</div>
			{/if}

			<label style={autogenerateAnnotations ? 'border-radius: 0 0 10px 0' : ''}>
				<div class="divider" />
				<img src="/folder.svg" alt="Browse filesystem." />
				<input type="file" accept="image/*" />
			</label>
		</div>

		{#if !autogenerateAnnotations}
			<!-- svelte-ignore a11y-no-static-element-interactions -->
			<div class="input" style="display: flex;" on:drop={(e) => handleAnnotationFile(e)}>
				<input
					id="annotation-input"
					type="text"
					placeholder="Drop annotation file here or browse your fs."
					value={$annotations_upload?.name || ''}
					style="border-radius: 0 0 10px 10px;"
					on:dragover={(e) => e.preventDefault()}
					readonly
				/>

				{#if $annotations_upload}
					<!-- svelte-ignore a11y-click-events-have-key-events -->
					<div on:click={() => resetAnnotationField()} style="position: absolute; width: 100%;">
						<UploadAsset name={$annotations_upload?.name} />
					</div>
				{/if}

				<label style="border-radius: 0 0 10px 0;">
					<div class="divider" />
					<img src="/folder.svg" alt="Browse filesystem." />
					<input type="file" accept="image/*" />
				</label>
			</div>
		{/if}
	</div>

	<div style="display: flex;">
		<div style="flex: 1;" />
		<button type="submit" on:click={() => handleUpload()}>UPLOAD</button>
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

	.divider {
		position: absolute;
		height: 30px;
		top: 10%;
		left: -2%;
		z-index: 1;
		border-left: 2px solid rgba(255, 255, 255, 0.2);
		pointer-events: none;
	}

	input[type='text'] {
		flex: 1;
		height: 40px;
		padding: 0 10px;

		color: white;
		border: none;
		font-size: 13px;
		background: transparent;

		// &:hover {
		// 	background-color: rgba(0, 0, 0, 0.2);
		// }

		&:focus {
			outline: none;
			// background-color: rgba(0, 0, 0, 0.2);
		}
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
