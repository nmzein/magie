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
	<button class="last" id="upload-image" on:click={() => showUploadDialog()}> <p>+</p></button>

	<dialog id="upload-dialog" class="panel">
		<form method="dialog" class="flex">
			<h1>Upload</h1>
			<div>
				<!-- svelte-ignore a11y-no-static-element-interactions -->
				<div class="flex input" style="border-radius: 8px 8px 0 0;" on:drop={(e) => handleImage(e)}>
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
