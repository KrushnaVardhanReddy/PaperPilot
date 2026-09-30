import os
from reportlab.pdfgen import canvas
from reportlab.lib.pagesizes import letter
from PyPDF2 import PdfReader, PdfWriter

def create_pdf(path, pages, text_prefix):
    c = canvas.Canvas(path, pagesize=letter)
    for i in range(pages):
        c.drawString(100, 700, f"{text_prefix} - Page {i+1}")
        c.showPage()
    c.save()

def create_image_pdf(path):
    c = canvas.Canvas(path, pagesize=letter)
    for i in range(3):
        c.drawString(100, 700, f"Image Doc - Page {i+1}")
        # Note: We won't actually embed an image, but text should be fine, or let's create a dummy image
        c.showPage()
    c.save()

def encrypt_pdf(input_path, output_path, password):
    reader = PdfReader(input_path)
    writer = PdfWriter()
    for page in reader.pages:
        writer.add_page(page)
    writer.encrypt(password)
    with open(output_path, "wb") as f:
        writer.write(f)

def main():
    os.makedirs("tests/e2e_fixtures", exist_ok=True)
    create_pdf("tests/e2e_fixtures/single_page.pdf", 1, "Single Page")
    create_pdf("tests/e2e_fixtures/multi_page.pdf", 5, "Multi Page")
    create_pdf("tests/e2e_fixtures/large_doc.pdf", 20, "Large Doc Report")
    create_image_pdf("tests/e2e_fixtures/image_doc.pdf")
    encrypt_pdf("tests/e2e_fixtures/single_page.pdf", "tests/e2e_fixtures/encrypted.pdf", "testpass123")

    # Let's also create dummy images for `pdf_images_to_pdf`
    # We will use PIL to create dummy images
    from PIL import Image
    img1 = Image.new('RGB', (100, 100), color = 'red')
    img1.save('tests/e2e_fixtures/img1.png')
    img2 = Image.new('RGB', (100, 100), color = 'blue')
    img2.save('tests/e2e_fixtures/img2.png')

if __name__ == "__main__":
    main()
