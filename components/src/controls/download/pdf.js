export const download_pdf_ = async (slug, errorGeneratingPdfMessage) => {
  const icon_svg_url = `/icons/${slug}.svg`;
  const res = await fetch(icon_svg_url);
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
