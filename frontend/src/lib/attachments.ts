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

export const resizeObserver = (callback: (element: Element) => void): Attachment => {
	return (element) => {
		const observer = new ResizeObserver((entries, _observer) => callback(entries[0].target));
		observer.observe(element);

		return () => observer.disconnect();
	};
};
