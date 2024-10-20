<script lang="ts">
	import { explorer, uploader } from '$states';
	import { defined } from '$helpers';
	import TopBar from './TopBar.svelte';
	import UploadAsset from './UploadAsset.svelte';
	import * as Pages from '$components/pages/index.ts';
	import * as Tabs from '$components/tabs/index.ts';
</script>

{#if explorer.showUploader}
	<div class="absolute z-10 m-10 h-[calc(100%-80px)] w-[calc(100%-80px)] rounded-xl">
		<div
			class="panel border-tertiary flex h-full w-full flex-col rounded-[inherit] border-2"
			role="dialog"
			aria-modal="true"
		>
			<TopBar />

			<div class="m-5 flex flex-1 flex-col items-center justify-center gap-5">
				<Pages.Root bind:currentPage={uploader.currentPage}>
					<Pages.Page nextDisabled={!defined(uploader.image)}>
						<UploadAsset bind:asset={uploader.image} placeholder="Image" />
					</Pages.Page>

					<Pages.Page
						nextDisabled={!uploader.annotationsSatisfied}
						done={async () => {
							explorer.showUploader = false;
							await uploader.upload();
						}}
					>
						<div class="flex h-full select-none flex-col justify-between gap-5">
							<Tabs.Root
								classes={{
									list: 'rounded-md bg-primary/10 w-fit p-1 self-center',
									trigger: {
										base: 'py-1 px-2 transition-colors rounded-[4px]',
										active: 'bg-primary/20'
									},
									content: 'h-full'
								}}
								bind:currentTab={uploader.options.annotations}
							>
								<Tabs.TriggerList>
									<Tabs.Trigger value="none">None</Tabs.Trigger>
									<Tabs.Trigger value="provide">Provide</Tabs.Trigger>
									<Tabs.Trigger value="generate" disabled>Generate</Tabs.Trigger>
								</Tabs.TriggerList>
								<Tabs.ContentSpace>
									<Tabs.Content value="none">
										<div
											class="text-secondary/80 bg-tertiary/50 flex h-full items-center justify-center rounded-md p-10 text-center text-sm"
										>
											Continue to upload without annotations, or, select a different option from the
											tabs above.
										</div>
									</Tabs.Content>
									<Tabs.Content value="provide">
										<UploadAsset bind:asset={uploader.annotations} placeholder="Annotations" />
									</Tabs.Content>
								</Tabs.ContentSpace>
							</Tabs.Root>
						</div>
					</Pages.Page>
				</Pages.Root>
			</div>
		</div>
	</div>
{/if}
