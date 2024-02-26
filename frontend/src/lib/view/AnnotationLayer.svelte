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
	let annotations = $derived.by(() => {
		return layer.annotations.map((annotation) => {
			return annotation.map(([x, y]) => [x * scaler, y * scaler]);
		});
	});

	$effect(() => {
		if (!canvas) return;
		canvas.width = canvasWidth;
	});

	function draw() {
		if (!context || !scaler) return;

		context.clearRect(0, 0, canvasWidth, canvasHeight);
		context.fillStyle = layer.fill;
		context.strokeStyle = layer.stroke;

		for (const annotation of annotations) {
			context.beginPath();
			context.moveTo(annotation[0][0], annotation[0][1]);
			for (let i = 1; i < annotation.length; i++) {
				context.lineTo(annotation[i][0], annotation[i][1]);
			}
			context.closePath();

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
