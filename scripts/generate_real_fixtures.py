import os
import shutil
import hashlib
from reportlab.pdfgen import canvas
from reportlab.lib.pagesizes import letter
from reportlab.lib.colors import black
from PIL import Image

def generate_fixtures():
    base_dir = os.path.dirname(os.path.abspath(__file__))
    fixtures_dir = os.path.abspath(os.path.join(base_dir, "..", "tests", "e2e_fixtures"))
    real_dir = os.path.join(fixtures_dir, "real")
    os.makedirs(real_dir, exist_ok=True)

    print(f"Generating fixtures in {real_dir}...")

    # 1. multi_page.pdf
    multi_path = os.path.join(real_dir, "multi_page.pdf")
    c = canvas.Canvas(multi_path, pagesize=letter)
    for i in range(1, 6):
        c.drawString(100, 700, f"P{i}")
        c.drawString(100, 680, f"This is page {i} of the document.")
        c.showPage()
    c.save()

    # 2. invoice_text.pdf
    invoice_path = os.path.join(real_dir, "invoice_text.pdf")
    c = canvas.Canvas(invoice_path, pagesize=letter)
    for i in range(1, 4):
        c.drawString(100, 750, f"Invoice {4820 + i}")
        c.drawString(100, 730, f"Total $1,250.00")
        c.drawString(100, 710, "Table header 1 | Table header 2")
        c.drawString(100, 690, "Row 1 Col 1    | Row 1 Col 2")
        c.showPage()
    c.save()

    # 3. with_blank.pdf
    blank_path = os.path.join(real_dir, "with_blank.pdf")
    c = canvas.Canvas(blank_path, pagesize=letter)
    c.drawString(100, 750, "Text Page 1")
    c.showPage() # Page 1
    c.showPage() # Page 2 (Blank)
    c.drawString(100, 750, "Text Page 3")
    c.showPage() # Page 3
    c.drawString(100, 750, " ") # Page 4 (Near blank / space)
    c.showPage()
    c.drawString(100, 750, "Text Page 5")
    c.showPage() # Page 5
    c.save()

    # 4. redact_test.pdf
    redact_path = os.path.join(real_dir, "redact_test.pdf")
    c = canvas.Canvas(redact_path, pagesize=letter)
    c.drawString(50, 700, "This document contains a secret:")
    c.drawString(50, 680, "SECRET 12345") # target for redaction at roughly y=680
    c.drawString(50, 660, "The secret is above.")
    c.showPage()
    c.save()

    # 5. form.pdf
    # Reportlab form generation is limited, but we can try to copy from existing or make a basic one
    form_path = os.path.join(real_dir, "form.pdf")
    orig_form = os.path.join(fixtures_dir, "form.pdf")
    if os.path.exists(orig_form):
        shutil.copy2(orig_form, form_path)
    else:
        # fallback simple PDF if form doesn't exist (though specs say to test forms)
        c = canvas.Canvas(form_path, pagesize=letter)
        c.drawString(100, 700, "Fallback Form Document")
        c.showPage()
        c.save()

    # 6. encrypted.pdf
    enc_path = os.path.join(real_dir, "encrypted.pdf")
    orig_enc = os.path.join(fixtures_dir, "encrypted.pdf")
    if os.path.exists(orig_enc):
        shutil.copy2(orig_enc, enc_path)
    else:
        # Create unencrypted one to be encrypted by PyPDF2 if needed, but we rely on existing
        c = canvas.Canvas(enc_path, pagesize=letter)
        c.drawString(100, 700, "This should be encrypted.")
        c.showPage()
        c.save()

    # 7. broken_xref.pdf
    broken_path = os.path.join(real_dir, "broken_xref.pdf")
    c = canvas.Canvas(broken_path, pagesize=letter)
    c.drawString(100, 700, "Valid content before corruption.")
    c.showPage()
    c.save()
    with open(broken_path, "rb+") as f:
        content = f.read()
        # Corrupt the EOF marker
        content = content.replace(b"%%EOF", b"%%CORRUPT")
        f.seek(0)
        f.write(content)
        f.truncate()

    # 8. img_sample.png
    img_path = os.path.join(real_dir, "img_sample.png")
    img = Image.new('RGB', (800, 600), color = 'red')
    img.save(img_path)

    # Need image_doc for OCR testing
    image_doc_path = os.path.join(real_dir, "image_doc.pdf")
    if os.path.exists(os.path.join(fixtures_dir, "image_doc.pdf")):
        shutil.copy2(os.path.join(fixtures_dir, "image_doc.pdf"), image_doc_path)
    else:
        c = canvas.Canvas(image_doc_path, pagesize=letter)
        c.drawImage(img_path, 100, 500, width=400, height=300)
        c.showPage()
        c.save()

    # Extra fixtures for tests that need single_page, search, etc.
    single_path = os.path.join(real_dir, "single_page.pdf")
    c = canvas.Canvas(single_path, pagesize=letter)
    c.drawString(100, 700, "This is a single page document.")
    c.showPage()
    c.save()

    search_path = os.path.join(real_dir, "search_test.pdf")
    c = canvas.Canvas(search_path, pagesize=letter)
    c.drawString(100, 700, "This is page 1 with a test string.")
    c.showPage()
    c.drawString(100, 700, "This is page 2.")
    c.showPage()
    c.drawString(100, 700, "This is page 3 with another test.")
    c.showPage()
    c.save()

    html_path = os.path.join(real_dir, "test.html")
    with open(html_path, "w") as f:
        f.write("<html><body><h1>HTML Test</h1><p>Paragraph</p></body></html>")

    md_path = os.path.join(real_dir, "test.md")
    with open(md_path, "w") as f:
        f.write("# MD Test\n\n- List item 1\n- List item 2")

    csv_path = os.path.join(real_dir, "test.csv")
    with open(csv_path, "w") as f:
        f.write("A,B,C\n1,2,3\n4,5,6")

    img1_path = os.path.join(real_dir, "img1.png")
    img2_path = os.path.join(real_dir, "img2.png")
    Image.new('RGB', (800, 600), color='blue').save(img1_path)
    Image.new('RGB', (800, 600), color='green').save(img2_path)

    # Need dummy p12 for signing
    p12_path = os.path.join(real_dir, "dummy.p12")
    if os.path.exists(os.path.join(fixtures_dir, "out", "tri_e2e", "dummy.p12")):
        shutil.copy2(os.path.join(fixtures_dir, "out", "tri_e2e", "dummy.p12"), p12_path)
    else:
        with open(p12_path, "wb") as f:
            f.write(b"dummy_cert_data")

    print("Fixtures generated successfully.")

if __name__ == "__main__":
    generate_fixtures()
