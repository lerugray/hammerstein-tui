#!/usr/bin/env python3
"""
Hammerstein TUI banner -- Renaissance-frontispiece, sibling to parent Hammerstein banner.

Layout (top -> bottom, all centered):
  - tagline (italic serif, cream): "Where you spend the day."
  - wordmark: [block-cursor] in Pompeiian red brackets, then "Hammerstein TUI" in cream italic
  - trio (small caps, muted gold): "AUDIT  -  DISPATCH  -  EXECUTE"

Renders 2x then downsamples for crisp text.
"""
import os
from PIL import Image, ImageDraw, ImageFont

# Output target
OUT = r"C:\Users\rweis\OneDrive\Documents\generalstaff-private\hammerstein-tui\docs\images\banner.png"

# Logical dimensions
W, H = 1500, 500
SCALE = 2
CW, CH = W * SCALE, H * SCALE

# Palette (matches parent Hammerstein banner family)
NAVY = (36, 59, 94, 255)        # ~#243B5E
CREAM = (245, 237, 216, 255)    # ~#F5EDD8
GOLD = (168, 150, 104, 255)     # ~#A89668
RED = (176, 70, 60, 255)        # ~#B0463C Pompeiian

# Fonts (Windows system; Palatino Linotype is the Renaissance/Aldine reference)
F_DIR = r"C:\Windows\Fonts"
F_ITALIC = os.path.join(F_DIR, "palai.ttf")    # Palatino Italic
F_BOLD = os.path.join(F_DIR, "palab.ttf")      # Palatino Bold (for bracketed monogram)
F_REG = os.path.join(F_DIR, "pala.ttf")        # Palatino Regular (for small caps trio)


def load(font_path, pt):
    """Load font at point size, scaled for retina."""
    return ImageFont.truetype(font_path, int(pt * SCALE))


def measure(draw, text, font):
    """Return (width, height, ascent_offset) for text rendered with font."""
    bbox = draw.textbbox((0, 0), text, font=font)
    return bbox[2] - bbox[0], bbox[3] - bbox[1], bbox[1]


def draw_letter_spaced(draw, text, font, x, y, fill, tracking_em=0.18):
    """Draw text with manual letter-spacing. tracking_em in font-em units."""
    ascent, _ = font.getmetrics()
    em = ascent  # close enough to em
    space = int(em * tracking_em)
    cur_x = x
    for ch in text:
        if ch == " ":
            cur_x += space * 3  # word space
            continue
        bbox = draw.textbbox((0, 0), ch, font=font)
        ch_w = bbox[2] - bbox[0]
        draw.text((cur_x, y), ch, font=font, fill=fill)
        cur_x += ch_w + space
    return cur_x - x  # total width


def measure_letter_spaced(draw, text, font, tracking_em=0.18):
    """Measure width of letter-spaced text without drawing."""
    ascent, _ = font.getmetrics()
    em = ascent
    space = int(em * tracking_em)
    total = 0
    for ch in text:
        if ch == " ":
            total += space * 3
            continue
        bbox = draw.textbbox((0, 0), ch, font=font)
        total += (bbox[2] - bbox[0]) + space
    return total - space  # trim trailing


def render():
    img = Image.new("RGBA", (CW, CH), NAVY)
    draw = ImageDraw.Draw(img)

    # === 1. Tagline (italic, cream, ~46pt) ===
    tagline = "Where you spend the day."
    f_tag = load(F_ITALIC, 46)
    tw, th, _ = measure(draw, tagline, f_tag)
    tagline_y = int(CH * 0.16)
    draw.text(((CW - tw) // 2, tagline_y), tagline, font=f_tag, fill=CREAM)

    # === 2. Wordmark: [block] + "Hammerstein TUI" ===
    # Bracketed monogram: drawn as "[ ]" in bold red + a filled red rectangle in the gap
    # Wordmark next to it in cream italic
    wordmark_text = "Hammerstein TUI"
    f_word = load(F_ITALIC, 108)
    f_brack = load(F_BOLD, 132)  # brackets slightly taller than wordmark for drop-cap presence

    ww, wh, w_off = measure(draw, wordmark_text, f_word)

    # Brackets: render "[" and "]" separately, place a filled cursor block between them
    f_brack_h = f_brack.getmetrics()[0]  # ascent
    lb_w, lb_h, _ = measure(draw, "[", f_brack)
    rb_w, _, _ = measure(draw, "]", f_brack)

    # Cursor block dimensions: roughly 0.5x cap-height wide, 0.7x cap-height tall
    cap_h_brack = lb_h
    cursor_w = int(cap_h_brack * 0.42)
    cursor_h = int(cap_h_brack * 0.60)

    # Total monogram width: [ + small gap + cursor + small gap + ]
    inner_pad = int(cap_h_brack * 0.08)
    monogram_w = lb_w + inner_pad + cursor_w + inner_pad + rb_w

    # Composition width: monogram + small overlap-ish gap + wordmark
    overlap = int(cap_h_brack * 0.05)  # right bracket gently kisses the H
    total_w = monogram_w + overlap + ww
    comp_x0 = (CW - total_w) // 2
    wordmark_y = int(CH * 0.42)  # baseline-ish anchor

    # Vertical alignment: align baselines. f_brack is slightly bigger so its top sits a bit higher.
    # Use baseline alignment: place wordmark first, then anchor brackets to same baseline.
    word_top = wordmark_y
    brack_top = wordmark_y - (f_brack.getmetrics()[0] - f_word.getmetrics()[0])

    # Draw "["
    x = comp_x0
    draw.text((x, brack_top), "[", font=f_brack, fill=RED)
    x += lb_w + inner_pad

    # Draw cursor block
    cursor_top = brack_top + (cap_h_brack - cursor_h) // 2 + int(cap_h_brack * 0.05)  # nudge down to optical center
    draw.rectangle([x, cursor_top, x + cursor_w, cursor_top + cursor_h], fill=RED)
    x += cursor_w + inner_pad

    # Draw "]"
    draw.text((x, brack_top), "]", font=f_brack, fill=RED)
    x += rb_w + overlap

    # Draw wordmark
    draw.text((x, word_top), wordmark_text, font=f_word, fill=CREAM)

    # === 3. Trio: small-caps gold, letter-spaced ===
    trio_text = "AUDIT  ·  DISPATCH  ·  EXECUTE"  # middle dots with breathing room
    f_trio = load(F_REG, 26)
    trio_w = measure_letter_spaced(draw, trio_text, f_trio, tracking_em=0.22)
    trio_x = (CW - trio_w) // 2
    trio_y = int(CH * 0.78)
    draw_letter_spaced(draw, trio_text, f_trio, trio_x, trio_y, GOLD, tracking_em=0.22)

    # Downsample to logical resolution for crisp output
    out = img.resize((W, H), Image.LANCZOS).convert("RGB")
    out.save(OUT, "PNG", optimize=True)
    print(f"Saved: {OUT} ({W}x{H})")


if __name__ == "__main__":
    render()
