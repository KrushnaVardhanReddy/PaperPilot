import ast
import re

with open('scripts/test_tri_interface_e2e.py', 'r') as f:
    content = f.read()

# Fix stdout_or_json type issue (could be bytes from API vs string from CLI)
new_check = '''            if isinstance(stdout_or_json, bytes):
                stdout_or_json = stdout_or_json.decode('utf-8', errors='ignore')
            if "5" not in stdout_or_json: return False, "Page count not 5", "Metadata error"'''

content = re.sub(
    r'            if "5" not in stdout_or_json: return False, "Page count not 5", "Metadata error"',
    new_check,
    content
)

new_check_bookmarks = '''        elif tool_id == "pdf_bookmarks":
            if not stdout_or_json: return False, "No output", "No output"
            if isinstance(stdout_or_json, bytes):
                stdout_or_json = stdout_or_json.decode('utf-8', errors='ignore')
            return True, "Bookmarks checked", None'''
content = re.sub(
    r'        elif tool_id == "pdf_bookmarks":\n            if not stdout_or_json: return False, "No output", "No output"\n            return True, "Bookmarks checked", None',
    new_check_bookmarks,
    content
)

new_check_classify = '''        elif tool_id == "pdf_classify_type":
            if not stdout_or_json: return False, "No output", "No output"
            if isinstance(stdout_or_json, bytes):
                stdout_or_json = stdout_or_json.decode('utf-8', errors='ignore')
            if "text" not in stdout_or_json.lower(): return False, "Expected text class", "Class mismatch"
            return True, "Classified as text", None'''
content = re.sub(
    r'        elif tool_id == "pdf_classify_type":\n            if not stdout_or_json: return False, "No output", "No output"\n            if "text" not in stdout_or_json\.lower\(\): return False, "Expected text class", "Class mismatch"\n            return True, "Classified as text", None',
    new_check_classify,
    content
)

new_check_validate = '''        elif tool_id == "pdf_validate":
            if not stdout_or_json: return False, "No output", "No output"
            if isinstance(stdout_or_json, bytes):
                stdout_or_json = stdout_or_json.decode('utf-8', errors='ignore')
            if "true" not in stdout_or_json.lower() and "valid" not in stdout_or_json.lower():
                return False, "Not valid", "Validation failed"
            return True, "Validation passed", None'''
content = re.sub(
    r'        elif tool_id == "pdf_validate":\n            if not stdout_or_json: return False, "No output", "No output"\n            if "true" not in stdout_or_json\.lower\(\) and "valid" not in stdout_or_json\.lower\(\): \n                return False, "Not valid", "Validation failed"\n            return True, "Validation passed", None',
    new_check_validate,
    content
)

new_check_hash = '''        elif tool_id == "pdf_hash":
            if not stdout_or_json: return False, "No output", "No output"
            if isinstance(stdout_or_json, bytes):
                stdout_or_json = stdout_or_json.decode('utf-8', errors='ignore')
            import hashlib
            h = hashlib.sha256(open("tests/e2e_fixtures/real/multi_page.pdf", "rb").read()).hexdigest()
            if h not in stdout_or_json: return False, f"Expected {h}", "Hash mismatch"
            return True, "Hash matches expected", None'''
content = re.sub(
    r'        elif tool_id == "pdf_hash":\n            if not stdout_or_json: return False, "No output", "No output"\n            import hashlib\n            h = hashlib\.sha256\(open\("tests/e2e_fixtures/real/multi_page\.pdf", "rb"\)\.read\(\)\)\.hexdigest\(\)\n            if h not in stdout_or_json: return False, f"Expected \{h\}", "Hash mismatch"\n            return True, "Hash matches expected", None',
    new_check_hash,
    content
)

new_check_search = '''        elif tool_id == "pdf_search":
            if not stdout_or_json: return False, "No output", "No output"
            if isinstance(stdout_or_json, bytes):
                stdout_or_json = stdout_or_json.decode('utf-8', errors='ignore')
            if "PAGE_TEXT_P3" not in stdout_or_json: return False, "Hit not found", "No hit"
            return True, "Search hit returned successfully", None'''
content = re.sub(
    r'        elif tool_id == "pdf_search":\n            if not stdout_or_json: return False, "No output", "No output"\n            if "PAGE_TEXT_P3" not in stdout_or_json: return False, "Hit not found", "No hit"\n            return True, "Search hit returned successfully", None',
    new_check_search,
    content
)

new_check_extract_text = '''        elif tool_id == "pdf_extract_text":
            if not stdout_or_json: return False, "No output", "No output"
            if isinstance(stdout_or_json, bytes):
                stdout_or_json = stdout_or_json.decode('utf-8', errors='ignore')
            if "Invoice 4821" not in stdout_or_json or "Total $1,250.00" not in stdout_or_json:
                return False, "Missing required strings", "Text mismatch"
            return True, "Text extracted successfully", None'''
content = re.sub(
    r'        elif tool_id == "pdf_extract_text":\n            if not stdout_or_json: return False, "No output", "No output"\n            if "Invoice 4821" not in stdout_or_json or "Total \$1,250\.00" not in stdout_or_json: \n                return False, "Missing required strings", "Text mismatch"\n            return True, "Text extracted successfully", None',
    new_check_extract_text,
    content
)

new_check_read_form = '''        elif tool_id == "pdf_read_form":
            if not stdout_or_json: return False, "No output", "No output"
            if isinstance(stdout_or_json, bytes):
                stdout_or_json = stdout_or_json.decode('utf-8', errors='ignore')
            try:
                data = json.loads(stdout_or_json)
                fields = str(data)
                if "first_name" not in fields: return False, "first_name field missing", "Field missing"
                return True, "Form fields read successfully", None
            except Exception as e:
                if "first_name" in stdout_or_json: return True, "Form fields read successfully", None
                return False, "Failed to parse json", "Parse error"'''
content = re.sub(
    r'        elif tool_id == "pdf_read_form":\n            if not stdout_or_json: return False, "No output", "No output"\n            try:\n                data = json\.loads\(stdout_or_json\)\n                fields = str\(data\)\n                if "first_name" not in fields: return False, "first_name field missing", "Field missing"\n                return True, "Form fields read successfully", None\n            except Exception as e:\n                if "first_name" in stdout_or_json: return True, "Form fields read successfully", None\n                return False, "Failed to parse json", "Parse error"',
    new_check_read_form,
    content
)


with open('scripts/test_tri_interface_e2e.py', 'w') as f:
    f.write(content)
