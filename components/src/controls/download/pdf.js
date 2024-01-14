const addScripts = (pdfkitVersion, blobStreamVersion) => {
  if (!window.PDFDocument) {
    const script = document.createElement('script');
    script.src = `/js/pdfkit-${pdfkitVersion}.js`;
    script.defer = true;
    document.body.appendChild(script);
  }
  if (!window.blobStream) {
    const script = document.createElement('script');
    script.src = `/js/blob-stream-${blobStreamVersion}.js`;
    script.defer = true;
    document.body.appendChild(script);
  }

  return new Promise((resolve) => {
    const interval = setInterval(() => {
      if (window.PDFDocument && window.blobStream) {
        clearInterval(interval);
        resolve();
      }
    }, 100);
  });
};

export const download_pdf_ = async (
  slug,
  errorGeneratingPdfMessage,
  pdfkitVersion,
  blobStreamVersion,
) => {
  const icon_svg_url = `/icons/${slug}.svg`;
  const [res, ,] = await Promise.all([
    fetch(icon_svg_url),
    addScripts(pdfkitVersion, blobStreamVersion),
  ]);
  const svg = await res.text();
  const svg_path = svg.split('"')[7];

  let doc;
  let stream;
  try {
    doc = new window.PDFDocument({ size: [24, 24] });
    stream = doc.pipe(window.blobStream());
    doc.path(svg_path).fill();
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
    const url = stream.toBlobURL('application/pdf');
    const a = document.createElement('a');
    a.href = url;
    a.download = `${slug}.pdf`;
    document.body.appendChild(a);
    a.click();
    URL.revokeObjectURL(url);
  });
};
