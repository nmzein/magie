<script lang="ts">
	import { WEBSOCKET_URL } from '$constants';
	import type { Image2DView } from './types';
	import { onMount } from 'svelte';

	let { view = $bindable() }: { view: Image2DView } = $props();

	let canvas: HTMLCanvasElement | undefined = $state();
	let worker: Worker | undefined = $state();
	let sharedInts: Int32Array | undefined;

	const FRAME_RATE_CAP = 60;
	const ENABLE_FRAMERATE_CAP = true;

	function withFrameCap<T extends (...args: any[]) => void>(
		handler: T,
		fps: number,
		enabled: boolean = true
	): T {
		if (!enabled) return handler as T;

		const minInterval = 1000 / fps;
		let lastTime = 0;

		return ((...args: Parameters<T>) => {
			const now = performance.now();
			if (now - lastTime >= minInterval) {
				lastTime = now;
				handler(...args);
			}
		}) as T;
	}

	onMount(() => {
		if (!canvas || !window.innerWidth || !window.innerHeight || !view.state) return;

		console.log('Window device pixel ratio is:', window.devicePixelRatio);

		const offscreen = canvas.transferControlToOffscreen();
		const sharedBuf = new SharedArrayBuffer(Int32Array.BYTES_PER_ELEMENT * 6);
		sharedInts = new Int32Array(sharedBuf);
		worker = new Worker(new URL('./worker.ts', import.meta.url), { type: 'module' });

		worker.postMessage(
			{
				type: 'init',
				data: {
					canvas: offscreen,
					width: window.innerWidth * window.devicePixelRatio,
					height: window.innerHeight * window.devicePixelRatio,
					wsUrl: WEBSOCKET_URL,
					storeId: view.state.storeId,
					id: view.state.id,
					layers: JSON.stringify(view.state.layers),
					sharedBuf
				}
			},
			[offscreen]
		);

		worker.onmessage = (e) => {
			const { type } = e.data;
			switch (type) {
				case 'connected':
					markDirty();
					break;
			}
		};

		return () => {
			worker?.postMessage({ type: 'disconnect' });
			worker?.terminate();
		};
	});

	function markDirty() {
		if (!sharedInts) return;
		const { scale, offsetX, offsetY } = view.state.transformer;

		// [0] canvas width
		sharedInts[0] = Math.round(window.innerWidth * window.devicePixelRatio);
		// [1] canvas height
		sharedInts[1] = Math.round(window.innerHeight * window.devicePixelRatio);
		// [2] offset x
		sharedInts[2] = Math.round(offsetX);
		// [3] offset y
		sharedInts[3] = Math.round(offsetY);
		// [4] scale * 1e6
		sharedInts[4] = Math.floor(scale * 1e6);
		// [5] dirty flag
		Atomics.store(sharedInts, 5, 1);
	}

	function onmousedown(e: MouseEvent) {
		e.preventDefault();
		view.state.transformer.panStart(e.clientX, e.clientY);
	}

	function ontouchstart(te: TouchEvent) {
		const e = te.touches[0];
		view.state.transformer.panStart(e.clientX, e.clientY);
	}

	function onmousemove(e: MouseEvent) {
		if (!view.state.transformer.isDragging) return;
		e.preventDefault();
		view.state.transformer.pan(e.clientX, e.clientY);
		markDirty();
	}

	function ontouchmove(te: TouchEvent) {
		if (!view.state.transformer.isDragging) return;
		const e = te.touches[0];
		view.state.transformer.pan(e.clientX, e.clientY);
		markDirty();
	}

	function onmouseup() {
		view.state.transformer.panStop();
		markDirty();
	}

	function ontouchend() {
		view.state.transformer.panStop();
		markDirty();
	}

	function onwheel(e: WheelEvent) {
		view.state.transformer.zoom(e.deltaY, e.clientX, e.clientY);
		markDirty();
	}

	function onresize() {
		markDirty();
	}
</script>

<svelte:window
	onresize={withFrameCap(onresize, FRAME_RATE_CAP, ENABLE_FRAMERATE_CAP)}
	onmousemove={withFrameCap(onmousemove, FRAME_RATE_CAP, ENABLE_FRAMERATE_CAP)}
	ontouchmove={withFrameCap(ontouchmove, FRAME_RATE_CAP, ENABLE_FRAMERATE_CAP)}
	{onmouseup}
	{ontouchend}
	onwheel={withFrameCap(onwheel, FRAME_RATE_CAP, ENABLE_FRAMERATE_CAP)}
/>

<canvas
	{onmousedown}
	{ontouchstart}
	style="cursor: {view.state.transformer.isDragging ? 'grab' : 'crosshair'};"
	bind:this={canvas}
	class="absolute h-full w-full"
></canvas>
