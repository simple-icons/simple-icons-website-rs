export function svg_path_bbox_(path) {
	try {
		return [globalThis.svgPathBbox(path), null];
	} catch (error) {
		return [[0, 0, 0, 0], error.message];
	}
}
