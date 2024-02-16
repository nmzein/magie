<script lang="ts">
	import { GetMetadata, GetImageSelection, GetAnnotations } from '$lib/api';
	import { image_list, metadata, image, annotations } from '$lib/stores';
	import Folder from '$lib/components/control/Folder.svelte';

	let showFilesystem = true;

	async function testSelection(image_name: string) {
		$metadata = undefined;
		$image = [[]];
		$annotations = undefined;

		GetMetadata(image_name);
		GetImageSelection({
			image_name: image_name,
			level: 0,
			start: { x: 0, y: 0 },
			end: { x: 12, y: 12 }
		});
		GetImageSelection({
			image_name: image_name,
			level: 1,
			start: { x: 0, y: 0 },
			end: { x: 3, y: 3 }
		});
		// GetImageSelection({
		// 	image_name: image_name,
		// 	level: 0,
		// 	start: { x: 30, y: 30 },
		// 	end: { x: 40, y: 40 }
		// });
		// GetImageSelection({
		// 	image_name: image_name,
		// 	level: 0,
		// 	start: { x: 0, y: 0 },
		// 	end: { x: 2, y: 2 }
		// });
		GetAnnotations(image_name);
	}
</script>

<div class="outer-container">
	<div class="inner-container">
		<!-- <span class="grey-heading"> FILESYSTEM </span> -->
		<Folder name="LOCAL STORAGE" bind:showFiles={showFilesystem}>
			{#if $image_list}
				{#if $image_list.length === 0}
					Upload an image to get started.
				{:else}
					<!-- <div style="padding: 0 10px;"> -->
					{#each $image_list as image_name}
						<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
						<!-- svelte-ignore a11y-click-events-have-key-events -->
						<p on:click={async () => testSelection(image_name)}>
							{image_name}
						</p>
					{/each}
					<!-- </div> -->
				{/if}
			{/if}
		</Folder>
		<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
		{#if !showFilesystem}
			<!-- svelte-ignore a11y-click-events-have-key-events -->
			<p
				on:click={() => (showFilesystem = !showFilesystem)}
				style="padding-left: 10px; margin-top: -10px;"
			>
				Click to expand filesystem.
			</p>
		{/if}
	</div>
</div>

<style lang="scss">
	p {
		margin: 0;
		cursor: pointer;

		&:hover {
			text-decoration: underline;
		}
	}
</style>
