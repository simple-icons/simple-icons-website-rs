export function svg_path_segments_(path) {
  try {
    return [window.svgPath(path).segments, null];
  } catch (e) {
    return [null, e.message];
  }
}

export function svg_path_bbox_(path) {
  try {
    return [window.svgPathBbox(path), null];
  } catch (e) {
    return [[0, 0, 0, 0], e.message];
  }
}
