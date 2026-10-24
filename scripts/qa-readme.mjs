import fs from "node:fs";
import path from "node:path";
import { unified } from "unified";
import remarkParse from "remark-parse";
import remarkRehype from "remark-rehype";
import { chromium } from "@playwright/test";
const esc = (s) =>
  String(s)
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;");
const render = (node) =>
  node.type === "text"
    ? esc(node.value)
    : node.type === "raw"
      ? node.value
      : node.type === "root"
        ? node.children.map(render).join("")
        : node.type === "element"
          ? `<${node.tagName}${Object.entries(node.properties || {})
              .map(([k, v]) => ` ${k}="${esc(v)}"`)
              .join(
                "",
              )}>${(node.children || []).map(render).join("")}</${node.tagName}>`
          : "";
const parser = unified()
  .use(remarkParse)
  .use(remarkRehype, { allowDangerousHtml: true });
const html = (s) => render(parser.runSync(parser.parse(s)));
const cell = (s) => html(s.trim()).replace(/^<p>|<\/p>$/g, "");
let markdown = fs.readFileSync("README.md", "utf8");
markdown = markdown.replace(
  /(^\|.+\|\r?\n\|[ :|\-]+\|\r?\n(?:\|.*\|(?:\r?\n|$))+)/gm,
  (table) => {
    const rows = table
      .trim()
      .split(/\r?\n/)
      .filter((_, i) => i !== 1);
    return (
      '<div class="table-wrap"><table>' +
      rows
        .map(
          (row, i) =>
            "<tr>" +
            row
              .slice(1, -1)
              .split("|")
              .map((c) => `<${i ? "td" : "th"}>${cell(c)}</${i ? "td" : "th"}>`)
              .join("") +
            "</tr>",
        )
        .join("") +
      "</table></div>\n"
    );
  },
);
fs.mkdirSync("output/readme-review", { recursive: true });
fs.writeFileSync(
  "output/readme-review/index.html",
  `<!doctype html><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><base href="/"><style>body{margin:0;font:16px/1.6 -apple-system,BlinkMacSystemFont,Segoe UI,sans-serif;color:#1f2328;background:white}main{max-width:980px;margin:40px auto;padding:32px;border:1px solid #d1d9e0;border-radius:8px}h1,h2{border-bottom:1px solid #d1d9e0;padding-bottom:10px;line-height:1.3}h2{margin-top:36px}img{max-width:100%;height:auto}a{color:#0969da;text-decoration:none}blockquote{border-left:4px solid #d1d9e0;color:#59636e;margin-left:0;padding-left:16px}pre{background:#f6f8fa;padding:16px;overflow:auto;border-radius:6px;font-size:13px}code{background:#f6f8fa;border-radius:4px;padding:2px 4px}table{border-collapse:collapse;width:100%;margin:16px 0}td,th{border:1px solid #d1d9e0;padding:10px;text-align:left}th{background:#f6f8fa}.table-wrap{overflow:auto}@media(max-width:600px){main{margin:0;padding:18px;border:0}body{font-size:14px}}</style><main>${html(markdown)}</main>`,
);
const browser = await chromium.launch({ channel: "chrome", headless: true });
const page = await browser.newPage({ viewport: { width: 1200, height: 900 } });
await page.goto("http://127.0.0.1:1420/output/readme-review/index.html");
await page.locator("img").last().waitFor();
const missing = await page
  .locator("img")
  .evaluateAll((imgs) =>
    imgs.filter((i) => !i.complete || !i.naturalWidth).map((i) => i.src),
  );
if (missing.length) throw Error("Missing images: " + missing.join(","));
await page.screenshot({
  path: "output/readme-review/desktop.png",
  fullPage: true,
});
await page.setViewportSize({ width: 390, height: 844 });
await page.screenshot({
  path: "output/readme-review/mobile.png",
  fullPage: true,
});
const overflow = await page.evaluate(
  () => document.documentElement.scrollWidth > innerWidth,
);
if (overflow) throw Error("Mobile overflow");
console.log(
  "README render passed: images load, desktop/mobile captured, no mobile overflow.",
);
await browser.close();
