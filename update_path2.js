const fs = require('fs');
let content = fs.readFileSync('apps/desktop/tests/e2e_ui.spec.ts', 'utf8');
content = content.replace("path.join(__dirname, '../../../tests/e2e_fixtures/single_page.pdf')", "path.resolve('../../tests/e2e_fixtures/single_page.pdf')");
fs.writeFileSync('apps/desktop/tests/e2e_ui.spec.ts', content);
