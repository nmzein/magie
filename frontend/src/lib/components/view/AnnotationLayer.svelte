<script lang="ts">
	let { layer, layerIndex, imageWidth, imageHeight } = $props<{
		layer: AnnotationLayer;
		layerIndex: number;
		imageWidth: number;
		imageHeight: number;
	}>();

	import type { AnnotationLayer } from '$types';

	let context: CanvasRenderingContext2D | null | undefined = $state();
	let canvasWidth: number | undefined = $state();
	let canvasHeight: number | undefined = $state();
	let scaler: number | undefined = $state();

	let id = 'annotation-layer-' + layerIndex;
	let fill = layer.colours.fill;
	let stroke = layer.colours.stroke;

	$effect(() => {
		context = (document.getElementById(id) as HTMLCanvasElement)?.getContext('2d');

		canvasWidth = document.getElementById('image-layers')?.getBoundingClientRect()?.width;
		if (!canvasWidth) return;

		// Check logic later. Fails when resizing viewport.
		scaler = canvasWidth / imageWidth;
		canvasHeight = imageHeight * scaler;
	});

	function draw(annotation: number[][]) {
		if (!context || !scaler) return;

		context.beginPath();
		context.moveTo(annotation[0][0] * scaler, annotation[0][1] * scaler);
		for (let [x, y] of annotation.slice(1)) context.lineTo(x * scaler, y * scaler);
		context.closePath();

		context.fillStyle = fill;
		context.strokeStyle = stroke;
		context.fill();
		context.stroke();
	}
</script>

<canvas width={canvasWidth} height={canvasHeight} {id}>
	{#each layer.annotations as annotation}
		{draw(annotation)}
	{/each}
</canvas>

<style lang="scss">
	canvas {
		position: absolute;
		z-index: 1;
	}
</style>
