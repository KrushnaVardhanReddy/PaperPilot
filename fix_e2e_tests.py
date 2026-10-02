import os
import glob

# The OperationsPanel changes added a new input[type="file"] to the DOM for the conversion tools.
# This breaks the e2e tests which assume there is only 1 input[type="file"] (or two, but they were using a generic locator).
# The error says "resolved to 2 elements: 1) getByLabel('+'), 2) getByLabel('Browse Files')"
# However, now there's another one: `id="inputFile"`.
# We need to change `page.locator('input[type="file"]')` to `page.locator('input[type="file"]').first()` in tests to grab the main upload zone one.

test_files = glob.glob('apps/desktop/tests/*.spec.ts')

for file in test_files:
    with open(file, 'r') as f:
        content = f.read()

    # We replace strict locators with first() or a specific class/id.
    content = content.replace("page.locator('input[type=\"file\"]')", "page.locator('input[type=\"file\"]').first()")

    with open(file, 'w') as f:
        f.write(content)
