import { http } from '$api';
import { defined } from '$helpers';

export class Repository {
	#generators: string[] = $state([]);
	#decoders: string[] = $state(['Auto (default)']);
	#encoders: string[] = $state(['OMEZarr']);

	get generators() {
		return this.#generators;
	}

	get decoders() {
		return this.#decoders;
	}

	get encoders() {
		return this.#encoders;
	}

	constructor() {
		http.generators().then((generators) => {
			if (!defined(generators)) return;
			this.#generators = generators;
		});
	}
}
