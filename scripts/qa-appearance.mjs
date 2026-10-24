import { chromium } from "@playwright/test";
import assert from "node:assert/strict";
import fs from "node:fs";
const browser = await chromium.launch({ channel: "chrome", headless: true });
const page = await browser.newPage({
  colorScheme: "dark",
  viewport: { width: 1440, height: 1000 },
});
await page.route("**/appearance-regression", (route) =>
  route.fulfill({
    contentType: "text/html",
    body: "<html><body></body></html>",
  }),
);
await page.goto("http://127.0.0.1:1420/appearance-regression");
const checks = await page.evaluate(async () => {
  const { mockIPC } = await import("/node_modules/@tauri-apps/api/mocks.js");
  const { emit } = await import("/node_modules/@tauri-apps/api/event.js");
  let persisted = { theme: "light", accent_color: "blue" };
  let pending = null;
  mockIPC(
    (cmd) =>
      cmd === "get_app_settings"
        ? (pending ?? Promise.resolve({ ...persisted }))
        : null,
    { shouldMockEvents: true },
  );
  localStorage.setItem("handy.theme", "dark");
  localStorage.setItem("vozprompt.accent", "pink");
  const theme = await import("/src/lib/utils/theme.ts");
  await theme.initializeAppearance();
  const initial = { ...document.documentElement.dataset };
  let release;
  pending = new Promise((resolve) => {
    release = resolve;
  });
  const sync = theme.syncThemeFromSettings();
  await emit("theme-changed", "dark");
  await emit("accent-color-changed", "green");
  release({ theme: "light", accent_color: "pink" });
  await sync;
  const race = { ...document.documentElement.dataset };
  pending = null;
  await theme.syncThemeFromSettings();
  const restored = { ...document.documentElement.dataset };
  return {
    initial,
    race,
    restored,
    cache: [
      localStorage.getItem("handy.theme"),
      localStorage.getItem("vozprompt.accent"),
    ],
  };
});
assert.equal(checks.initial.theme, "light");
assert.equal(checks.initial.accent, "blue");
assert.equal(checks.race.theme, "dark");
assert.equal(checks.race.accent, "green");
assert.equal(checks.restored.theme, "light");
assert.equal(checks.restored.accent, "blue");
assert.deepEqual(checks.cache, ["light", "blue"]);
console.log(
  "PASS: saved settings override stale cache; events win over stale reads; resync restores saved appearance.",
);
await page.goto("http://127.0.0.1:1420/design-preview.html");
await page.locator(".accent-options button").first().waitFor();
await page.locator(".accent-options button").nth(6).click();
await page.locator(".theme-switch button").first().click();
await page.waitForTimeout(350);
assert.equal(await page.locator("html").getAttribute("data-theme"), "light");
assert.equal(await page.locator("html").getAttribute("data-accent"), "blue");
fs.mkdirSync("output/playwright", { recursive: true });
await page.screenshot({
  path: "output/playwright/appearance-fixed-desktop.png",
});
await page.setViewportSize({ width: 390, height: 844 });
await page.screenshot({
  path: "output/playwright/appearance-fixed-mobile.png",
});
console.log("PASS: desktop and mobile light/blue under dark OS.");
const overlay = await browser.newPage({
  colorScheme: "dark",
  viewport: { width: 400, height: 120 },
});
await overlay.route("**/appearance-overlay-test", (route) =>
  route.fulfill({
    contentType: "text/html",
    body: '<html><body><div id="root"></div></body></html>',
  }),
);
await overlay.goto("http://127.0.0.1:1420/appearance-overlay-test");
await overlay.evaluate(async () => {
  const { mockIPC, mockWindows } = await import(
    "/node_modules/@tauri-apps/api/mocks.js"
  );
  window.__appearanceSettings = {
    theme: "dark",
    accent_color: "pink",
    app_language: "pt",
    overlay_position: "bottom",
  };
  mockWindows("recording_overlay");
  mockIPC(
    (cmd) =>
      cmd === "get_app_settings"
        ? { ...window.__appearanceSettings }
        : cmd === "plugin:os|locale"
          ? "pt-BR"
          : null,
    { shouldMockEvents: true },
  );
  const refresh = await import("/@react-refresh");
  refresh.default.injectIntoGlobalHook(window);
  window.$RefreshReg$ = () => {};
  window.$RefreshSig$ = () => (type) => type;
  window.__vite_plugin_react_preamble_installed__ = true;
  await import("/src/overlay/main.tsx");
});
await overlay.waitForTimeout(500);
await overlay.evaluate(async () => {
  // No appearance event: simulate a sleeping webview missing the change.
  window.__appearanceSettings = {
    ...window.__appearanceSettings,
    theme: "light",
    accent_color: "blue",
  };
  const { emit } = await import("/node_modules/@tauri-apps/api/event.js");
  await emit("show-overlay", "recording");
});
await overlay.waitForFunction(
  () =>
    document.documentElement.dataset.theme === "light" &&
    document.documentElement.dataset.accent === "blue",
);
await overlay.screenshot({
  path: "output/playwright/appearance-overlay-recovered.png",
});
console.log(
  "PASS: reopening a stale dark/pink overlay restores light/blue without an appearance event.",
);
await browser.close();
