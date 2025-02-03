import type { Directory } from '$types';
import { http } from '$api';
import { InitExplorerState } from '$states';
import { defined } from '$helpers';

export class RepositoryState {
	#registry: Directory | undefined = $state();
	#generators: string[] = $state([]);
	#decoders: string[] = $state(['Auto (default)']);
	#encoders: string[] = $state(['OMEZarr']);

	get registry() {
		return this.#registry;
	}

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
		http.registry().then((registry) => {
			if (!defined(registry)) return;
			this.#registry = registry;
			InitExplorerState();
		});

		http.generators().then((generators) => {
			if (!defined(generators)) return;
			this.#generators = generators;
		});
	}
}
