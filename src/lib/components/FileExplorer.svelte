<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import { ImageUploadStore, AnnotationFileUploadStore } from '$lib/stores';
	import { SendImage, SendFiles, GetImagesList, GetAnnotationGenerators } from '$lib/api';
	import Switch from './Switch.svelte';

	let Image: File | undefined;
	let AnnotationFile: File | undefined;
	let ImagesList: string[] = [];

	let AnnotationGenerators: string[] = [];
	let AutogenerateAnnotations = true;

	let SelectedAnnotationGenerator = '';

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
			ImagesList = list;
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

<div style="display: flex; flex-direction: column;">
	<div id="container">
		<div
			id="annotation-controls"
			style="display: flex;
				   flex-direction: column;
				   gap: 10px;"
		>
			<span style="color: rgba(255, 255, 255, 0.6);"> ANNOTATIONS </span>
			<div style="display: flex;">
				<div style="flex: 1; display: flex; flex-direction: row; gap: 5px; padding-top: 3px;">
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
				style="border-radius: 8px 8px 0 0;"
				on:dragover={(e) => e.preventDefault()}
			/>

			<label style="border-radius: 0 8px 0 0;">
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
					style="border-radius: 0 0 8px 8px;"
					on:dragover={(e) => e.preventDefault()}
				/>
				<label style="border-radius: 0 0 8px 0;">
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

				// Output error message
			}}>UPLOAD</button
		>
	</div>

	{#each ImagesList as Image}
		<p>{Image}</p>
	{/each}
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

	#annotation-controls {
		background-color: rgba(0, 0, 0, 0.2);
		font-size: 14px;
		font-family: 'JetBrains Mono', monospace;
		padding: 10px;
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

	#container {
		backdrop-filter: blur(15px);
		background: rgba(255, 255, 255, 0.15);
		box-shadow: 0 15px 15px rgba(0, 0, 0, 0.1);
		border-radius: 10px;
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
		width: 94%;
		height: 40px;
		padding: 0 10px;
		border-radius: 10px;

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
		width: 100px;
	}

	img {
		width: 20px;
		height: 20px;
		margin: 10px;
	}
</style>
