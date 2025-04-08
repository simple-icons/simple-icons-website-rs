export const add_scripts_ = (badgeMakerVersion, svgPathBboxVersion) => {
  if (!document.querySelector('script#badge-maker')) {
    const script = document.createElement('script');
    script.src = `/js/badge-maker-${badgeMakerVersion}.js`;
    script.id = 'badge-maker';
    script.defer = true;
    document.body.appendChild(script);
  }

  if (!document.querySelector('script#svg-path-bbox')) {
    const script = document.createElement('script');
    script.src = `/js/svg-path-bbox-${svgPathBboxVersion}.js`;
    script.id = 'svg-path-bbox';
    script.defer = true;
    document.body.appendChild(script);
  }
};

export const is_badge_maker_loaded = () =>
  window.makeBadgeWithBadgeMaker !== undefined;
