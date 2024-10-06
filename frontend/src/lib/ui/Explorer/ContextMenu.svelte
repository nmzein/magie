<script lang="ts">
	import type { Bounds, Point } from '$types';
	import * as Dropdown from '$components/dropdown';

	let {
		show = $bindable(false),
		position,
		parentBounds
	}: { show: boolean; position: Point; parentBounds: Bounds } = $props();

	const classes = {
		list: `flex flex-col mt-[4px] ml-[4px] bg-tertiary/90 rounded-[5px] border border-primary/10 backdrop-blur-[45px] z-10 text-sm`,
		item: 'flex flex-row gap-[10px] items-center m-[2px] px-[10px] py-[7.5px] rounded-[5px] hover:bg-primary/10'
	};

	let clientHeight: number | undefined = $state();
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
	bind:clientHeight
	class="absolute z-[1000]"
	style={`transform: translate(${position.x - parentBounds.left}px, ${position.y - parentBounds.top + (clientHeight ?? 0) / 2}px)`}
	oncontextmenu={(e) => e.preventDefault()}
>
	<Dropdown.Root {classes} bind:show>
		<Dropdown.List>
			<Dropdown.Item>Select All</Dropdown.Item>
			<Dropdown.Item>Paste</Dropdown.Item>
		</Dropdown.List>
	</Dropdown.Root>
</div>
