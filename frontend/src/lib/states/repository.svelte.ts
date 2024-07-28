import type { Directory } from '$types';
import { http } from '$api';

export class Repository {
	public registry: Directory | undefined = $state();
	public generators: string[] = $state([]);
	public decoders: string[] = $state([]);
	public encoders: string[] = $state([]);

	constructor() {
		http.GetRegistry().then((registry) => {
			if (registry === undefined) return;
			// TODO: Actually store on server and make it so that it always has index 0.
			registry.subdirectories.push({ id: -1, name: 'Bin', subdirectories: [], files: [] });
			this.registry = registry;
		});

		http.GetGenerators().then((generators) => {
			if (generators === undefined) return;
			this.generators = generators;
		});
	}
}
