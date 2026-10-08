import os
from reportlab.pdfgen import canvas
from reportlab.lib.pagesizes import letter
from reportlab.lib.colors import black, white
import pypdf
from PIL import Image

FIXTURE_DIR = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "tests", "e2e_fixtures", "real"))
os.makedirs(FIXTURE_DIR, exist_ok=True)

def create_merge_a():
    c = canvas.Canvas(os.path.join(FIXTURE_DIR, "merge_a.pdf"), pagesize=letter)
    c.drawString(100, 700, "MERGE_PAGE_AAA")
    c.save()

def create_merge_b():
    c = canvas.Canvas(os.path.join(FIXTURE_DIR, "merge_b.pdf"), pagesize=letter)
    c.drawString(100, 700, "MERGE_PAGE_BBB")
    c.save()

def create_multi_page():
    c = canvas.Canvas(os.path.join(FIXTURE_DIR, "multi_page.pdf"), pagesize=letter)
    for i in range(1, 6):
        c.drawString(100, 700, f"PAGE_TEXT_P{i}")
        c.showPage()
    c.save()

def create_with_blank():
    c = canvas.Canvas(os.path.join(FIXTURE_DIR, "with_blank.pdf"), pagesize=letter)
    c.drawString(100, 700, "CONTENT_PAGE_1")
    c.showPage()
    c.showPage() # Blank
    c.drawString(100, 700, "CONTENT_PAGE_3")
    c.showPage()
    c.drawString(100, 700, " ") # Near blank
    c.showPage()
    c.drawString(100, 700, "CONTENT_PAGE_5")
    c.showPage()
    c.save()

def create_redact_test():
    c = canvas.Canvas(os.path.join(FIXTURE_DIR, "redact_test.pdf"), pagesize=letter)
    c.drawString(100, 700, "HEADER_INFO")
    c.drawString(100, 600, "SECRET 12345")
    c.drawString(100, 500, "FOOTER_INFO")
    c.save()

def create_form():
    c = canvas.Canvas(os.path.join(FIXTURE_DIR, "form.pdf"), pagesize=letter)
    form = c.acroForm
    c.drawString(10, 700, "First Name:")
    form.textfield(name='first_name', tooltip='First Name',
                   x=110, y=680, borderStyle='inset',
                   width=300, height=30, textColor=black, fillColor=white)
    c.drawString(10, 650, "Agree:")
    form.checkbox(name='agree', tooltip='Agree to terms',
                  x=110, y=630, buttonStyle='check',
                  size=20, textColor=black, fillColor=white)
    c.drawString(10, 600, "Country:")
    form.choice(name='country', tooltip='Select Country',
                value='USA', options=[('USA', 'United States'), ('CAN', 'Canada')],
                x=110, y=580, width=300, height=30)
    c.save()

def create_encrypted():
    create_multi_page()
    reader = pypdf.PdfReader(os.path.join(FIXTURE_DIR, "multi_page.pdf"))
    writer = pypdf.PdfWriter()
    writer.append_pages_from_reader(reader)
    writer.encrypt("user123", "owner123")
    with open(os.path.join(FIXTURE_DIR, "encrypted.pdf"), "wb") as f:
        writer.write(f)

def create_broken_xref():
    # create a valid pdf, then truncate it
    create_multi_page()
    with open(os.path.join(FIXTURE_DIR, "multi_page.pdf"), "rb") as f:
        data = f.read()
    with open(os.path.join(FIXTURE_DIR, "broken_xref.pdf"), "wb") as f:
        # truncate last 100 bytes
        f.write(data[:-100])

def create_invoice_text():
    c = canvas.Canvas(os.path.join(FIXTURE_DIR, "invoice_text.pdf"), pagesize=letter)
    c.drawString(100, 700, "Invoice 4821")
    c.drawString(100, 600, "Item 1: $500.00")
    c.drawString(100, 500, "Total $1,250.00")
    c.showPage()
    c.drawString(100, 700, "Invoice 4821 - Page 2")
    c.drawString(100, 600, "Item 2: $750.00")
    c.drawString(100, 500, "Total $1,250.00")
    c.showPage()
    c.drawString(100, 700, "Invoice 4821 - Page 3")
    c.drawString(100, 600, "Terms and Conditions")
    c.drawString(100, 500, "Total $1,250.00")
    c.save()

def create_img_sample():
    img = Image.new('RGB', (800, 600), color = 'red')
    img.save(os.path.join(FIXTURE_DIR, "img_sample.png"))

def create_csv():
    with open(os.path.join(FIXTURE_DIR, "test.csv"), "w") as f:
        f.write("ColA,ColB,ColC\n")
        for i in range(1, 6):
            f.write(f"10,20,30\n")

def create_html():
    with open(os.path.join(FIXTURE_DIR, "test.html"), "w") as f:
        f.write("<h1>HTML Test</h1>")

def create_md():
    with open(os.path.join(FIXTURE_DIR, "test.md"), "w") as f:
        f.write("# MD Test\n")

if __name__ == "__main__":
    print(f"Generating fixtures in {FIXTURE_DIR}...")
    create_merge_a()
    create_merge_b()
    create_multi_page()
    create_with_blank()
    create_redact_test()
    create_form()
    create_encrypted()
    create_broken_xref()
    create_invoice_text()
    create_img_sample()
    create_csv()
    create_html()
    create_md()

    # Needs to create image_doc.pdf for pdf_ocr
    c = canvas.Canvas(os.path.join(FIXTURE_DIR, "image_doc.pdf"), pagesize=letter)
    c.drawImage(os.path.join(FIXTURE_DIR, "img_sample.png"), 0, 0, width=500, height=400)
    c.save()

    print("Done generating fixtures.")
