import os
import glob

# The "input[type='file']" first() matches the single-file hidden input for DropZone logic or some specific thing.
# We need the `input[type="file"][multiple]` for uploading MULTIPLE files, or just `.svelte-3a0yo8` (but that's brittle).
# Actually, the dropzone uses a hidden input. Let's just use `page.locator('input[multiple]')` which is definitely the DropZone file input capable of receiving arrays of files.

test_files = glob.glob('apps/desktop/tests/*.spec.ts')

for file in test_files:
    with open(file, 'r') as f:
        content = f.read()

    # Replace the previously modified lines
    content = content.replace("page.locator('input[type=\"file\"]').first()", "page.locator('input[multiple]')")

    with open(file, 'w') as f:
        f.write(content)
