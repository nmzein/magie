<script lang="ts">
	import { browser } from '$app/environment';
	import { Stage, Layer, Shape } from 'svelte-konva';
	import type { Point } from '$lib/types';

	export let coordinates: Point[];
	export let colours = { fill: '#e07470', stroke: '#a12c28' };
	export let scale = 1;

	import { MetadataStore } from '$lib/stores';
	import type { ImageMetadata } from '$lib/types';
	import { onMount, onDestroy } from 'svelte';

	let metadata: ImageMetadata | undefined;

	const UnsubscribeMetadataStore = MetadataStore.subscribe((values) => {
		metadata = values;
	});

	let XScaler: number;
	let YScaler: number;
	let ImageGridWidth: number;
	let ImageGridHeight: number;

	onMount(() => {
		window.addEventListener('resize', ResizeAnnotations);

		ResizeAnnotations();

		return () => {
			window.removeEventListener('resize', ResizeAnnotations);
		};
	});

	// coordinate * (#image-grid Width (var) / Actual Image Width)
	const ResizeAnnotations = () => {
		const ImageGridDimensions = document.getElementById('image-grid')?.getClientRects();
		ImageGridWidth = ImageGridDimensions?.[0].width || 0;
		ImageGridHeight = ImageGridDimensions?.[0].height || 0;

		if (ImageGridWidth && ImageGridHeight && metadata?.cols && metadata?.rows) {
			XScaler = ImageGridWidth / (metadata.cols * 1024 * scale);
			YScaler = ImageGridHeight / (metadata.rows * 1024 * scale);
		}
	};

	onDestroy(() => {
		UnsubscribeMetadataStore();
	});
</script>

{#if browser && document.getElementById('image-grid')}
	<div>
		<!-- Move out of component -->
		<Stage
			config={{
				width: ImageGridWidth,
				height: ImageGridHeight
			}}
		>
			<Layer>
				<Shape
					config={{
						sceneFunc: function (context, shape) {
							context.beginPath();

							context.moveTo(coordinates[0].x * XScaler, coordinates[0].y * YScaler);
							for (let i = 1; i < coordinates.length; i++) {
								context.lineTo(coordinates[i].x * XScaler, coordinates[i].y * YScaler);
							}

							context.closePath();

							// special Konva.js method
							context.fillStrokeShape(shape);
						},
						fill: colours.fill,
						stroke: colours.stroke,
						strokeWidth: 1
					}}
				/>
			</Layer>
		</Stage>
	</div>
{/if}

<style>
	div {
		position: absolute;
	}
</style>
