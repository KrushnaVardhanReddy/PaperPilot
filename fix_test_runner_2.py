import ast
import re

with open('scripts/test_tri_interface_e2e.py', 'r') as f:
    content = f.read()

# Fix pypdf error
new_decrypt = '''        elif tool_id == "pdf_decrypt":
            if not os.path.exists(output_path): return False, "Output not found", "No file"
            try:
                reader = pypdf.PdfReader(output_path)
                if reader.is_encrypted: return False, "Still encrypted", "Encryption check"
                if "PAGE_TEXT_P1" not in reader.pages[0].extract_text(): return False, "Decrypted text mismatch", "Text mismatch"
                return True, "Decrypted successfully, text intact", None
            except Exception as e:
                return False, f"Failed to decrypt/read: {e}", "Parse Error"'''

content = re.sub(
    r'        elif tool_id == "pdf_decrypt":\n            if not os\.path\.exists\(output_path\): return False, "Output not found", "No file"\n            reader = pypdf\.PdfReader\(output_path\)\n            if reader\.is_encrypted: return False, "Still encrypted", "Encryption check"\n            if "PAGE_TEXT_P1" not in reader\.pages\[0\]\.extract_text\(\): return False, "Decrypted text mismatch", "Text mismatch"\n            return True, "Decrypted successfully, text intact", None',
    new_decrypt,
    content
)

# And fix encrypt similarly just in case
new_encrypt = '''        elif tool_id == "pdf_encrypt":
            if not os.path.exists(output_path): return False, "Output not found", "No file"
            try:
                reader = pypdf.PdfReader(output_path)
                if not reader.is_encrypted: return False, "Not encrypted", "Encryption check"
                try:
                    reader.pages[0]
                    return False, "Could read without password", "Security hole"
                except Exception:
                    pass
                reader.decrypt("secret123")
                if "PAGE_TEXT_P1" not in reader.pages[0].extract_text(): return False, "Decrypted text mismatch", "Text mismatch"
                return True, "Requires password to read, text intact after decrypt", None
            except Exception as e:
                return False, f"Failed to read encrypted: {e}", "Parse Error"'''

content = re.sub(
    r'        elif tool_id == "pdf_encrypt":\n            if not os\.path\.exists\(output_path\): return False, "Output not found", "No file"\n            reader = pypdf\.PdfReader\(output_path\)\n            if not reader\.is_encrypted: return False, "Not encrypted", "Encryption check"\n            try:\n                reader\.pages\[0\]\n                return False, "Could read without password", "Security hole"\n            except Exception:\n                pass\n            reader\.decrypt\("secret123"\)\n            if "PAGE_TEXT_P1" not in reader\.pages\[0\]\.extract_text\(\): return False, "Decrypted text mismatch", "Text mismatch"\n            return True, "Requires password to read, text intact after decrypt", None',
    new_encrypt,
    content
)

with open('scripts/test_tri_interface_e2e.py', 'w') as f:
    f.write(content)
