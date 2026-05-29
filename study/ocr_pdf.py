"""
OCR a PDF to text using easyocr + PyMuPDF.
Usage: python ocr_pdf.py <input.pdf> [start_page] [end_page]
Output: study/ocr-output/<filename>.txt
"""
import sys
import os
import fitz  # PyMuPDF
import easyocr

def main():
    if len(sys.argv) < 2:
        print("Usage: python ocr_pdf.py <input.pdf> [start_page] [end_page]")
        sys.exit(1)

    pdf_path = sys.argv[1]
    if not os.path.exists(pdf_path):
        print(f"File not found: {pdf_path}")
        sys.exit(1)

    # Open PDF
    doc = fitz.open(pdf_path)
    total_pages = doc.page_count

    start = int(sys.argv[2]) - 1 if len(sys.argv) > 2 else 0
    end = int(sys.argv[3]) if len(sys.argv) > 3 else total_pages
    start = max(0, start)
    end = min(total_pages, end)

    print(f"PDF: {pdf_path}")
    print(f"Pages: {start+1}–{end} of {total_pages}")

    # Init OCR (Chinese + English)
    print("Loading OCR model (first run downloads model, ~100MB)...")
    reader = easyocr.Reader(['ch_sim', 'en'], gpu=False)
    print("OCR ready.")

    # Output path
    basename = os.path.splitext(os.path.basename(pdf_path))[0]
    out_dir = os.path.join(os.path.dirname(__file__), "ocr-output")
    os.makedirs(out_dir, exist_ok=True)
    out_path = os.path.join(out_dir, f"{basename}_p{start+1}-{end}.txt")

    with open(out_path, "w", encoding="utf-8") as out:
        for i in range(start, end):
            page_num = i + 1
            print(f"  OCR page {page_num}/{end}...", end=" ", flush=True)

            # Render page at 300 DPI for good OCR quality
            page = doc[i]
            pix = page.get_pixmap(dpi=300)
            img_bytes = pix.tobytes("png")

            # OCR
            results = reader.readtext(img_bytes, detail=0)
            text = "\n".join(results)

            out.write(f"\n{'='*60}\n")
            out.write(f"=== Page {page_num} ===\n")
            out.write(f"{'='*60}\n\n")
            out.write(text)
            out.write("\n")

            print(f"{len(text)} chars")

    doc.close()
    print(f"\nDone. Output: {out_path}")

if __name__ == "__main__":
    main()
