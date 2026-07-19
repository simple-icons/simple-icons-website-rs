// eslint-disable-next-line max-params
export const make_badge = (label, message, color, style, logoBase64) => {
	return globalThis.makeBadgeWithBadgeMaker({
		label,
		message,
		color,
		style,
		logoBase64,
	});
};
