export const make_badge = (label, message, color, style, logoBase64) => {
  return window.makeBadgeWithBadgeMaker({
    label,
    message,
    color,
    style,
    logoBase64,
  });
};
