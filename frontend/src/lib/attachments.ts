import type { Attachment } from 'svelte/attachments';

export const boundingClientRect = (callback: (rect: DOMRect) => void): Attachment => {
	return (element) => {
		function loop() {
			callback(element.getBoundingClientRect());
			requestAnimationFrame(loop);
		}

		loop();
	};
};

export const resizeObserver = (callback: (rect: DOMRect) => void): Attachment => {
	return (element) => {
		function update() {
			callback(element.getBoundingClientRect());
		}

		const observer = new globalThis.ResizeObserver(update);
		observer.observe(element);

		update();

		return () => observer.disconnect();
	};
};
