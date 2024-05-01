<script lang="ts">
	let { file, indent }: { file: { name: string; metadata: Image }; indent: string } = $props();

	import type { Image } from '$lib/types';
	import { LoadImage } from '$api';
</script>

<button style="padding: 4px {indent};" onclick={async () => LoadImage(file.metadata)}>
	<img src="/default_file.svg" alt="File" />
	{file.name}
</button>

<style lang="scss">
	button {
		display: flex;
		cursor: pointer;

		&:hover {
			background-color: rgba(255, 255, 255, 0.1);
			text-decoration: underline;
		}
	}

	img {
		margin-left: 2px;
		margin-right: 5px;
		width: 18px;
		height: 18px;
	}
</style>
