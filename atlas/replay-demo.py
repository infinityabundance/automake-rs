#!/usr/bin/env python3
"""Render an animated terminal GIF of `cargo xtask atlas-replay` for the atlas README."""
from PIL import Image, ImageDraw, ImageFont

REG = "/usr/share/fonts/TTF/MesloLGMNerdFontMono-Regular.ttf"
BLD = "/usr/share/fonts/TTF/MesloLGMNerdFontMono-Bold.ttf"
FS = 19
font = ImageFont.truetype(REG, FS)
fontb = ImageFont.truetype(BLD, FS)

# palette (GitHub dark)
BG = (13, 17, 23)
BAR = (22, 27, 34)
FG = (230, 237, 243)
DIM = (139, 148, 158)
GRN = (86, 211, 100)
RED = (248, 81, 73)
BLU = (88, 166, 255)
YEL = (210, 153, 34)
PRM = (126, 231, 135)
RframeDot = [(255, 95, 86), (255, 189, 46), (39, 201, 63)]

CW = font.getbbox("M")[2]          # char width (monospace)
LH = FS + 8                        # line height
PADX, PADY = 22, 16
BARH = 34
COLS = 78
W = PADX * 2 + CW * COLS
ROWS = 16
H = BARH + PADY * 2 + LH * ROWS

# The session: list of segments per line. Each line = list of (text, color, bold)
PROMPT = [(" ", BLU, True), ("~/automake-rs", PRM, True), (" $ ", FG, False)]
CMD = "cargo xtask atlas-replay ayumin/open-cobol"
OUT = [
    [("atlas-replay: recipe atlas/recipes/ayumin__open-cobol.json", DIM, False)],
    [("  [1/5] ", BLU, True), ("clone https://github.com/ayumin/open-cobol ...", FG, False)],
    [("        clone: ", DIM, False), ("ok", GRN, True)],
    [("  [2/5] ", BLU, True), ("checkout 72578e8fe3f1 ", FG, False), ("(pinned)", DIM, False)],
    [("  [3/5] ", BLU, True), ("autoreconf-rs -fi ...", FG, False)],
    [("        autoreconf: ", DIM, False), ("ok (configure generated)", GRN, True)],
    [("  [4/5] ", BLU, True), ("./configure ...", FG, False)],
    [("        configure: ", DIM, False), ("ok", GRN, True)],
    [("  [5/5] ", BLU, True), ("make -j2 ...", FG, False)],
    [("        make: ", DIM, False), ("ok", GRN, True)],
    [("        verify: ", DIM, False), ("6 matched", GRN, True), (", ", DIM, False),
     ("4 hash-mismatch", YEL, True), (", ", DIM, False), ("2 missing", DIM, True),
     (" (of 12)", DIM, False)],
    [("", FG, False)],
    [("atlas-replay: ayumin/open-cobol — ", FG, False), ("diverged", YEL, True),
     ("  (6/12 byte-identical)", DIM, False)],
]

def base():
    img = Image.new("RGB", (W, H), BG)
    d = ImageDraw.Draw(img)
    d.rectangle([0, 0, W, BARH], fill=BAR)
    for i, c in enumerate(RframeDot):
        cx = 18 + i * 22
        d.ellipse([cx, BARH//2-7, cx+14, BARH//2+7], fill=c)
    d.text((W//2 - 70*1, BARH//2 - FS//2), "atlas-replay", font=font, fill=DIM)
    return img, d

def draw_line(d, row, segs, cursor=False, partial=None):
    y = BARH + PADY + row * LH
    x = PADX
    for text, color, bold in segs:
        if partial is not None and text is partial[0]:
            text = partial[1]
        d.text((x, y), text, font=(fontb if bold else font), fill=color)
        x += CW * len(text)
    if cursor:
        d.rectangle([x, y+2, x+CW-2, y+FS+2], fill=FG)

frames = []
def snap(img, dur):
    frames.append((img.copy(), dur))

# Phase 1: type the command
for i in range(len(CMD)+1):
    img, d = base()
    draw_line(d, 0, PROMPT + [(CMD[:i], FG, False)], cursor=True)
    snap(img, 35 if i < len(CMD) else 500)

# Phase 2: reveal output lines progressively
shown = []
for li, line in enumerate(OUT):
    shown.append(line)
    img, d = base()
    # scroll: keep last ROWS-1 lines after the command
    draw_line(d, 0, PROMPT + [(CMD, FG, False)])
    start = max(0, len(shown) - (ROWS - 2))
    for r, sline in enumerate(shown[start:]):
        draw_line(d, r + 1, sline)
    # pacing: pause longer on the "..." action lines and the final verdict
    last = "".join(t for t, _, _ in line)
    dur = 90
    if last.endswith("...)") or "..." in last and "ok" not in last:
        dur = 70
    if "ok" in last:
        dur = 320
    if "diverged" in last:
        dur = 2600
    snap(img, dur)

# hold final
snap(frames[-1][0], 2600)

imgs = [f[0] for f in frames]
durs = [f[1] for f in frames]
out = "/home/one/automake-rs/atlas/replay-demo.gif"
imgs[0].save(out, save_all=True, append_images=imgs[1:], duration=durs, loop=0, optimize=True)
print("wrote", out, "frames=", len(imgs), "size W×H=", W, "×", H)
