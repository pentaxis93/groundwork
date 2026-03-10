#!/usr/bin/env python3
from __future__ import annotations

import io
import shutil
import sys
from pathlib import Path

try:
    import cairosvg
    from PIL import Image, ImageDraw, ImageFont
except ImportError:
    print(
        "Missing dependencies. Install with:\n"
        "  python3 -m venv /tmp/groundwork-logo-venv\n"
        "  /tmp/groundwork-logo-venv/bin/python -m pip install cairosvg Pillow",
        file=sys.stderr,
    )
    raise SystemExit(1)


SCRIPT_DIR = Path(__file__).resolve().parent
MARK_SVG = SCRIPT_DIR / "groundwork-mark.svg"
FULL_SVG = SCRIPT_DIR / "groundwork-full.svg"
DIST_DIR = SCRIPT_DIR / "dist"

BACKGROUND = "#2A241C"
TITLE_COLOR = "#BDA06A"
TAGLINE_COLOR = "#9B7A3C"

EXPECTED_OUTPUTS = [
    "favicon.ico",
    "favicon.svg",
    "apple-touch-icon.png",
    "logo-32.png",
    "logo-64.png",
    "logo-128.png",
    "logo-256.png",
    "logo-512.png",
    "github-social-preview.png",
    "og-image.png",
    "avatar-500.png",
]


def require_sources() -> None:
    missing = [str(path) for path in (MARK_SVG, FULL_SVG) if not path.exists()]
    if missing:
        raise FileNotFoundError(f"Missing source files: {', '.join(missing)}")


def svg_to_png_bytes(svg_path: Path, width: int, height: int) -> bytes:
    return cairosvg.svg2png(url=str(svg_path), output_width=width, output_height=height)


def render_mark_image(size: int) -> Image.Image:
    raw = svg_to_png_bytes(MARK_SVG, size, size)
    return Image.open(io.BytesIO(raw)).convert("RGBA")


def save_mark_png(filename: str, size: int) -> None:
    target = DIST_DIR / filename
    target.write_bytes(svg_to_png_bytes(MARK_SVG, size, size))


def text_height(draw: ImageDraw.ImageDraw, text: str, font: ImageFont.ImageFont) -> int:
    box = draw.textbbox((0, 0), text, font=font)
    return box[3] - box[1]


def text_center_x(draw: ImageDraw.ImageDraw, canvas_width: int, text: str, font: ImageFont.ImageFont) -> int:
    box = draw.textbbox((0, 0), text, font=font)
    text_width = box[2] - box[0]
    return (canvas_width - text_width) // 2


def load_font(size: int, bold: bool = False) -> ImageFont.ImageFont:
    candidates = (
        ["/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf", "/usr/share/fonts/dejavu/DejaVuSans-Bold.ttf"]
        if bold
        else ["/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf", "/usr/share/fonts/dejavu/DejaVuSans.ttf"]
    )
    for candidate in candidates:
        path = Path(candidate)
        if path.exists():
            return ImageFont.truetype(str(path), size=size)
    return ImageFont.load_default()


def save_social_like(filename: str, width: int, height: int) -> None:
    image = Image.new("RGB", (width, height), BACKGROUND)
    draw = ImageDraw.Draw(image)

    title = "groundwork"
    tagline = "agentic development methodology"

    title_font = load_font(max(48, int(height * 0.115)), bold=True)
    tagline_font = load_font(max(24, int(height * 0.05)), bold=False)

    mark_size = int(min(width, height) * 0.24)
    gap_mark_title = int(height * 0.07)
    gap_title_tagline = int(height * 0.03)

    title_h = text_height(draw, title, title_font)
    tagline_h = text_height(draw, tagline, tagline_font)
    block_height = mark_size + gap_mark_title + title_h + gap_title_tagline + tagline_h
    top = (height - block_height) // 2

    mark = render_mark_image(mark_size)
    mark_x = (width - mark_size) // 2
    image.paste(mark, (mark_x, top), mark)

    title_y = top + mark_size + gap_mark_title
    draw.text(
        (text_center_x(draw, width, title, title_font), title_y),
        title,
        font=title_font,
        fill=TITLE_COLOR,
    )

    tagline_y = title_y + title_h + gap_title_tagline
    draw.text(
        (text_center_x(draw, width, tagline, tagline_font), tagline_y),
        tagline,
        font=tagline_font,
        fill=TAGLINE_COLOR,
    )

    image.save(DIST_DIR / filename, format="PNG")


def save_avatar() -> None:
    size = 500
    image = Image.new("RGB", (size, size), BACKGROUND)
    mark_size = 280
    mark = render_mark_image(mark_size)
    mark_x = (size - mark_size) // 2
    mark_y = (size - mark_size) // 2
    image.paste(mark, (mark_x, mark_y), mark)
    image.save(DIST_DIR / "avatar-500.png", format="PNG")


def save_favicon_ico() -> None:
    icon = render_mark_image(32)
    icon.save(DIST_DIR / "favicon.ico", format="ICO", sizes=[(32, 32)])


def main() -> int:
    require_sources()
    DIST_DIR.mkdir(parents=True, exist_ok=True)

    shutil.copyfile(MARK_SVG, DIST_DIR / "favicon.svg")
    save_favicon_ico()

    save_mark_png("apple-touch-icon.png", 180)
    for size in (32, 64, 128, 256, 512):
        save_mark_png(f"logo-{size}.png", size)

    save_social_like("github-social-preview.png", 1280, 640)
    save_social_like("og-image.png", 1200, 630)
    save_avatar()

    missing = [name for name in EXPECTED_OUTPUTS if not (DIST_DIR / name).exists()]
    if missing:
        raise RuntimeError(f"Generation incomplete. Missing: {', '.join(missing)}")

    print("Generated assets:")
    for name in EXPECTED_OUTPUTS:
        print(f"- {DIST_DIR / name}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
