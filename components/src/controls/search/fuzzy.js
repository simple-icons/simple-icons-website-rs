window.SEARCHER = null;

export function build_searcher(candidates) {
  window.SEARCHER = new window.Searcher(candidates, {
    keySelector: (item) => item[0],
  });
}

export function search(query) {
  return window.SEARCHER.search(query);
}
