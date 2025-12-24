<script lang="ts">
	import type { Image2DView } from './types';

	let { view = $bindable() }: { view: Image2DView } = $props();

	// const FRAME_RATE_CAP = 60;
	// const ENABLE_FRAMERATE_CAP = true;

	// function withFrameCap<T extends (...args: any[]) => void>(
	// 	handler: T,
	// 	fps: number,
	// 	enabled: boolean = true
	// ): T {
	// 	if (!enabled) return handler as T;

	// 	const minInterval = 1000 / fps;
	// 	let lastTime = 0;

	// 	return ((...args: Parameters<T>) => {
	// 		const now = performance.now();
	// 		if (now - lastTime >= minInterval) {
	// 			lastTime = now;
	// 			handler(...args);
	// 		}
	// 	}) as T;
	// }
	//
</script>

<canvas
	id={view.viewer.canvasId}
	style="cursor: {view.viewer.isDragging ? 'grab' : 'crosshair'};"
	class="absolute h-full w-full"
></canvas>
