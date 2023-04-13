export async function download_pdf_(slug, errorMessageSchema) {
  const icon_svg_url = `/icons/${slug}.svg`;
  const res = await fetch(icon_svg_url);
  const svg = await res.text();
  const svg_path = svg.split('"')[7];

  let doc, stream;
  try {
    doc = new PDFDocument({ size: [24, 24] });
    stream = doc.pipe(blobStream());
    doc.path(svg_path).fill();
  } catch (e) {
    // Some icon paths are not parsed correctly by PDFKit ('/e/' for example)
    // so we catch the error and generate a PDF with the error message
    doc = new PDFDocument({ size: 'A8' });
    stream = doc.pipe(blobStream());
    console.error(e);
    doc.fontSize(12);
    doc.text(errorMessageSchema.replace('{}', e.message), 0, 0, {
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
}
