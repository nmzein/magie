<script lang="ts">
	import { explorer, uploader } from '$states';
	import { scale } from 'svelte/transition';
	import TopBar from './TopBar.svelte';
	import UploadAsset from './UploadAsset.svelte';
	import { defined } from '$helpers';
	import * as Pages from '$components/pages/index.ts';
</script>

{#if explorer.showUploader}
	<div id="outer" transition:scale={{ duration: 200 }}>
		<div id="inner" class="panel flex-column" role="dialog" aria-modal="true">
			<TopBar />

			<div id="main" class="flex-column">
				<Pages.Root>
					<Pages.Page nextDisabled={!defined(uploader.image)}>
						<UploadAsset bind:asset={uploader.image} placeholder="Image" />
					</Pages.Page>

					<Pages.Page nextDisabled={!defined(uploader.annotations)}>
						<UploadAsset bind:asset={uploader.annotations} placeholder="Annotations" />
					</Pages.Page>
				</Pages.Root>
			</div>
		</div>
	</div>
{/if}

<style lang="scss">
	#outer {
		position: absolute;
		z-index: 5;
		width: calc(100% - 80px);
		height: calc(100% - 80px);
		margin: 40px;

		border-radius: var(--border-radius);
	}

	#inner {
		z-index: 6;
		box-shadow: 0 0 10px 0 rgba(0, 0, 0, 0.2);
		width: 100%;
		height: 100%;
		border-radius: inherit;
		border: 2px solid #3e3e3e;
	}

	#main {
		flex: 1;
		justify-content: center;
		align-items: center;
		margin: 20px;
		gap: 20px;
	}
</style>
