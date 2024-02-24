<script lang="ts">
	let { imageLayersDiv, layer, layerIndex, imageWidth, imageHeight } = $props<{
		imageLayersDiv: HTMLDivElement;
		layer: AnnotationLayer;
		layerIndex: number;
		imageWidth: number;
		imageHeight: number;
	}>();

	import type { AnnotationLayer } from '$types';

	let canvas: HTMLCanvasElement | undefined = $state();
	let context: CanvasRenderingContext2D | null | undefined = $derived(canvas?.getContext('2d'));
	let canvasWidth: number = $derived(imageLayersDiv.getBoundingClientRect()?.width);
	let scaler: number = $derived(canvasWidth / imageWidth);
	let canvasHeight: number = $derived(imageHeight * scaler);

	$effect(() => {
		if (!canvas) return;
		canvas.width = canvasWidth;
		// canvas.height = canvasHeight;
	});

	let id = 'annotation-layer-' + layerIndex;
	let fill = $derived(layer.colours.fill);
	let stroke = $derived(layer.colours.stroke);

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

<canvas bind:this={canvas} height={canvasHeight} {id}>
	{#each layer.annotations as annotation}
		{draw(annotation)}
	{/each}
</canvas>

<style lang="scss">
	canvas {
		position: absolute;
		z-index: 1;
		width: 100%;
	}
</style>
