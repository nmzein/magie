<script lang="ts">
	import { LoadImage } from '$lib/api';
	import { image_list } from '$lib/stores';
	import Folder from '$lib/components/control/Folder.svelte';

	let showFilesystem = true;
</script>

<div class="outer-container">
	<div class="inner-container">
		<Folder name="LOCAL STORAGE" bind:showFiles={showFilesystem}>
			{#if $image_list}
				{#if $image_list.length === 0}
					Upload an image to get started.
				{:else}
					{#each $image_list as imageName}
						<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
						<!-- svelte-ignore a11y-click-events-have-key-events -->
						<p on:click={async () => LoadImage(imageName)}>
							{imageName}
						</p>
					{/each}
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
