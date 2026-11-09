"""Package the imagegen original into Web/PWA and Windows icon sizes.

Only resize and encode; all artwork comes from the saved generated original.
Run: python tools/import_app_icon.py (requires Pillow).
"""
from pathlib import Path
from PIL import Image

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / 'assets' / 'app-icon' / 'kamakura.png'
WEB_SIZES = {'icon-512.png': 512, 'icon-192.png': 192, 'apple-touch-icon.png': 180}
ICO_SIZES = [(n, n) for n in (16, 24, 32, 48, 64, 128, 256)]


def main():
    with Image.open(SOURCE) as original:
        if original.width != original.height:
            raise ValueError('App icon original must be square; do not crop the artwork.')
        icon = original.convert('RGBA')
    if icon.getchannel('A').getextrema() != (255, 255):
        raise ValueError('App icon original must be fully opaque.')
    for filename, size in WEB_SIZES.items():
        icon.resize((size, size), Image.Resampling.LANCZOS).save(ROOT / 'web' / filename)
    icon.save(ROOT / 'assets' / 'app_icon.ico', sizes=ICO_SIZES)
    # Validate the encoded sizes, including every Windows icon directory entry.
    for filename, size in WEB_SIZES.items():
        with Image.open(ROOT / 'web' / filename) as output:
            assert output.size == (size, size), filename
            assert output.getchannel('A').getextrema() == (255, 255), filename
    with Image.open(ROOT / 'assets' / 'app_icon.ico') as output:
        assert output.ico.sizes() == set(ICO_SIZES)
    print('Kamakura app icon: Web 512/192, Apple 180, Windows 16-256 generated and verified.')


if __name__ == '__main__':
    main()
