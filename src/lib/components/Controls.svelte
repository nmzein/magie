<script lang="ts">
	import { sendProcessRequest, sendListRequest } from '$lib/api';
	import { onDestroy, onMount } from 'svelte';
	import { writable } from 'svelte/store';

	let namesOfImages: string[] = [];
	let imageUploadQueue: string[] = [];

	const imageUploadQueueStore = writable<string[]>([]);
	const unsubscribeImageUploadQueueStore = imageUploadQueueStore.subscribe((values) => {
		imageUploadQueue = values;
	});

	onMount(async () => {
		try {
			namesOfImages = await sendListRequest();
			console.log(namesOfImages);
		} catch (error) {
			console.error('Error fetching list of images: ', error);
		}
	});

	let showLargePanel: boolean = false;

	function openLargePanel() {
		showLargePanel = !showLargePanel;
		const arrowIcon = document.getElementById('arrow-icon');
		if (arrowIcon) {
			if (showLargePanel) {
				arrowIcon.style.transform = 'translateY(1px) rotate(0deg)';
			} else {
				arrowIcon.style.transform = 'translateY(1px) rotate(180deg)';
			}
		}
	}

	function showUploadDialog() {
		let dialog = document.getElementById('upload-dialog') as HTMLDialogElement;
		dialog?.showModal();
	}

	function handleImage(event: DragEvent) {
		event.preventDefault();

		let file = event.dataTransfer?.files[0];
		let name = file?.name || '';

		// console.log(name);
		// imageUploadQueueStore.update((queue) => [...queue, name]);

		let imageInput = document.getElementById('image-input');
		imageInput?.setAttribute('value', name);
		imageInput?.setAttribute('readonly', 'true');
	}

	function handleAnnotation(event: DragEvent) {
		event.preventDefault();

		let file = event.dataTransfer?.files[0];
		let name = file?.name || '';

		// console.log(name);
		// imageUploadQueueStore.update((queue) => [...queue, name]);

		let annotationInput = document.getElementById('annotation-input');
		annotationInput?.setAttribute('value', name);
		annotationInput?.setAttribute('readonly', 'true');
	}

	onDestroy(() => {
		unsubscribeImageUploadQueueStore();
	});
</script>

