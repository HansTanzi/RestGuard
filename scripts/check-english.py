#!/usr/bin/env python3
# Fails when comments, log messages or assertion messages contain non-English text.
# Non-English means any non-ASCII letter (CJK, kana, accented Latin, ...); symbols such as … → ° are fine.
# UI translations live in the excluded i18n files. Append `allow-non-english` to a line to exempt it.
# Usage: python scripts/check-english.py
import os
import re
import subprocess
import sys
import unicodedata
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
EXCLUDE = {"src/lib/i18n.ts", "src-tauri/src/i18n/text.rs", "pnpm-lock.yaml"}
ALLOW = "allow-non-english"

# (line comment markers, block comment delimiters, quote characters, backslash escapes)
C_LIKE = (("//",), (("/*", "*/"),), "\"'`", True)
HASH = (("#",), (), "\"'", True)
MARKUP = ((), (("<!--", "-->"),), '"', False)
LANGS = {
    # No ' here: in Rust it is mostly lifetimes
    ".rs": (("//",), (("/*", "*/"),), '"', True),
    ".ts": C_LIKE,
    ".js": C_LIKE,
    ".mjs": C_LIKE,
    ".cjs": C_LIKE,
    ".svelte": (("//",), (("/*", "*/"), ("<!--", "-->")), "\"'`", True),
    ".css": ((), (("/*", "*/"),), "\"'", True),
    ".html": MARKUP,
    ".xml": MARKUP,
    ".plist": MARKUP,
    ".yml": HASH,
    ".yaml": HASH,
    ".toml": HASH,
    ".sh": HASH,
    ".ps1": HASH,
    ".py": HASH,
    "PKGBUILD": HASH,
}

# Calls whose string arguments are logs, errors or assertion messages; matched against code with strings blanked out
LOG_CALL = re.compile(
    r"\b(?:e?print(?:ln)?!|panic!|unreachable!|todo!|unimplemented!|(?:debug_)?assert(?:_eq|_ne)?!"
    r"|(?:log|tracing)::\w+!|Err)\s*\("
    r"|\.expect\s*\(|\bconsole\.\w+\s*\("
)
# Single-line statements that print or raise
LOG_STMT = re.compile(r"\bthrow\b|\bWrite-(?:Host|Output|Error|Warning|Verbose)\b|\becho\b|\bprintf\b")


def foreign(text):
    return any(ord(ch) > 127 and unicodedata.category(ch).startswith("L") for ch in text)


def scan(line, spec, state):
    """Splits a line into code (strings blanked), string contents and comment contents.
    state["block"] holds the closing delimiter while inside a block comment spanning lines."""
    line_markers, blocks, quotes, escapes = spec
    code, strings, comments = [], [], []
    i, n = 0, len(line)
    while i < n:
        if state["block"]:
            end = line.find(state["block"], i)
            if end < 0:
                comments.append(line[i:])
                break
            comments.append(line[i:end])
            i = end + len(state["block"])
            state["block"] = None
            continue
        ch = line[i]
        if ch in quotes:
            j = i + 1
            while j < n and line[j] != ch:
                j += 2 if escapes and line[j] == "\\" else 1
            strings.append(line[i + 1 : j])
            code.append('""')
            i = j + 1
            continue
        marker = next((m for m in line_markers if line.startswith(m, i)), None)
        if marker:
            comments.append(line[i + len(marker) :])
            break
        block = next((b for b in blocks if line.startswith(b[0], i)), None)
        if block:
            state["block"] = block[1]
            i += len(block[0])
            continue
        code.append(ch)
        i += 1
    return "".join(code), strings, comments


def check(rel, spec):
    errors = []
    state = {"block": None}
    depth = 0  # open parentheses of a log call spanning lines
    text = (ROOT / rel).read_text(encoding="utf-8", errors="replace")
    for lineno, line in enumerate(text.splitlines(), 1):
        code, strings, comments = scan(line, spec, state)
        call = LOG_CALL.search(code)
        in_log = depth > 0 or call or LOG_STMT.search(code)
        if depth > 0:
            depth = max(depth + code.count("(") - code.count(")"), 0)
        elif call:
            rest = code[call.start() :]
            depth = max(rest.count("(") - rest.count(")"), 0)
        if ALLOW in line:
            continue
        if any(foreign(c) for c in comments):
            errors.append((lineno, "comment", line.strip()))
        elif in_log and any(foreign(s) for s in strings):
            errors.append((lineno, "log/assert message", line.strip()))
    return errors


def main():
    # The offending lines are non-ASCII; Windows consoles default to a legacy code page
    sys.stdout.reconfigure(encoding="utf-8", errors="replace")
    files =subprocess.run(
        ["git", "ls-files", "-z"], cwd=ROOT, capture_output=True, text=True, check=True
    ).stdout.split("\0")
    failed = 0
    for rel in filter(None, files):
        if rel in EXCLUDE:
            continue
        path = Path(rel)
        spec = LANGS.get(path.suffix) or LANGS.get(path.name)
        if not spec or not (ROOT / rel).is_file():
            continue
        for lineno, kind, line in check(rel, spec):
            failed += 1
            if os.environ.get("GITHUB_ACTIONS"):
                print(f"::error file={rel},line={lineno}::Non-English {kind}")
            print(f"{rel}:{lineno}: non-English {kind}: {line}")
    if failed:
        print(f"\n{failed} line(s) with non-English text. Write them in English, "
              f"or append `{ALLOW}` if the text must stay as is.")
        return 1
    print("OK: comments, logs and assertion messages are in English")
    return 0


if __name__ == "__main__":
    sys.exit(main())
