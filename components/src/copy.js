export const copy_blob_as_image_with_navigator_clipboard = async (blob) => {
  await navigator.clipboard.write([
    new ClipboardItem({
      [blob.type]: blob,
    }),
  ]);
};
