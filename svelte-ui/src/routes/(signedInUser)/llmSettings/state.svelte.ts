class GlobalLLMState {
	readonly defaultFavModels = [0, 1, 2, 3];
	settings: LLMSettings = $state({
		apiConfigs: [],
		promptConfigs: [],
		favModels: []
	});
	// Private mutable state
	private _activeFav: FavModel | undefined = $state();
	// Public read-only state
	get activeFav() {
		return this._activeFav;
	}

	setActiveFavById(id: number) {
		this._activeFav = this.settings.favModels.find((model: FavModel) => model.id === id)!;
		localStorage.setItem('activeFavId', id.toString());
	}
}

export const llmState = new GlobalLLMState();

export enum LlmSdk {
	OpenAI = 'OpenAI',
	Anthropic = 'Anthropic',
	OpenAICompatible = 'OpenAICompatible'
}
