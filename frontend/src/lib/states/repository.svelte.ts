import { http } from '$api';
import { defined } from '$helpers';

export class Repository {
	#generators: string[] = $state([]);
	#decoders: string[] = $state([]);
	#encoders: string[] = $state([]);

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
		$effect.root(() => {
			$effect(() => {
				http.modules().then((modules) => {
					if (!defined(modules)) return;
					this.#generators = modules.generators;
					this.#decoders = modules.decoders;
					this.#encoders = modules.encoders;
				});
			});
		});
	}
}