<nav>
	<div id="container">
		{#if showLargePanel}
			<div class="panel large">
				<div id="target">
					<!-- Dropdown to select from available images. -->
					<select class="first">
						{#each namesOfImages as name}
							<option>{name}</option>
						{/each}
					</select>

					<div class="divider" />
					<div class="arrow"><img src="/icons8-chevron-26.png" alt="" /></div>

					<!-- <button class="last" id="upload-image" on:click={() => sendProcessRequest()} -->
					<button class="last" id="upload-image" on:click={() => showUploadDialog()}>
						<p>+</p></button
					>

					<dialog id="upload-dialog" class="panel">
						<form method="dialog" class="flex">
							<h1>Upload</h1>
							<div>
								<!-- svelte-ignore a11y-no-static-element-interactions -->
								<div
									class="flex input"
									style="border-radius: 8px 8px 0 0;"
									on:drop={(e) => handleImage(e)}
								>
									<input
										id="image-input"
										type="text"
										placeholder="Drop image here or enter a URL."
										on:dragover={(e) => e.preventDefault()}
									/>
									<div class="browse-cont">
										<label class="browse" style="border-radius: 0 8px 0 0;">
											Browse
											<input type="file" accept="image/*" />
										</label>
									</div>
								</div>
								<!-- svelte-ignore a11y-no-static-element-interactions -->
								<div
									class="flex input"
									style="border-radius: 0 0 8px 8px;"
									on:drop={(e) => handleAnnotation(e)}
								>
									<input
										id="annotation-input"
										type="text"
										placeholder="Drop annotation file here or enter a URL."
										style="border-radius: 0 0 0 8px;"
										on:dragover={(e) => e.preventDefault()}
									/>
									<div class="browse-cont">
										<label class="browse" style="border-radius: 0 0 8px 0;">
											Browse
											<input type="file" accept="image/*" />
										</label>
									</div>
								</div>
							</div>
							{#each imageUploadQueue as imageName}
								<p>{imageName}</p>
							{/each}
							<button type="submit" id="upload-dialog-submit">Submit</button>
						</form>
					</dialog>
				</div>
			</div>
		{:else}
			<div class="hidden large" />
		{/if}

		<div class="panel small">
			<div class="placeholder" />
			<button id="show-panel" on:click={() => openLargePanel()}
				><img id="arrow-icon" src="/icons8-chevron-26.png" alt="Show large panel." /></button
			>
		</div>
	</div>
</nav>

<!-- // #drop-image {
	// 	display: flex;
	// 	flex-direction: column;
	// 	gap: 10px;
	// 	align-items: center;
	// 	justify-content: center;
	// 	padding: 30px 0;
	// 	// margin-bottom: 20px;

	// 	height: 100px;
	// 	width: 98.5%;
	// 	border: 2px dashed rgba(255, 255, 255, 0.3);
	// 	border-radius: 10px;
	// }

	// #dialog-contents {
		// display: flex;
	// } -->

<!-- <div id="url-image"> -->
<!-- <input type="text" placeholder="Link to image." class="first" /> -->
<!-- <button class="last"><p>+</p></button> -->
<!-- </div> -->
<!-- <span> --- or --- </span> -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<!-- <div id="drop-image" on:drop={(e) => handleImage(e)}> -->
<!-- <span> Drag and drop image(s) here. </span> -->
<!-- <span> --- or --- </span> -->
<!-- <label id="browse"> -->
<!-- Browse -->
<!-- <input type="file" accept="image/*" /> -->
<!-- </label> -->
<!-- </div> -->

<!-- // position: absolute;
// right: 0;
// bottom: 0;
// margin-right: 15px;
// margin-bottom: 15px; -->

<!-- 
// #url-image {
	// 	width: 100%;
	// 	display: flex;
	// 	align-items: center;
	// 	justify-content: center;
	// }
-->

<style lang="scss">
	.flex {
		display: flex;
	}

	nav {
		display: block;
		position: absolute;

		top: 0;
		bottom: 0;
		right: 0;

		width: 350px;
		margin: 10px;

		pointer-events: none;
	}

	button {
		border-radius: 30px;
		height: 40px;
		padding: 0 10px;
		backdrop-filter: blur(15px);
		background: rgba(255, 255, 255, 0.15);
		box-shadow: 0 15px 15px rgba(0, 0, 0, 0.1);

		&:hover {
			background-color: rgba(255, 255, 255, 0.1);
		}

		& > p {
			display: inline-block;
			transform: translateY(-18.75px);
		}
	}

	dialog {
		min-width: 400px;
		min-height: 200px;
		padding: 20px;
	}

	form {
		display: flex;
		flex-direction: column;
		gap: 20px;
		text-align: right;
	}

	h1 {
		text-align: left;
	}

	input[type='file'] {
		display: none;
	}

	input[type='text'] {
		width: 100%;
		height: 40px;
		padding: 0 10px;

		color: white;
		border: none;
		font-size: 15px;
		// backdrop-filter: blur(15px);
		// background: rgba(255, 255, 255, 0.15);
		// box-shadow: 0 15px 15px rgba(0, 0, 0, 0.1);
		background: transparent;

		&:focus {
			outline: none;
		}
	}

	.input {
		width: 100%;
		height: 40px;

		backdrop-filter: blur(15px);
		background: rgba(255, 255, 255, 0.15);
		box-shadow: 0 15px 15px rgba(0, 0, 0, 0.1);
	}

	.browser-cont {
		display: flex;
	}

	.browse {
		border-radius: 15px !important;
		padding: 4px 10px;
		// margin: 10px 0;
		// width: 90px !important;
		// height: 30px !important;
		// backdrop-filter: blur(15px);
		background: rgba(20, 20, 20, 1);
		// box-shadow: 0 15px 15px rgba(0, 0, 0, 0.1);
		border: none;
		cursor: pointer;

		color: white;
		font-weight: 500;
		font-size: 14px;

		&:hover {
			background-color: rgba(10, 10, 10, 1);
		}
	}

	#upload-dialog-submit {
		width: 100px;
		align-self: flex-end;
	}

	.divider {
		position: absolute;
		height: 30px;
		top: 13px;
		left: 84.7%;
		z-index: 1;
		border-left: 2px solid rgba(255, 255, 255, 0.1);
		pointer-events: none;
	}

	.arrow {
		position: absolute;
		top: 18px;
		left: 78.3%;
		z-index: 10;
		pointer-events: none;

		& > img {
			width: 15px;
		}
	}

	#show-panel > img {
		width: 20px;
		height: 20px;
		transform: translateY(1px) rotate(180deg);
	}

	#container {
		display: flex;
		flex-direction: column;

		height: 100%;
		gap: 10px;
	}

	#target {
		display: flex;
		width: 100%;
		height: 40px;
	}

	#target > select {
		flex: 11;
		padding: 0 10px;

		backdrop-filter: blur(15px);
		background: rgba(255, 255, 255, 0.15);
		box-shadow: 0 15px 15px rgba(0, 0, 0, 0.1);

		&:hover {
			background-color: rgba(255, 255, 255, 0.1);
		}

		// -moz-appearance: none; /* Firefox */
		-webkit-appearance: none; /* Safari and Chrome */
		appearance: none;
	}

	#target > button {
		font-size: 24px;
		flex: 1;
	}

	.panel {
		color: white;
		background-color: rgba(0, 0, 0, 0.75);
		border: 1px solid rgba(255, 255, 255, 0.125);
		border-radius: 10px;
		backdrop-filter: blur(16px) saturate(180%);
		-webkit-backdrop-filter: blur(16px) saturate(180%);
		pointer-events: all;
	}

	.hidden {
		visibility: hidden;
	}

	.large {
		flex: 96;
		padding: 8px;
	}

	.small {
		flex: 4;
		min-height: 40px;
		padding: 5px;
		display: flex;

		button {
			height: 100%;
		}
	}

	.placeholder {
		flex: 1;
	}

	.first {
		border-radius: 8px 0 0 8px;
	}

	.last {
		border-radius: 0 8px 8px 0;
	}
</style>
