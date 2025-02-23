export const add_scripts_ = (pdfkitVersion, blobStreamVersion) => {
  if (!document.querySelector('script#pdfkit')) {
    const script = document.createElement('script');
    script.src = `/js/pdfkit-${pdfkitVersion}.js`;
    script.id = 'pdfkit';
    script.defer = true;
    document.body.appendChild(script);
  }

  if (!document.querySelector('script#blob-stream')) {
    const script = document.createElement('script');
    script.src = `/js/blob-stream-${blobStreamVersion}.js`;
    script.id = 'blob-stream';
    script.defer = true;
    document.body.appendChild(script);
  }
};

const waitForPdfkitAndBlobStreamOnWindow = () =>
  new Promise((resolve) => {
    const interval = setInterval(() => {
      if (window.PDFDocument && window.blobStream) {
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
  const [res, ,] = await Promise.all([
    fetch(icon_svg_url),
    waitForPdfkitAndBlobStreamOnWindow(),
  ]);
  const svg = await res.text();

  const [width, height] = [480, 480];

  const svgBlob = new Blob([svg], {
    type: 'image/svg+xml;charset=utf-8',
  });

  const url = (window.URL || window.webkitURL || window).createObjectURL(
    svgBlob,
  );

  const canvas = document.createElement('canvas');
  canvas.width = width;
  canvas.height = height;
  const ctx = canvas.getContext('2d');
  const img = new Image();
  img.width = width;
  img.height = height;
  img.src = url;
  img.onload = () => {
    ctx.drawImage(img, 0, 0, width, height);
    const canvasDataUrl = canvas.toDataURL(`image/png`);

    let doc;
    let stream;
    try {
      doc = new window.PDFDocument({ size: [width, height] });
      stream = doc.pipe(window.blobStream());
      doc.image(canvasDataUrl, 0, 0);
      doc.save('invoice.pdf');
    } catch (e) {
      // Some icon paths are not parsed correctly by PDFKit ('/e/' for example)
      // so we catch the error and generate a PDF with the error message
      doc = new window.PDFDocument({ size: 'A8' });
      stream = doc.pipe(window.blobStream());
      console.error(e);
      doc.fontSize(12);
      doc.text(`${errorGeneratingPdfMessage} ${e.message}`, 0, 0, {
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
      (window.URL || window.webkitURL || window).revokeObjectURL(url);
    });
  };
};
