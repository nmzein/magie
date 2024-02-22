<script lang="ts">
	let {
		content,
		showContent = false,
		depth
	} = $props<{ content: DirectoryNode; showContent: boolean; depth: number }>();

	import type { DirectoryNode } from '$lib/types';
	import Directory from '$control/Directory.svelte';
	import File from './File.svelte';

	let indent = (depth * 10).toString() + 'px';
	let nextIndent = ((depth + 1) * 10).toString() + 'px';

	let src = showContent ? '/default_folder_opened.svg' : '/default_folder.svg';
	let alt = showContent ? 'Open Directory' : 'Directory';
</script>

<div>
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<!-- svelte-ignore a11y-no-static-element-interactions -->
	<div
		id="label"
		class="grey-heading"
		style="padding: 2px {indent}; {depth == 1 && showContent ? 'margin-bottom: 3px;' : ''}"
		onclick={() => (showContent = !showContent)}
	>
		{#if depth > 1}
			<img {src} {alt} />
		{/if}
		{content.name}
	</div>

	<div style="display: {showContent ? 'visible' : 'none'};">
		{#each content.files as file}
			<!-- <p onclick={async () => LoadImage(image)}> -->
			<File {file} indent={nextIndent} />
		{/each}
		{#each content.children as directory}
			<Directory content={directory} showContent={false} depth={depth + 1} />
		{/each}
	</div>
</div>

<style lang="scss">
	#label {
		cursor: pointer;
		user-select: none;
		display: flex;
		flex-direction: row;
		&:hover {
			background-color: rgba(255, 255, 255, 0.1);
		}
	}

	div {
		display: flex;
		flex-direction: column;
	}

	img {
		margin-right: 5px;
		width: 20px;
		height: 20px;
	}
</style>
