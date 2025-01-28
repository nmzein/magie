<script lang="ts">
	import { explorer, repository } from '$states';
	import TopBar from './TopBar.svelte';
	import UploadAsset from './UploadAsset.svelte';
	import * as Pages from '$components/pages/index.ts';
	import * as Tabs from '$components/tabs/index.ts';

	$effect(() => {
		if (explorer!.uploader.image) {
			explorer!.uploader.options.name = explorer!.uploader.image.name;
		}
	});
</script>

{#if explorer!.uploader.show}
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div
		class="absolute z-10 h-full w-full min-w-72 rounded-xl bg-black/50 p-6"
		onkeydown={(e) => e.stopPropagation()}
	>
		<div
			class="panel border-tertiary flex h-full w-full flex-col rounded-[inherit] border-2"
			role="dialog"
			aria-modal="true"
		>
			<TopBar />

			<div class="flex flex-1 flex-col items-center justify-center gap-5 p-5">
				<Pages.Root bind:currentPage={explorer!.uploader.currentPage}>
					<Pages.Page nextDisabled={!explorer!.uploader.imageSatisfied}>
						<div class="flex h-full gap-5">
							<div class="flex-3 flex flex-col gap-1">
								<span class="text-secondary text-sm">IMAGE</span>
								<UploadAsset
									bind:asset={explorer!.uploader.image}
									placeholder="Click to browse filesystem or drag a file here."
								/>
							</div>
							<div class="flex flex-1 flex-col gap-4 text-sm">
								<div class="flex flex-col gap-1">
									<span class="text-secondary">NAME</span>
									<input
										type="text"
										bind:value={explorer!.uploader.options.name}
										class="outline-tertiary hover:outline-secondary w-full rounded-md p-2 outline transition-all"
									/>
								</div>
								<div class="flex flex-col gap-1">
									<span class="text-secondary">DECODING FORMAT</span>
									<select
										bind:value={explorer!.uploader.options.decoder}
										class="outline-tertiary hover:outline-secondary w-full rounded-md p-2 outline transition-all"
									>
										{#each repository.decoders as decoder}
											<option value={decoder}>{decoder}</option>
										{/each}
									</select>
								</div>
								<div class="flex flex-col gap-1">
									<span class="text-secondary">ENCODING FORMAT</span>
									<select
										bind:value={explorer!.uploader.options.encoder}
										class="outline-tertiary hover:outline-secondary w-full rounded-md p-2 outline transition-all"
									>
										{#each repository.encoders as encoder}
											<option value={encoder}>{encoder}</option>
										{/each}
									</select>
								</div>
							</div>
						</div>
					</Pages.Page>

					<Pages.Page
						nextDisabled={!explorer!.uploader.annotationsSatisfied}
						done={async () => await explorer!.uploader.upload(explorer!.directory.data.id)}
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
								bind:currentTab={explorer!.uploader.options.annotations}
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
										<div class="flex h-full gap-5">
											<div class="flex-3 flex flex-col gap-1">
												<span class="text-secondary text-sm">ANNOTATIONS</span>
												<UploadAsset
													bind:asset={explorer!.uploader.annotations}
													placeholder="Click to browse filesystem or drag a file here."
												/>
											</div>
											<div class="flex flex-1 flex-col gap-4 text-sm">
												<div class="flex flex-col gap-1">
													<span class="text-secondary">GENERATOR</span>
													<select
														bind:value={explorer!.uploader.options.generator}
														class="outline-tertiary hover:outline-secondary w-full rounded-md p-2 outline transition-all"
													>
														{#each repository.generators as generator}
															<option value={generator}>{generator}</option>
														{/each}
													</select>
												</div>
											</div>
										</div>
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
