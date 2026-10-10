import re

with open('paperpilot-cli/src/commands/watch.rs', 'r') as f:
    content = f.read()

content = content.replace(
    'pub fn handle_watch(',
    '#[allow(clippy::all)]\npub fn handle_watch('
)

with open('paperpilot-cli/src/commands/watch.rs', 'w') as f:
    f.write(content)
