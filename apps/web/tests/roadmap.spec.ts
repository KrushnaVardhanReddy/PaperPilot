import { test, expect } from '@playwright/test';
import type { RoadmapDocument } from '../src/lib/roadmap';

test.beforeEach(async ({ page }) => {
  // Exercise the existing badge's cache path; roadmap data always comes from the production build.
  await page.addInitScript(() => {
    sessionStorage.setItem('gh_dls_KrushnaVardhanReddy/PaperPilot', JSON.stringify({ count: 1250, tag: 'test-cache', timestamp: Date.now() }));
  });
});

test('production roadmap matches generated milestones and renders all three columns', async ({ page }, testInfo) => {
  const errors: string[] = [];
  page.on('pageerror', error => errors.push(error.message));
  const data = await (await page.request.get('/roadmap.json')).json() as RoadmapDocument;
  expect(data.source_files.length).toBeGreaterThanOrEqual(13);
  await page.goto('/');
  await page.locator('.header-actions .roadmap-trigger').click();
  const dialog = page.getByRole('dialog', { name: 'PaperPilot Live Roadmap' });
  await expect(dialog).toBeVisible();
  await expect(page).toHaveURL(/\/roadmap$/);
  for (const status of ['shipped', 'in_progress', 'planned']) {
    const column = dialog.locator(`[data-status="${status}"]`);
    await expect(column.locator('.milestone-card')).toHaveCount(data.items.filter(item => item.status === status).length);
  }
  await expect(dialog.getByRole('searchbox')).toBeFocused();
  await expect(dialog).toContainText('78+');
  await dialog.screenshot({ path: testInfo.outputPath('roadmap-desktop.png') });
  expect(errors).toEqual([]);
});

test('search, category filters and clearing an empty result work together', async ({ page }) => {
  await page.goto('/roadmap');
  const dialog = page.getByRole('dialog');
  await expect(dialog.locator('.milestone-card').first()).toBeVisible();
  await dialog.getByRole('searchbox').fill('pdf_nup');
  await expect(dialog.locator('.milestone-card[data-task-id="5.3.2"]')).toBeVisible();
  await dialog.getByRole('button', { name: 'Security', exact: true }).click();
  await expect(dialog.getByRole('button', { name: 'Security', exact: true })).toHaveAttribute('aria-pressed', 'true');
  await dialog.getByRole('searchbox').fill('no-such-roadmap-milestone-zzzz');
  await expect(dialog.locator('.milestone-card')).toHaveCount(0);
  await expect(dialog.getByText('No matching milestones', { exact: true })).toHaveCount(3);
  await dialog.getByRole('button', { name: 'Clear filters' }).first().click();
  await expect(dialog.getByRole('searchbox')).toHaveValue('');
  await expect(dialog.getByRole('button', { name: 'All', exact: true })).toHaveAttribute('aria-pressed', 'true');
  await expect(dialog.locator('.milestone-card').first()).toBeVisible();
});

test('planned items link to the GitHub form with the selected task prefilled', async ({ page }) => {
  await page.goto('/roadmap');
  const card = page.locator('[data-status="planned"] .milestone-card').first();
  await expect(card).toBeVisible();
  const title = await card.locator('h4').innerText();
  const id = await card.getAttribute('data-task-id');
  const link = card.getByRole('link', { name: /^Vote or request/ });
  const url = new URL((await link.getAttribute('href'))!);
  expect(url.origin + url.pathname).toBe('https://github.com/KrushnaVardhanReddy/PaperPilot/issues/new');
  expect(url.searchParams.get('template')).toBe('feature_request.yml');
  expect(url.searchParams.get('title')).toBe(`[FEATURE]: ${title}`.slice(0, 240));
  expect(url.searchParams.get('problem')).toContain(id);
  expect(url.searchParams.get('proposed_solution')).toContain('phases/');
  await expect(link).toHaveAttribute('target', '_blank');
  await expect(link).toHaveAttribute('rel', 'noopener noreferrer');
});

test('native dialog contains keyboard focus and closes with Escape or the backdrop', async ({ page }) => {
  await page.goto('/');
  const trigger = page.locator('.header-actions .roadmap-trigger');
  await trigger.click();
  await expect(page.getByRole('dialog')).toBeVisible();
  const close = page.getByRole('button', { name: 'Close roadmap' });
  const lastLink = page.getByRole('dialog').getByRole('link', { name: 'Join the conversation' });
  await lastLink.focus();
  await page.keyboard.press('Tab');
  await expect(close).toBeFocused();
  await page.keyboard.press('Shift+Tab');
  await expect(lastLink).toBeFocused();
  await page.keyboard.press('Escape');
  await expect(page.getByRole('dialog')).toHaveCount(0);
  await expect(trigger).toBeFocused();
  await page.locator('.app-footer').getByRole('link', { name: 'Roadmap', exact: true }).click();
  await expect(page.getByRole('dialog')).toBeVisible();
  await page.mouse.click(2, 2);
  await expect(page.getByRole('dialog')).toHaveCount(0);
});

test('direct route, reload, back and forward navigation preserve roadmap state', async ({ page }) => {
  await page.goto('/roadmap');
  await expect(page.getByRole('dialog')).toBeVisible();
  await page.reload();
  await expect(page.getByRole('dialog')).toBeVisible();
  await page.getByRole('button', { name: 'Close roadmap' }).click();
  await expect(page).toHaveURL('http://127.0.0.1:4187/');
  await page.locator('.header-actions .roadmap-trigger').click();
  await page.goBack();
  await expect(page.getByRole('dialog')).toHaveCount(0);
  await page.goForward();
  await expect(page.getByRole('dialog')).toBeVisible();
});

test('a failed data request offers retry and recovers from the actual static asset', async ({ page }) => {
  await page.route('**/roadmap.json', route => route.abort());
  await page.goto('/roadmap');
  await expect(page.getByRole('alert')).toContainText('could not be loaded');
  await page.unroute('**/roadmap.json');
  await page.getByRole('button', { name: 'Try again' }).click();
  await expect(page.locator('.milestone-card').first()).toBeVisible();
  await expect(page.getByRole('alert')).toHaveCount(0);
});

test('mobile layout fits the viewport with usable search and stacked columns', async ({ page }, testInfo) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto('/roadmap');
  const dialog = page.getByRole('dialog');
  await expect(dialog.locator('.milestone-card').first()).toBeVisible();
  const bounds = await dialog.boundingBox();
  expect(bounds!.x).toBeGreaterThanOrEqual(0);
  expect(bounds!.width).toBeLessThanOrEqual(390);
  const shipped = await dialog.locator('[data-status="shipped"]').boundingBox();
  const active = await dialog.locator('[data-status="in_progress"]').boundingBox();
  expect(shipped!.width).toBeGreaterThan(bounds!.width * 0.8);
  expect(active!.y).toBeGreaterThan(shipped!.y);
  expect(await dialog.evaluate(element => element.scrollWidth <= element.clientWidth + 1)).toBe(true);
  const close = await dialog.getByRole('button', { name: 'Close roadmap' }).boundingBox();
  expect(close!.height).toBeGreaterThanOrEqual(44);
  await dialog.getByRole('searchbox').fill('semantic');
  await expect(dialog.locator('.milestone-card').first()).toBeVisible();
  await dialog.screenshot({ path: testInfo.outputPath('roadmap-mobile.png') });
});
