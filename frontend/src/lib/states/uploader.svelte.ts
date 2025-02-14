import { http } from '$api';
import { defined } from '$helpers';
import { explorer, repository } from '$states';
import type { UploaderOptions } from '$types';

export class UploaderState {
	image: File | undefined = $state();
	annotations: File | undefined = $state();
	storeId: number | undefined = $state();
	parentId: number | undefined = $state();
	options: UploaderOptions = $state({
		name: '',
		encoder: repository.encoders[0],
		decoder: repository.decoders[0],
		generator: repository.generators[0],
		annotations: 'none'
	});
	imageSatisfied: boolean = $derived(defined(this.image) && this.options.name !== '');
	annotationsSatisfied: boolean = $derived(
		this.options.annotations === 'none' ||
			(this.options.annotations === 'provide' && defined(this.annotations))
	);
	currentPage: number = $state(0);
	#show = $state(false);

	get show() {
		return this.#show;
	}

	open() {
		this.#show = true;
		this.storeId = explorer!.storeId;
		this.parentId = explorer!.directoryId;
	}

	close() {
		this.#show = false;
	}

	async upload() {
		this.#show = false;

		if (
			!defined(this.storeId) ||
			!defined(this.parentId) ||
			!defined(this.image) ||
			(['provide', 'generate'].includes(this.options.annotations) &&
				!defined(this.options.generator))
		)
			return;

		if (this.options.annotations === 'provide') {
			await http.image.upload(
				this.storeId,
				this.parentId,
				this.image,
				this.annotations,
				this.options
			);
		} else {
			await http.image.upload(this.storeId, this.parentId, this.image, undefined, this.options);
		}

		this.reset();
	}

	reset() {
		this.image = undefined;
		this.annotations = undefined;
		this.currentPage = 0;
		this.options.name = '';
		this.storeId = undefined;
		this.parentId = undefined;
	}
}
