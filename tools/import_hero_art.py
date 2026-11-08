"""Compile the six generated hero atlases without changing their painted content.

Only lossless cell extraction, transparent padding, resizing and BGRA encoding.
The original generated PNGs remain beside the compiled asset.
"""
from pathlib import Path
import json
from PIL import Image

ROOT = Path(__file__).resolve().parents[1]
ASSETS = ROOT / 'assets' / 'heroes'
SIZE = 128

def compile_atlases():
    names = json.loads((ASSETS / 'roster.json').read_text(encoding='utf-8'))
    raw = bytearray()
    entries = []
    catalog = Image.new('RGBA', (9 * SIZE, 6 * (SIZE + 26)), '#10222c')
    from PIL import ImageDraw, ImageFont
    draw = ImageDraw.Draw(catalog)
    try:
        font = ImageFont.truetype('C:/Windows/Fonts/YuGothM.ttc', 13)
    except OSError:
        font = ImageFont.load_default()
    for index, name in enumerate(names):
        sheet, slot = divmod(index, 9)
        path = ASSETS / f'sheet-{sheet + 1}.png'
        with Image.open(path) as src:
            src = src.convert('RGBA')
            col, row = slot % 3, slot // 3
            cell = src.crop((round(col * src.width / 3), round(row * src.height / 3),
                             round((col + 1) * src.width / 3), round((row + 1) * src.height / 3)))
        alpha = cell.getchannel('A')
        box = alpha.getbbox()
        if not box:
            raise ValueError(f'Empty sprite for {name}')
        if alpha.getextrema()[0] == 255:
            raise ValueError(f'Atlas lacks transparent background: {path}')
        # Uniform framing with room for selection rings. Never erase generated pixels.
        cell = cell.crop(box)
        cell.thumbnail((SIZE - 12, SIZE - 12), Image.Resampling.LANCZOS)
        sprite = Image.new('RGBA', (SIZE, SIZE))
        sprite.alpha_composite(cell, ((SIZE - cell.width) // 2, SIZE - 6 - cell.height))
        sprite.save(ASSETS / f'hero-{index:02d}.png')
        for red, green, blue, alpha in sprite.getdata():
            raw.extend(((blue * alpha + 127) // 255, (green * alpha + 127) // 255,
                        (red * alpha + 127) // 255, alpha))
        x, y = index % 9 * SIZE, index // 9 * (SIZE + 26)
        catalog.alpha_composite(sprite, (x, y))
        draw.text((x + 5, y + SIZE + 2), name, fill='#f6dda5', font=font)
        entries.append({'id': index, 'name': name, 'sheet': sheet + 1, 'row': row, 'column': col,
                        'sprite': f'hero-{index:02d}.png'})
    (ASSETS / 'kamakura.bgra').write_bytes(raw)
    (ASSETS / 'manifest.json').write_text(json.dumps(entries, ensure_ascii=False, indent=2), encoding='utf-8')
    catalog.convert('RGB').save(ASSETS / 'catalog.jpg', quality=94)
    print(f'Compiled {len(names)} distinct hero sprites; {len(raw)} BGRA bytes')

if __name__ == '__main__':
    compile_atlases()
