<script lang="ts">
	import { GetMetadata, GetImageSelection, SendUploadAssets, GetAnnotations } from '$lib/api';
	import {
		image_upload,
		annotations_upload,
		image_list,
		annotation_generator_list
	} from '$lib/stores';
	import Switch from '$lib/components/control/Switch.svelte';

	// TODO: Cache settings choices locally.
	let selected_annotation_generator = $annotation_generator_list?.[0];
	let autogen_annotations = true;

	function HandleImage(event: DragEvent) {
		event.preventDefault();

		let file = event.dataTransfer?.files[0];
		$image_upload = file;

		let imageInput = document.getElementById('image-input');
		imageInput?.setAttribute('readonly', 'true');
	}

	function HandleAnnotationFile(event: DragEvent) {
		event.preventDefault();

		let file = event.dataTransfer?.files[0];
		$annotations_upload = file;

		let annotationInput = document.getElementById('annotation-input');
		annotationInput?.setAttribute('readonly', 'true');
	}

	function HandleUpload() {
		if ($image_upload && selected_annotation_generator) {
			// Still need to do this check as the user may have
			// uploaded an annotation file earlier and then
			// switched to autogeneration.
			if (autogen_annotations) {
				SendUploadAssets($image_upload, undefined, selected_annotation_generator);
			} else if (annotations_upload) {
				SendUploadAssets($image_upload, $annotations_upload, selected_annotation_generator);
			}
		}
		// TODO: Output error message for other cases.
	}
</script>

<div style="display: flex; flex-direction: column; gap: 15px;">
	<div class="outer-container">
		<div class="inner-container">
			<span class="grey-heading"> FILESYSTEM </span>
			{#if $image_list}
				{#if $image_list.length === 0}
					Upload an image to get started.
				{:else}
					<div style="padding: 0 10px;">
						{#each $image_list as image_name}
							<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
							<!-- svelte-ignore a11y-click-events-have-key-events -->
							<p
								on:click={() => {
									GetMetadata(image_name);
									GetImageSelection({
										image_name: image_name,
										level: 0,
										start: { x: 0, y: 0 },
										end: { x: 2, y: 2 }
									});
								}}
							>
								{image_name}
							</p>
						{/each}
					</div>
				{/if}
			{/if}
		</div>
	</div>

	<div>
		<div class="outer-container">
			<div class="inner-container" style="border-radius: 10px 10px 0 0;">
				<span class="grey-heading"> ANNOTATIONS </span>
				<div style="display: flex;">
					<div style="flex: 1; display: flex; gap: 5px; padding-top: 3px;">
						AUTOGENERATE
						<Switch bind:checked={autogen_annotations} />
					</div>
					{#if $annotation_generator_list}
						<select style="flex: 1;" bind:value={selected_annotation_generator}>
							{#each $annotation_generator_list as annotation_generator}
								<option value={annotation_generator}>{annotation_generator}</option>
							{/each}
						</select>
					{/if}
				</div>
			</div>

			<!-- svelte-ignore a11y-no-static-element-interactions -->
			<div class="input" style="display: flex;" on:drop={(e) => HandleImage(e)}>
				<input
					id="image-input"
					type="text"
					placeholder="Drop image here or enter a URL."
					value={$image_upload?.name || ''}
					style={autogen_annotations ? 'border-radius: 0 0 10px 10px' : ''}
					on:dragover={(e) => e.preventDefault()}
				/>

				<label style={autogen_annotations ? 'border-radius: 0 0 10px 0' : ''}>
					<div class="divider" />
					<img src="/folder.svg" alt="Browse filesystem." />
					<input type="file" accept="image/*" />
				</label>
			</div>

			{#if !autogen_annotations}
				<!-- svelte-ignore a11y-no-static-element-interactions -->
				<div class="input" style="display: flex;" on:drop={(e) => HandleAnnotationFile(e)}>
					<input
						id="annotation-input"
						type="text"
						placeholder="Drop annotation file here or enter a URL."
						value={$annotations_upload?.name || ''}
						style="border-radius: 0 0 10px 10px;"
						on:dragover={(e) => e.preventDefault()}
					/>
					<label style="border-radius: 0 0 10px 0;">
						<div class="divider" />
						<img src="/icons8-folder.svg" alt="Browse filesystem." />
						<input type="file" accept="image/*" />
					</label>
				</div>
			{/if}
		</div>

		<div style="display: flex;">
			<div style="flex: 1;" />
			<button type="submit" on:click={() => HandleUpload()}>UPLOAD</button>
		</div>
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

	.grey-heading {
		color: rgba(255, 255, 255, 0.6);
	}

	.outer-container {
		backdrop-filter: blur(15px);
		background: rgba(255, 255, 255, 0.15);
		box-shadow: 0 15px 15px rgba(0, 0, 0, 0.1);
		font-size: 14px;
		border-radius: 10px;
		font-family: 'JetBrains Mono', monospace;
	}

	.inner-container {
		background-color: rgba(0, 0, 0, 0.2);
		padding: 10px;
		border-radius: 10px;
		display: flex;
		flex-direction: column;
		gap: 10px;
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

		&:hover {
			background-color: rgba(0, 0, 0, 0.2);
		}

		&:focus {
			outline: none;
			background-color: rgba(0, 0, 0, 0.2);
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

	p {
		margin: 0;
		cursor: pointer;

		&:hover {
			text-decoration: underline;
		}
	}
</style>
