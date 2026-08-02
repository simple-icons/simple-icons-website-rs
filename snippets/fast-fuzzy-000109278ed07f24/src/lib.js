globalThis.SEARCHER = null;

export function build_searcher(candidates) {
	globalThis.SEARCHER = new globalThis.Searcher(candidates, {
		keySelector: (item) => item[0],
	});
}

export function search(query) {
	return globalThis.SEARCHER.search(query);
}
