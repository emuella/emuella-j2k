"""Shared fail-closed content policy for public source and release archives."""

from __future__ import annotations

import hashlib
import re
from collections.abc import Mapping
from pathlib import PurePosixPath

# Adding a suffix here is a policy change: it declares that every UTF-8 file with
# that suffix is an expected public source/text format. Extensionless files are
# allowed only through the basename list below.
ALLOWED_TEXT_SUFFIXES = frozenset(
    {
        ".json",
        ".lock",
        ".md",
        ".py",
        ".rs",
        ".sh",
        ".toml",
        ".txt",
        ".yaml",
        ".yml",
    }
)

# Legal and packaging tools use these reviewed extensionless or unusual names.
ALLOWED_TEXT_BASENAMES = frozenset(
    {
        ".cargo_vcs_info.json",
        ".gitattributes",
        ".gitignore",
        "Cargo.toml.orig",
        "LICENSE",
        "LICENSE-APACHE",
        "LICENSE-APACHE-2.0",
        "LICENSE-BSD-2-CLAUSE",
        "LICENSE-MIT",
        "LICENSE-OPENJPH-BSD-2-CLAUSE",
        "LICENSE-ZLIB",
        "METADATA",
        "NOTICE",
        "PKG-INFO",
        "RECORD",
        "WHEEL",
    }
)

# These categories make diagnostics explicit. All other unapproved suffixes are
# rejected as unknown, so this is not intended to be an exhaustive blacklist.
FORBIDDEN_SUFFIX_CATEGORIES: Mapping[str, str] = {
    # Executables and dynamically loaded code.
    ".apk": "executable",
    ".appimage": "executable",
    ".bin": "executable",
    ".class": "executable",
    ".com": "executable",
    ".dex": "executable",
    ".dll": "executable",
    ".dylib": "executable",
    ".exe": "executable",
    ".jar": "executable/archive",
    ".msi": "executable",
    ".pyd": "executable",
    ".so": "executable",
    ".wasm": "executable",
    # Objects, static libraries, debug data, and compiled intermediates.
    ".a": "object/library",
    ".bc": "object/library",
    ".dwo": "object/debug",
    ".lib": "object/library",
    ".o": "object",
    ".obj": "object",
    ".pdb": "object/debug",
    ".pyc": "compiled Python",
    # Archives and packages.
    ".7z": "archive",
    ".bz2": "archive",
    ".crate": "archive",
    ".deb": "archive/package",
    ".gz": "archive",
    ".rar": "archive",
    ".rpm": "archive/package",
    ".tar": "archive",
    ".tgz": "archive",
    ".whl": "archive/package",
    ".xz": "archive",
    ".zip": "archive",
    ".zst": "archive",
    # Raster/vector images and codec fixtures.
    ".avif": "image",
    ".bmp": "image",
    ".gif": "image",
    ".heic": "image",
    ".ico": "image",
    ".j2c": "image/codec fixture",
    ".j2k": "image/codec fixture",
    ".jhc": "image/codec fixture",
    ".jp2": "image/codec fixture",
    ".jpeg": "image",
    ".jph": "image/codec fixture",
    ".jpg": "image",
    ".pgm": "image/codec fixture",
    ".pgx": "image/codec fixture",
    ".png": "image",
    ".ppm": "image/codec fixture",
    ".svg": "image",
    ".tif": "image/codec fixture",
    ".tiff": "image/codec fixture",
    ".webp": "image",
    # Audio and video.
    ".aac": "media",
    ".avi": "media",
    ".flac": "media",
    ".m4a": "media",
    ".mkv": "media",
    ".mov": "media",
    ".mp3": "media",
    ".mp4": "media",
    ".mpeg": "media",
    ".ogg": "media",
    ".wav": "media",
    ".webm": "media",
    # Databases and columnar data.
    ".arrow": "database/data",
    ".db": "database",
    ".duckdb": "database",
    ".feather": "database/data",
    ".mdb": "database",
    ".parquet": "database/data",
    ".sqlite": "database",
    ".sqlite3": "database",
    # Models, tensors, and checkpoints.
    ".ckpt": "model",
    ".gguf": "model",
    ".h5": "model",
    ".onnx": "model",
    ".pt": "model",
    ".pth": "model",
    ".safetensors": "model",
    ".tflite": "model",
    # Rendered/editable documents.
    ".doc": "document",
    ".docx": "document",
    ".epub": "document",
    ".odf": "document",
    ".odp": "document",
    ".ods": "document",
    ".odt": "document",
    ".pdf": "document",
    ".ppt": "document",
    ".pptx": "document",
    ".rtf": "document",
    ".xls": "document",
    ".xlsx": "document",
}

