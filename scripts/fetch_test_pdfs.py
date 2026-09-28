import os
import urllib.request

FIXTURES_DIR = os.path.join(os.path.dirname(__file__), "..", "tests", "fixtures")
os.makedirs(FIXTURES_DIR, exist_ok=True)

# A collection of public domain / CC0 test PDFs covering different edge cases
PDFS = {
    "simple.pdf": "https://www.w3.org/WAI/ER/tests/xhtml/testfiles/resources/pdf/dummy.pdf",
    "multi_page.pdf": "https://pdfobject.com/pdf/sample-3pp.pdf",
    "text_heavy.pdf": "https://www.gutenberg.org/files/1342/1342-pdf.pdf", # Pride & Prejudice (large)
}

def download_file(url, filename):
    path = os.path.join(FIXTURES_DIR, filename)
    if os.path.exists(path):
        print(f"✅ Already exists: {filename}")
        return
    print(f"⬇️  Downloading {filename}...")
    try:
        urllib.request.urlretrieve(url, path)
        print(f"✅ Saved to {path}")
    except Exception as e:
        print(f"❌ Failed to download {filename}: {e}")

if __name__ == "__main__":
    print("🚀 Fetching PDF test corpus...")
    for filename, url in PDFS.items():
        download_file(url, filename)
    print("\n✅ All test fixtures ready.")
