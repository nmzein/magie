<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import { ImageUploadStore, AnnotationFileUploadStore } from '$lib/stores';
	import {
		SendImage,
		SendFiles,
		GetImagesList,
		GetAnnotationGenerators,
		GetMetadata,
		GetImageSelection
	} from '$lib/api';
	import Switch from './Switch.svelte';
	import type { ImageSelection } from '$lib/types';

	let Image: File | undefined;
	let AnnotationFile: File | undefined;
	let imageNames: string[] = [];

	let AnnotationGenerators: string[] = [];
	let AutogenerateAnnotations = true;

	let SelectedAnnotationGenerator = '';
	let selection: ImageSelection = { start: { x: 0, y: 0 }, end: { x: 2, y: 2 } };

	const UnsubscribeImageUploadStore = ImageUploadStore.subscribe((value) => {
		Image = value;
	});

	const UnsubscribeAnnotationFileUploadStore = AnnotationFileUploadStore.subscribe((value) => {
		AnnotationFile = value;
	});

	function HandleImage(event: DragEvent) {
		event.preventDefault();

		let file = event.dataTransfer?.files[0];
		ImageUploadStore.set(file);

		let imageInput = document.getElementById('image-input');
		imageInput?.setAttribute('readonly', 'true');
	}

	function HandleAnnotationFile(event: DragEvent) {
		event.preventDefault();

		let file = event.dataTransfer?.files[0];
		AnnotationFileUploadStore.set(file);

		let annotationInput = document.getElementById('annotation-input');
		annotationInput?.setAttribute('readonly', 'true');
	}

	onMount(() => {
		GetImagesList().then((list) => {
			imageNames = list;
		});

		GetAnnotationGenerators().then((generators) => {
			AnnotationGenerators = generators;
			SelectedAnnotationGenerator = generators[0];
		});
	});

	onDestroy(() => {
		UnsubscribeImageUploadStore();
		UnsubscribeAnnotationFileUploadStore();
	});
</script>

<div style="display: flex; flex-direction: column; gap: 20px;">
	<div>
		<div class="outer-container">
			<div class="inner-container" style="border-radius: 10px 10px 0 0;">
				<span class="grey-heading"> ANNOTATIONS </span>
				<div style="display: flex;">
					<div style="flex: 1; display: flex; gap: 5px; padding-top: 3px;">
						AUTOGENERATE
						<Switch bind:checked={AutogenerateAnnotations} />
					</div>
					<select style="flex: 1;" bind:value={SelectedAnnotationGenerator}>
						{#each AnnotationGenerators as AnnotationGenerator}
							<option value={AnnotationGenerator}>{AnnotationGenerator}</option>
						{/each}
					</select>
				</div>
			</div>

			<!-- svelte-ignore a11y-no-static-element-interactions -->
			<div class="input" style="display: flex;" on:drop={(e) => HandleImage(e)}>
				<input
					id="image-input"
					type="text"
					placeholder="Drop image here or enter a URL."
					value={Image?.name || ''}
					style={AutogenerateAnnotations ? 'border-radius: 0 0 10px 10px' : ''}
					on:dragover={(e) => e.preventDefault()}
				/>

				<label style={AutogenerateAnnotations ? 'border-radius: 0 0 10px 0' : ''}>
					<div class="divider" />
					<img src="/icons8-folder.svg" alt="Browse filesystem." />
					<input type="file" accept="image/*" />
				</label>
			</div>

			{#if !AutogenerateAnnotations}
				<!-- svelte-ignore a11y-no-static-element-interactions -->
				<div class="input" style="display: flex;" on:drop={(e) => HandleAnnotationFile(e)}>
					<input
						id="annotation-input"
						type="text"
						placeholder="Drop annotation file here or enter a URL."
						value={AnnotationFile?.name || ''}
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
			<button
				type="submit"
				on:click={() => {
					if (Image) {
						if (AutogenerateAnnotations) {
							SendImage(Image, SelectedAnnotationGenerator);
						} else if (AnnotationFile) {
							SendFiles(Image, AnnotationFile, SelectedAnnotationGenerator);
						}
					}
					// TODO: Output error message for other cases.
				}}>UPLOAD</button
			>
		</div>
	</div>

	<div class="outer-container">
		<div class="inner-container">
			<span class="grey-heading"> FILESYSTEM </span>
			<div style="padding: 0 10px;">
				{#each imageNames as imageName}
					<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
					<!-- svelte-ignore a11y-click-events-have-key-events -->
					<p
						on:click={() => {
							GetMetadata(imageName);
							GetImageSelection(imageName, selection);
						}}
					>
						{imageName}
					</p>
				{/each}
			</div>
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
