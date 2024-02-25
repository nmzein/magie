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
	});

	function draw() {
		if (!context || !scaler) return;

		context.clearRect(0, 0, canvasWidth, canvasHeight);

		for (const annotation of layer.annotations) {
			context.beginPath();
			context.moveTo(annotation[0][0] * scaler, annotation[0][1] * scaler);
			for (let [x, y] of annotation.slice(1)) context.lineTo(x * scaler, y * scaler);
			context.closePath();

			context.fillStyle = layer.fill;
			context.strokeStyle = layer.stroke;
			context.fill();
			context.stroke();
		}
	}
</script>

<canvas
	bind:this={canvas}
	height={canvasHeight}
	id={'annotation-layer-' + layerIndex}
	style="display: {layer.visible ? 'block' : 'none'}; opacity: {layer.opacity};"
>
	{draw()}
</canvas>

<style lang="scss">
	canvas {
		position: absolute;
		z-index: 1;
		width: 100%;
	}
</style>
