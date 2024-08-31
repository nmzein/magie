import type { UploaderSettings } from '$types';
import { http } from '$api';
import { repository } from '$states';

export class Uploader {
	public parentDirectoryId: number | undefined = $state();
	public image: File | undefined = $state();
	public annotations: File | undefined = $state();
	// TODO: Rename to options
	public settings: UploaderSettings = $derived({
		generator: repository.generators[0],
		annotations: 'none'
	});

	public async upload() {
		if (this.parentDirectoryId === undefined || this.image === undefined) return;

		if (this.settings.annotations === 'provide') {
			await http.SendUploadAssets(
				this.parentDirectoryId,
				this.image,
				this.annotations,
				this.settings
			);
		} else {
			await http.SendUploadAssets(this.parentDirectoryId, this.image, undefined, this.settings);
		}

		this.reset();
	}

	public reset() {
		this.parentDirectoryId = undefined;
		this.image = undefined;
		this.annotations = undefined;
	}
}
