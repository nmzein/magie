import type { UploaderOptions } from '$types';
import { http } from '$api';
import { repository } from '$states';

export class Uploader {
	public parentDirectoryId: number | undefined = $state();
	public image: File | undefined = $state();
	public annotations: File | undefined = $state();
	public currentPage: number = $state(0);
	public generator: string = $derived(repository.generators[0]);
	public options: UploaderOptions = $state({
		annotations: 'none'
	});
	public annotationsSatisfied: boolean = $derived(
		this.options.annotations === 'none' ||
			(this.options.annotations === 'provide' && this.annotations !== undefined)
	);

	public async upload() {
		if (
			this.parentDirectoryId === undefined ||
			this.image === undefined ||
			(['provide', 'generate'].includes(this.options.annotations) && this.generator === undefined)
		)
			return;

		if (this.options.annotations === 'provide') {
			await http.SendUploadAssets(
				this.parentDirectoryId,
				this.image,
				this.annotations,
				this.generator
			);
		} else {
			await http.SendUploadAssets(this.parentDirectoryId, this.image, undefined, this.generator);
		}

		this.reset();
	}

	public reset() {
		this.parentDirectoryId = undefined;
		this.image = undefined;
		this.annotations = undefined;
	}
}
