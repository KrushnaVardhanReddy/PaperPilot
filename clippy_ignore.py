import re

with open("paperpilot-pdf/src/operations/form.rs", "r") as f:
    form_rs = f.read()

form_rs = form_rs.replace("""impl PdfOperation for ReadFormOperation {""", """#[allow(clippy::collapsible_if)]\nimpl PdfOperation for ReadFormOperation {""")
form_rs = form_rs.replace("""impl PdfOperation for FillFormOperation {""", """#[allow(clippy::collapsible_if)]\nimpl PdfOperation for FillFormOperation {""")
form_rs = form_rs.replace("""impl PdfOperation for CreateFormFieldOperation {""", """#[allow(clippy::collapsible_if)]\nimpl PdfOperation for CreateFormFieldOperation {""")

with open("paperpilot-pdf/src/operations/form.rs", "w") as f:
    f.write(form_rs)

with open("paperpilot-pdf/src/operations/classify.rs", "r") as f:
    classify_rs = f.read()

classify_rs = classify_rs.replace("""    pub fn new() -> Self {""", """    #[allow(clippy::new_without_default)]\n    pub fn new() -> Self {""")
classify_rs = classify_rs.replace('text.push_str(" ");', "text.push(' ');")

with open("paperpilot-pdf/src/operations/classify.rs", "w") as f:
    f.write(classify_rs)
