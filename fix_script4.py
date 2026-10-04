import sys

with open("scripts/test_tri_interface_e2e.py", "r") as f:
    content = f.read()

# Fix gateway starting
content = content.replace("    # gateway.start()", "    gateway.start()")
content = content.replace("    # gateway.stop()", "    gateway.stop()")

# Fix report writing append bug
content = content.replace('    with open(REPORT_PATH, "a" if os.path.exists(REPORT_PATH) else "w") as f:\n        f.seek(0, 2)\n        if f.tell() == 0:\n             f.write("\\n".join(report_content) + "\\n\\n" + "\\n".join(detailed_report) + "\\n")\n        else:\n             f.write("\\n\\n" + "\\n".join(detailed_report) + "\\n")',
                          '    with open(REPORT_PATH, "w") as f:\n        f.write("\\n".join(report_content) + "\\n\\n" + "\\n".join(detailed_report) + "\\n")')

with open("scripts/test_tri_interface_e2e.py", "w") as f:
    f.write(content)
