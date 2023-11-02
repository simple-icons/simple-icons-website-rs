export function svg_path_bbox_(path) {
  try {
    return [window.svgPathBbox(path), null];
  } catch (e) {
    return [[0, 0, 0, 0], e.message];
  }
}
