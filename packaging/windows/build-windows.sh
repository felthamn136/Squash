#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

mkdir -p dist

if command -v x86_64-w64-mingw32-gcc >/dev/null 2>&1; then
  LINKER="$(command -v x86_64-w64-mingw32-gcc)"
  DLLTOOL="$(command -v x86_64-w64-mingw32-dlltool)"
elif command -v x86_64-w64-mingw32-gcc-posix >/dev/null 2>&1; then
  LINKER="$(command -v x86_64-w64-mingw32-gcc-posix)"
  DLLTOOL="$(command -v x86_64-w64-mingw32-dlltool)"
else
  echo "Missing MinGW-w64. Install gcc-mingw-w64-x86-64-posix and mingw-w64-x86-64-dev." >&2
  exit 1
fi

command -v makensis >/dev/null 2>&1 || {
  echo "Missing NSIS (makensis). Install nsis." >&2
  exit 1
}

python3 - <<'PY'
from PIL import Image, ImageDraw

image = Image.new("RGBA", (256, 256), "#101010")
draw = ImageDraw.Draw(image)
draw.rounded_rectangle((18, 18, 238, 238), radius=28, fill="#171c18", outline="#92efad", width=8)
draw.rectangle((128, 42, 136, 214), fill="#92efad")
draw.ellipse((82, 112, 118, 148), fill="#92efad")
draw.rectangle((202, 82, 214, 174), fill="#92efad")
image.save("packaging/windows/squash-76.ico", sizes=[(16, 16), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256)])
PY

rustup target add x86_64-pc-windows-gnu
CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER="$LINKER" \
CARGO_TARGET_X86_64_PC_WINDOWS_GNU_DLLTOOL="$DLLTOOL" \
cargo build --release --target x86_64-pc-windows-gnu

makensis packaging/windows/setup.nsi
makensis packaging/windows/uninstaller.nsi

ls -lh dist/Squash-76-Setup.exe dist/Squash-76-Uninstaller.exe
