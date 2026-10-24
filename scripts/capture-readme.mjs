import { chromium } from "@playwright/test";
import fs from "node:fs";
const browser = await chromium.launch({ channel: "chrome", headless: true });
const context = await browser.newContext({
  viewport: { width: 1100, height: 780 },
  deviceScaleFactor: 1,
  reducedMotion: "reduce",
});
const page = await context.newPage();
await page.goto("http://localhost:1420/design-preview.html");
await page.locator(".accent-options button").first().waitFor();
await page.locator(".theme-switch button").first().click();
await page.locator(".accent-options button").nth(6).click();
await page.locator(".voice-intro").scrollIntoViewIfNeeded();
await page.screenshot({ path: "docs/media/app-light.png" });
await page.locator(".theme-switch button").nth(1).click();
await page.locator(".voice-intro").scrollIntoViewIfNeeded();
await page.screenshot({ path: "docs/media/app-dark.png" });
await page.locator(".appearance-panel").scrollIntoViewIfNeeded();
await page.screenshot({ path: "docs/media/appearance-dark.png" });
// Capture a reproducible UI sequence; FFmpeg assembles these frames into a GIF.
await page.setViewportSize({ width: 960, height: 700 });
fs.mkdirSync("output/readme-frames", { recursive: true });
let index = 0;
for (const [theme, color] of [
  [0, 0],
  [1, 0],
  [1, 6],
  [0, 6],
  [0, 4],
  [1, 8],
]) {
  await page.locator(".theme-switch button").nth(theme).click();
  await page.locator(".accent-options button").nth(color).click();
  await page.locator(".appearance-panel").scrollIntoViewIfNeeded();
  await page.waitForTimeout(250);
  await page.screenshot({
    path: `output/readme-frames/${String(index++).padStart(2, "0")}.png`,
  });
}
await browser.close();
console.log(
  "Captured light/dark screenshots and appearance demo frames using synthetic preview data.",
);
