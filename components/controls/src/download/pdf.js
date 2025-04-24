/* global document, Image */

export const add_scripts_ = (pdfkitVersion, blobStreamVersion) => {
	if (!document.querySelector('script#pdfkit')) {
		const script = document.createElement('script');
		script.src = `/js/pdfkit-${pdfkitVersion}.js`;
		script.id = 'pdfkit';
		script.defer = true;
		document.body.append(script);
	}

	if (!document.querySelector('script#blob-stream')) {
		const script = document.createElement('script');
		script.src = `/js/blob-stream-${blobStreamVersion}.js`;
		script.id = 'blob-stream';
		script.defer = true;
		document.body.append(script);
	}
};

const waitForPdfkitAndBlobStreamOnWindow = () =>
	new Promise((resolve) => {
		const interval = setInterval(() => {
			if (globalThis.PDFDocument && globalThis.blobStream) {
				clearInterval(interval);
				resolve();
			}
		}, 100);
	});

export const download_pdf_ = async (
	slug,
	errorGeneratingPdfMessage,
	pdfkitVersion,
	blobStreamVersion,
) => {
	const icon_svg_url = `/icons/${slug}.svg`;
	add_scripts_(pdfkitVersion, blobStreamVersion);
	const [response, ,] = await Promise.all([
		fetch(icon_svg_url),
		waitForPdfkitAndBlobStreamOnWindow(),
	]);
	const svg = await response.text();

	const [width, height] = [480, 480];

	const svgBlob = new Blob([svg], {
		type: 'image/svg+xml;charset=utf-8',
	});

	const url = (
		globalThis.URL ||
		globalThis.webkitURL ||
		globalThis
	).createObjectURL(svgBlob);

	const canvas = document.createElement('canvas');
	canvas.width = width;
	canvas.height = height;
	const ctx = canvas.getContext('2d');
	const img = new Image();
	img.width = width;
	img.height = height;
	img.src = url;
	img.addEventListener('load', () => {
		ctx.drawImage(img, 0, 0, width, height);
		const canvasDataUrl = canvas.toDataURL(`image/png`);

		let doc;
		let stream;
		try {
			doc = new globalThis.PDFDocument({size: [width, height]});
			stream = doc.pipe(globalThis.blobStream());
			doc.image(canvasDataUrl, 0, 0);
			doc.save('invoice.pdf');
		} catch (error) {
			// Some icon paths are not parsed correctly by PDFKit ('/e/' for example)
			// so we catch the error and generate a PDF with the error message
			doc = new globalThis.PDFDocument({size: 'A8'});
			stream = doc.pipe(globalThis.blobStream());
			console.error(error);
			doc.fontSize(12);
			doc.text(`${errorGeneratingPdfMessage} ${error.message}`, 0, 0, {
				align: 'center',
			});
		}

		doc.end();
		stream.on('finish', () => {
			const url_ = stream.toBlobURL('application/pdf');
			const a = document.createElement('a');
			a.href = url_;
			a.download = `${slug}.pdf`;
			a.click();
			(globalThis.URL || globalThis.webkitURL || globalThis).revokeObjectURL(
				url,
			);
		});
	});
};
