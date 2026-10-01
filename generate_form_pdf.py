from reportlab.pdfgen import canvas
from reportlab.lib.pagesizes import letter
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont

def create_form(path):
    c = canvas.Canvas(path, pagesize=letter)
    form = c.acroForm
    c.drawString(10, 750, 'Form Document')
    form.textfield(name='TestText', tooltip='Test Text',
                   x=10, y=700, width=150, height=20,
                   fillColor=None, borderColor=None,
                   textColor=None, forceBorder=False)
    form.checkbox(name='TestCheckbox', tooltip='Test Checkbox',
                  x=10, y=650, size=20, buttonStyle='check',
                  fillColor=None, borderColor=None,
                  textColor=None, forceBorder=False)
    c.save()

create_form('tests/e2e_fixtures/form.pdf')
create_form('tests/e2e_fixtures/form_filled.pdf')
