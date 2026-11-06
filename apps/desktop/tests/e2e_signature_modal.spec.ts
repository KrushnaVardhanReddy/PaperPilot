import { test, expect } from '@playwright/test';
import * as path from 'path';

test.describe('Spec 152: Interactive Visual Signature & Stamp Placement System', () => {

  test.beforeEach(async ({ page }) => {
    // Intercept and mock Tauri IPC invoke
    await page.addInitScript(() => {
      localStorage.setItem('paperpilot_onboarding_completed', 'true');
      (window as any).__last_tool_invocations = [];
      (window as any).__TAURI_INTERNALS__ = {
        transformCallback: (cb: any) => 1,
        invoke: async (cmd: string, args: any) => {
          if (cmd === 'invoke_mcp_tool') {
            (window as any).__last_tool_invocations.push({ cmd, args });
            return {
              success: true,
              message: `${args?.toolName || 'tool'} executed successfully.`,
              output_path: args?.arguments?.output || 'output.pdf'
            };
          }
          if (cmd === 'read_file_bytes') {
            return [];
          }
          if (cmd === 'is_portable_mode_cmd') return false;
          if (cmd === 'get_feature_packs') return [];
          return { success: true };
        }
      };
    });
  });

  test('Flow: Click ✍ Sign / Annotate, customize typed cursive signature, adopt, stamp on page, and apply with pdf_stamp', async ({ page }) => {
    await page.goto('/');

    // 1. Upload sample PDF fixture
    const fileInput = page.locator('input[multiple][type="file"]');
    await fileInput.setInputFiles(path.resolve('../../tests/e2e_fixtures/single_page.pdf'));

    // Select document
    const docName = page.locator('#tab-doc-0');
    await expect(docName).toBeVisible({ timeout: 10000 });
    await docName.click({ force: true });
    await page.evaluate(() => { window.dispatchEvent(new CustomEvent('test-select-doc', { detail: 0 })); });

    // Wait for PDF canvas to render
    const canvas = page.locator('.viewer-container canvas');
    await expect(canvas).toBeVisible({ timeout: 10000 });

    // 2. Click floating pill ✍ button
    const signBtn = page.locator('#btn-sign-annotate');
    await expect(signBtn).toBeVisible();
    await signBtn.click();

    // 3. Verify SignatureModal opens
    const modalTitle = page.locator('#sig-modal-title');
    await expect(modalTitle).toBeVisible();
    await expect(modalTitle).toHaveText('Create & Adopt Signature');

    // Verify tabs are available
    const typeTab = page.getByRole('button', { name: /Type/i });
    const drawTab = page.getByRole('button', { name: /Draw/i });
    const uploadTab = page.getByRole('button', { name: /Upload/i });
    await expect(typeTab).toBeVisible();
    await expect(drawTab).toBeVisible();
    await expect(uploadTab).toBeVisible();

    // 4. Type name and select cursive style
    const nameInput = page.locator('#sig-full-name-input');
    await expect(nameInput).toBeVisible();
    await nameInput.fill('Sarah Jenkins');

    // Pick blue ink color
    const blueColorBtn = page.getByRole('button', { name: 'Blue' });
    await blueColorBtn.click();
    await expect(blueColorBtn).toHaveClass(/selected/);

    // Pick Dancing Script font card
    const dancingCard = page.locator('.font-card', { hasText: 'Dancing Script' });
    await dancingCard.click();
    await expect(dancingCard).toHaveClass(/selected/);

    // 5. Click "Adopt & Sign"
    const adoptBtn = page.getByRole('button', { name: /Adopt & Sign/i });
    await expect(adoptBtn).toBeEnabled();
    await adoptBtn.click();

    // Modal should close
    await expect(modalTitle).not.toBeVisible();

    // 6. Verify stamp placement layer is active and stamp on page
    const stampLayer = page.locator('.signature-stamp-layer');
    await expect(stampLayer).toBeVisible();
    await expect(stampLayer).toHaveClass(/placing/);

    // Click on page to place the stamp at coordinates (200, 250)
    await stampLayer.click({ position: { x: 200, y: 250 } });

    // 7. Verify stamp is placed on document
    const placedStamp = page.locator('.placed-stamp-box');
    await expect(placedStamp).toBeVisible();
    await expect(placedStamp).toHaveClass(/selected/);

    // Verify move badge and delete button are visible on selected stamp
    const moveBadge = page.locator('.stamp-drag-badge');
    await expect(moveBadge).toBeVisible();
    const deleteBtn = page.locator('.stamp-delete-btn');
    await expect(deleteBtn).toBeVisible();

    // 8. Verify floating action bar appears with "Apply to Document"
    const actionBar = page.locator('.signature-action-bar');
    await expect(actionBar).toBeVisible();
    await expect(actionBar).toContainText('signature placed on Page 1');

    const applyBtn = page.locator('.btn-apply-stamps');
    await expect(applyBtn).toBeVisible();

    // 9. Click "Apply to Document" to bake signature with pdf_stamp
    await applyBtn.click();

    // Verify IPC call to pdf_stamp was dispatched
    const invocations = await page.evaluate(() => (window as any).__last_tool_invocations);
    expect(invocations.length).toBeGreaterThan(0);
    const stampInvocation = invocations.find((inv: any) => inv.args?.toolName === 'pdf_stamp');
    expect(stampInvocation).toBeTruthy();
    expect(stampInvocation.args.arguments.image_data).toContain('data:image/png');
    expect(stampInvocation.args.arguments.pages).toBe('1');

    // Verify toast notification appears
    const toast = page.locator('.toast.toast-success');
    await expect(toast).toBeVisible();
    await expect(toast).toContainText('Signature successfully applied');
  });

  test('Flow: Re-opening ✍ button with cached signatures shows popover menu', async ({ page }) => {
    // Seed localStorage with a saved signature
    await page.addInitScript(() => {
      const dummySig = {
        id: 'sig_test_123',
        dataUrl: 'data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==',
        width: 150,
        height: 50,
        label: 'Signature (John Doe)',
        type: 'type',
        isInitials: false,
        createdAt: Date.now()
      };
      localStorage.setItem('paperpilot:saved_signatures', JSON.stringify([dummySig]));
    });

    await page.goto('/');

    const fileInput = page.locator('input[multiple][type="file"]');
    await fileInput.setInputFiles(path.resolve('../../tests/e2e_fixtures/single_page.pdf'));

    const docName = page.locator('#tab-doc-0');
    await expect(docName).toBeVisible({ timeout: 10000 });
    await docName.click({ force: true });
    await page.evaluate(() => { window.dispatchEvent(new CustomEvent('test-select-doc', { detail: 0 })); });

    await page.waitForSelector('.viewer-container canvas');

    // Click ✍ button
    const signBtn = page.locator('#btn-sign-annotate');
    await signBtn.click();

    // Since signature is cached, it should open the signature popover menu
    const popover = page.locator('.signature-menu-popover');
    await expect(popover).toBeVisible();
    await expect(page.locator('.sig-menu-item', { hasText: 'Place Signature' })).toBeVisible();
    await expect(page.locator('.sig-menu-item', { hasText: 'Place Initials' })).toBeVisible();
    await expect(page.locator('.sig-menu-item', { hasText: 'Freehand Pen' })).toBeVisible();
    await expect(page.locator('.sig-menu-item', { hasText: 'Create New Signature...' })).toBeVisible();

    // Click "Place Signature"
    await page.locator('.sig-menu-item', { hasText: 'Place Signature' }).click();

    // Popover should close and layer should enter placement mode
    await expect(popover).not.toBeVisible();
    const stampLayer = page.locator('.signature-stamp-layer');
    await expect(stampLayer).toHaveClass(/placing/);
  });
});