MAGIC_SIGNATURES: tuple[tuple[bytes, str], ...] = (
    (b"\x7fELF", "ELF executable/object"),
    (b"MZ", "PE executable/object"),
    (b"\xfe\xed\xfa\xce", "Mach-O executable/object"),
    (b"\xfe\xed\xfa\xcf", "Mach-O executable/object"),
    (b"\xce\xfa\xed\xfe", "Mach-O executable/object"),
    (b"\xcf\xfa\xed\xfe", "Mach-O executable/object"),
    (b"\xca\xfe\xba\xbe", "Mach-O universal binary/Java class"),
    (b"\x00asm", "WebAssembly executable"),
    (b"!<arch>\n", "static-library archive"),
    (b"PK\x03\x04", "ZIP archive"),
    (b"\x1f\x8b", "gzip archive"),
    (b"BZh", "bzip2 archive"),
    (b"\xfd7zXZ\x00", "xz archive"),
    (b"7z\xbc\xaf'\x1c", "7-Zip archive"),
    (b"Rar!\x1a\x07", "RAR archive"),
    (b"\x89PNG\r\n\x1a\n", "PNG image"),
    (b"\xff\xd8\xff", "JPEG image"),
    (b"GIF87a", "GIF image"),
    (b"GIF89a", "GIF image"),
    (b"%PDF-", "PDF document"),
    (b"SQLite format 3\x00", "SQLite database"),
    (b"\xd0\xcf\x11\xe0\xa1\xb1\x1a\xe1", "OLE document/database"),
)

SHA256_PATTERN = re.compile(r"[0-9a-f]{64}\Z")
PUBLIC_OPENJPH_IDENTIFIER = re.compile(
    r"\bpub\s+(?!\()[^;\n{=]*\bopenjph[A-Za-z0-9_]*\b",
    re.IGNORECASE,
)
ALLOWED_CONTROL_BYTES = frozenset({9, 10, 12, 13})


def sha256_bytes(content: bytes) -> str:
    return hashlib.sha256(content).hexdigest()


def contains_public_openjph_identifier(text: str) -> bool:
    """Return whether Rust-like source exports an OpenJPH-named identifier."""
    return PUBLIC_OPENJPH_IDENTIFIER.search(text) is not None


def binary_reason(content: bytes) -> str | None:
    """Return why content is binary, or None for strict UTF-8 text."""
    for signature, description in MAGIC_SIGNATURES:
        if content.startswith(signature):
            return description
    if b"\x00" in content:
        return "NUL byte"
    try:
        content.decode("utf-8")
    except UnicodeDecodeError:
        return "non-UTF-8 bytes"
    for value in content:
        if value < 32 and value not in ALLOWED_CONTROL_BYTES:
            return f"control byte 0x{value:02x}"
        if value == 127:
            return "DEL control byte"
    return None


def is_allowed_text_name(path: PurePosixPath) -> bool:
    return (
        path.name in ALLOWED_TEXT_BASENAMES
        or path.suffix.lower() in ALLOWED_TEXT_SUFFIXES
    )


def content_policy_errors(
    path: PurePosixPath,
    content: bytes,
    *,
    hash_exceptions: Mapping[PurePosixPath, str] | None = None,
) -> list[str]:
    """Apply the public-file policy, allowing only exact hash-pinned exceptions."""
    exceptions = hash_exceptions or {}
    expected_hash = exceptions.get(path)
    if expected_hash is not None:
        if not SHA256_PATTERN.fullmatch(expected_hash):
            return [f"invalid exception SHA-256 for {path}: {expected_hash!r}"]
        actual_hash = sha256_bytes(content)
        if actual_hash != expected_hash:
            return [
                f"hash-pinned exception differs: {path} "
                f"(expected {expected_hash}, got {actual_hash})"
            ]
        return []

    suffix = path.suffix.lower()
    category = FORBIDDEN_SUFFIX_CATEGORIES.get(suffix)
    if category is not None:
        return [f"forbidden {category} file: {path}"]
    if not is_allowed_text_name(path):
        displayed_suffix = suffix or "<none>"
        return [f"unreviewed file type {displayed_suffix}: {path}"]
    reason = binary_reason(content)
    if reason is not None:
        return [f"binary content in approved text file: {path} ({reason})"]
    return []


def exception_configuration_errors(
    paths: set[PurePosixPath], exceptions: Mapping[PurePosixPath, str]
) -> list[str]:
    """Reject malformed or stale exception entries."""
    errors: list[str] = []
    for path, expected_hash in exceptions.items():
        if path.is_absolute() or ".." in path.parts or "\\" in path.as_posix():
            errors.append(f"unsafe hash-pinned exception path: {path}")
        if not SHA256_PATTERN.fullmatch(expected_hash):
            errors.append(f"invalid exception SHA-256 for {path}: {expected_hash!r}")
        if path not in paths:
            errors.append(f"stale hash-pinned exception: {path}")
    return errors
