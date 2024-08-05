import type { UploaderSettings } from '$types';
import { http } from '$api';
import { repository } from '$states';

export const Uploader = () => {
	let parentDirectoryId: number | undefined = $state();
	let image: File | undefined = $state();
	let annotations: File | undefined = $state();
	// TODO: Rename to options
	let settings: UploaderSettings = $derived({
		generator: repository.generators[0],
		annotations: 'none'
	});

	async function upload() {
		if (parentDirectoryId === undefined || image === undefined) return;

		if (settings.annotations === 'provide') {
			await http.SendUploadAssets(parentDirectoryId, image, annotations, settings);
		} else {
			await http.SendUploadAssets(parentDirectoryId, image, undefined, settings);
		}

		reset();
	}

	function reset() {
		parentDirectoryId = undefined;
		image = undefined;
		annotations = undefined;
	}

	return {
		set parentDirectoryId(value: number | undefined) {
			parentDirectoryId = value;
		},
		set image(value: File | undefined) {
			image = value;
		},
		set annotations(value: File | undefined) {
			annotations = value;
		},
		settings,
		upload,
		reset
	};
};
