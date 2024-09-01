<script lang="ts">
	import { scale } from 'svelte/transition';
	import { explorer, uploader } from '$states';
	import { defined } from '$helpers';
	import TopBar from './TopBar.svelte';
	import UploadAsset from './UploadAsset.svelte';
	import * as Pages from '$components/pages/index.ts';
	import * as Tabs from '$components/tabs/index.ts';
</script>

{#if explorer.showUploader}
	<div
		class="absolute z-10 m-10 h-[calc(100%-80px)] w-[calc(100%-80px)] rounded-xl"
		transition:scale={{ duration: 200 }}
	>
		<div
			class="panel border-tertiary flex h-full w-full flex-col rounded-[inherit] border-2 shadow-[0_0_10px_rgba(0,0,0,0.2)]"
			role="dialog"
			aria-modal="true"
		>
			<TopBar />

			<div class="m-5 flex flex-1 flex-col items-center justify-center gap-5">
				<Pages.Root>
					<Pages.Page nextDisabled={!defined(uploader.image)}>
						<UploadAsset bind:asset={uploader.image} placeholder="Image" />
					</Pages.Page>

					<Pages.Page nextDisabled={!uploader.annotationsSatisfied}>
						<Tabs.Root classes={{ content: 'h-full' }}>
							<Tabs.List>
								<Tabs.Trigger
									value="none"
									sideEffect={() => (uploader.settings.annotations = 'none')}
								>
									None
								</Tabs.Trigger>
								<Tabs.Trigger
									value="provide"
									sideEffect={() => (uploader.settings.annotations = 'provide')}
								>
									Provide
								</Tabs.Trigger>
								<Tabs.Trigger
									value="generate"
									sideEffect={() => (uploader.settings.annotations = 'generate')}
								>
									Generate
								</Tabs.Trigger>
							</Tabs.List>
							<Tabs.Content value="provide">
								<UploadAsset bind:asset={uploader.annotations} placeholder="Annotations" />
							</Tabs.Content>
						</Tabs.Root>
					</Pages.Page>
				</Pages.Root>
			</div>
		</div>
	</div>
{/if}
