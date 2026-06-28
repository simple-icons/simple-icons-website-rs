/* global document, Image, ClipboardItem */

const [WIDTH, HEIGHT] = [640, 640];

export const download_image = async (slug, format, onload) => {
	const icon_svg_url = `/icons/${slug}.svg`;
	const response = await fetch(icon_svg_url);
	const svg = await response.text();

	const svgBlob = new Blob([svg], {
		type: 'image/svg+xml;charset=ascii',
	});

	const DOMURL = globalThis.URL || globalThis.webkitURL || globalThis;
	const url = DOMURL.createObjectURL(svgBlob);

	const canvas = document.createElement('canvas');
	canvas.width = WIDTH;
	canvas.height = HEIGHT;
	const ctx = canvas.getContext('2d');
	const img = new Image();
	img.width = WIDTH;
	img.height = HEIGHT;
	img.src = url;
	img.addEventListener('load', () => {
		ctx.drawImage(img, 0, 0, WIDTH, HEIGHT);
		onload(
			canvas
				.toDataURL(`image/${format}`)
				.replace(`image/${format}`, 'image/octet-stream'),
		);
	});
};

const setToLinkAndDownload = (url, slug, format) => {
	const a = document.createElement('a');
	a.href = url;
	a.download = `${slug}.${format}`;
	a.click();
	(globalThis.URL || globalThis.webkitURL || globalThis).revokeObjectURL(url);
};

export const download_png_ = (slug) => {
	return download_image(slug, 'png', (url) => {
		setToLinkAndDownload(url, slug, 'png');
	});
};

export const download_jpg_ = (slug) => {
	return download_image(slug, 'jpg', (url) => {
		setToLinkAndDownload(url, slug, 'jpg');
	});
};

export const copy_as_base64_jpg_ = (slug) => {
	download_image(slug, 'jpg', (url) => {
		const img = new Image();
		img.src = url;
		img.addEventListener('load', () => {
			const canvas = document.createElement('canvas');
			canvas.width = img.width;
			canvas.height = img.height;
			const ctx = canvas.getContext('2d');
			ctx.drawImage(img, 0, 0);
			const base64 = canvas.toDataURL('image/jpeg');
			navigator.clipboard.writeText(base64);
		});
	});
};

export const copy_as_base64_png_ = (slug) => {
	download_image(slug, 'png', (url) => {
		const img = new Image();
		img.src = url;
		img.addEventListener('load', () => {
			const canvas = document.createElement('canvas');
			canvas.width = img.width;
			canvas.height = img.height;
			const ctx = canvas.getContext('2d');
			ctx.drawImage(img, 0, 0);
			const base64 = canvas.toDataURL('image/png');
			navigator.clipboard.writeText(base64);
		});
	});
};

const copy_as_image = (slug, format) => {
	download_image(slug, format, (url) => {
		const img = new Image();
		img.src = url;
		img.addEventListener('load', () => {
			const canvas = document.createElement('canvas');
			canvas.width = img.width;
			canvas.height = img.height;
			const ctx = canvas.getContext('2d');
			ctx.drawImage(img, 0, 0);
			canvas.toBlob((blob) => {
				navigator.clipboard.write([
					new ClipboardItem({
						[blob.type]: blob,
					}),
				]);
			});
		});
	});
};

export const copy_as_image_png_ = (slug) => {
	copy_as_image(slug, 'png');
};

export const copy_as_image_jpg_ = (slug) => {
	copy_as_image(slug, 'jpg');
};
