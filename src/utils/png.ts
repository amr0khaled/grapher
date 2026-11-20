import { LatexImage } from "../types";
import katex from "katex";
import katexCss from "katex/dist/katex.min.css?inline";

let cssCache: Promise<string> | null = null;


async function toDataUrl(url: string): Promise<string> {
  const blob = await (await fetch(url)).blob();
  const r = new FileReader();
  return new Promise((resolve, reject) => {
    r.onload = () => resolve(r.result as string);
    r.onerror = reject;
    r.readAsDataURL(blob);
  });
}

function loadImage(src: string): Promise<HTMLImageElement> {
  return new Promise((resolve, reject) => {
    const img = new Image();
    img.onload = () => resolve(img);
    img.onerror = () => reject(new Error("SVG failed to load as image"));
    img.src = src;
  });
}


function getKatexCss(): Promise<string> {
  cssCache ??= (async () => {
    let css = katexCss.replace(
      /,\s*url\([^)]*\)\s*format\(["'](?:woff|truetype)["']\)/g,
      ""
    );

    const urlRe = /url\(["']?([^"')]+)["']?\)/g;
    const urls = [...new Set([...css.matchAll(urlRe)].map((m) => m[1]))];

    await Promise.all(
      urls.map(async (u) => {
        if (u.startsWith("data:")) return;
        try {
          const data = await toDataUrl(new URL(u, document.baseURI).href);
          css = css.split(u).join(data);
        } catch (e) {
          console.warn("Could not inline font", u, e);
        }
      })
    );
    return css;
  })();
  return cssCache;
}

export async function latexToImage(
  latex: string,
  color: string,
  fontPx = 16,
  maxWidth = Infinity
): Promise<LatexImage> {
  // Drop MathLive placeholders, which KaTeX can't parse
  const clean = latex.replace(/\\placeholder(\[[^\]]*\])?\{[^}]*\}/g, "");

  const markup = katex.renderToString(clean, {
    throwOnError: false,
    output: "html", // skip the hidden MathML copy
  });

  const css = await getKatexCss();

  // Measure in the real DOM, with the same CSS applied
  const host = document.createElement("div");
  host.style.cssText = `position:absolute;left:-9999px;top:0;visibility:hidden;white-space:nowrap;font-size:${fontPx}px;line-height:normal;`;
  host.innerHTML = `<style>${css}</style>${markup}`;
  document.body.appendChild(host);
  await document.fonts.ready;

  const target = host.querySelector(".katex") as HTMLElement;
  const rect = target.getBoundingClientRect();
  const w = Math.ceil(rect.width) + 4;
  const h = Math.ceil(rect.height) + 4;
  const inner = new XMLSerializer().serializeToString(target);
  host.remove();

  const svg =
    `<svg xmlns="http://www.w3.org/2000/svg" width="${w}" height="${h}">` +
    `<style><![CDATA[${css}]]></style>` +
    `<foreignObject width="100%" height="100%">` +
    `<div xmlns="http://www.w3.org/1999/xhtml" style="font-size:${fontPx}px;color:${color};white-space:nowrap;line-height:normal;padding:2px;">${inner}</div>` +
    `</foreignObject></svg>`;

  const img = await loadImage(
    "data:image/svg+xml;charset=utf-8," + encodeURIComponent(svg)
  );

  const k = w > maxWidth ? maxWidth / w : 1;
  return { img, width: w * k, height: h * k };
}
