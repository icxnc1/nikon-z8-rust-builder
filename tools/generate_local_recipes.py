#!/usr/bin/env python3
"""Local, optional bootstrap of pinned upstream v0.5.1 patch recipes.

Needs original C3.11 firmware and the already-downloaded upstream release toolkit.
Python is only the OFFLINE preparatory step; native Rust builder does not invoke it.
The output contains firmware-derived bytes and MUST NEVER be uploaded.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import sys

EXPECTED = "d67413c2ede6b9ce8887175f20700bd9fbad2a973a4820968c45503d15d80d96"
EG_SHA = "f85e0138e2f5e660ece8560de62e0d12cb795e0e14983e50f20b61dce906c46c"
UPSTREAM = "25fefa1e17f321ee285bea77b3c54427f9912e4b"
RECIPE_SHA = "ac5536b557dec974c742ba70e3ef50de5892a7a2ff66296bc2bea3c4b7b6dbed"
FEATURES = ("video-recording", "log-colour", "still-photo-tools", "raw-precapture")


def sha(buf: bytes) -> str:
    return hashlib.sha256(buf).hexdigest()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--stock", required=True, type=Path)
    parser.add_argument("--toolkit", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()

    original = args.stock.read_bytes()
    if sha(original) != EXPECTED:
        raise ValueError("C3.11 SHA-256 mismatch: reject untrusted firmware")
    toolkit = args.toolkit
    if not (toolkit / "resources/compiled.json").is_file():
        raise FileNotFoundError("Missing upstream v0.5.1 compiled.json")
    if not (toolkit / "resources/combinations.json").is_file():
        raise FileNotFoundError("Missing upstream v0.5.1 combinations.json")
    if args.output.exists():
        raise FileExistsError("Output already exists: refuse overwrite")

    sys.path.insert(0, str(toolkit.resolve()))
    from z8prores.package import Package  # type: ignore
    from features import compose  # type: ignore

    eg = Package(original).eg
    if sha(eg) != EG_SHA:
        raise ValueError("Expected C3.11 EG data hash mismatch")
    cookbook = json.loads((toolkit / "resources/combinations.json").read_text(encoding="utf8"))
    units = json.loads((toolkit / "resources/compiled.json").read_text(encoding="utf8"))["units"]
    if len(units) != 20:
        raise ValueError("Expected exactly 20 upstream compiled units")
    for name, entry in units.items():
        resource = entry["resource"]
        if not resource.startswith("resources/") or ".." in resource:
            raise ValueError("Unsafe upstream resource name")
        raw = (toolkit / resource).read_bytes()
        if len(raw) != entry["bytes"] or sha(raw) != entry["sha256"]:
            raise ValueError(f"Compiled unit corrupted: {name}")

    args.output.mkdir(parents=True)
    compiled = []
    for mask in range(1, 16):
        selected = [name for i, name in enumerate(FEATURES) if mask & (1 << i)]
        profile = cookbook["profiles"]["+".join(selected)]
        image, _, rows = compose(eg, selected, check_recipe=True)
        edits = sorted(image.edits, key=lambda edit: edit["offset"])
        if len(edits) != len(rows) or len(edits) != len(profile["patches"]):
            raise ValueError(f"Recipe row mismatch: {mask}")
        windows = []
        blob = bytearray()
        cursor = 0
        for index, edit in enumerate(edits):
            pos = edit["offset"]
            old, new = edit["old"], edit["new"]
            length = len(new)
            if length == 0 or pos < cursor or pos + length > len(eg) - 2:
                raise ValueError("Invalid or overlapping patch window")
            if eg[pos:pos + length] != old:
                raise ValueError("Patch expected preimage differs")
            if sha(old) != rows[index]["old_sha256"] or sha(new) != rows[index]["new_sha256"]:
                raise ValueError("Per-window hash mismatch")
            if rows[index] != cookbook["patches"][profile["patches"][index]]:
                raise ValueError("Reviewed upstream patch mismatch")
            windows.append({
                "offset": pos, "bytes": length, "blob_at": len(blob),
                "old_sha256": sha(old), "new_sha256": sha(new),
            })
            blob += new
            cursor = pos + length
        if len(image.new) != len(eg):
            raise ValueError("Patched EG length differs")
        bname = f"mask-{mask:02}.patch"
        jname = f"mask-{mask:02}.json"
        desc = {
            "schema": "NIKON_Z8_RUST_PATCH_WINDOWS_V051_V1",
            "mask": mask, "features": selected, "source_commit": UPSTREAM,
            "source_version": "v0.5.1",
            "original_fw_sha256": EXPECTED, "original_eg_sha256": EG_SHA,
            "reviewed_profile": profile["id"], "windows": windows,
            "patch_bytes": len(blob), "patch_sha256": sha(blob),
            "count": len(windows), "compiled_assets_verified": True,
            "original_untouched": True, "flash_approved": False,
        }
        (args.output / bname).write_bytes(blob)
        desc_text = json.dumps(desc, indent=2) + "\n"
        (args.output / jname).write_text(desc_text, encoding="utf8")
        compiled.append({
            "mask": mask, "features": selected, "patch_file": bname,
            "descriptor_file": jname, "patch_sha256": sha(blob),
            "descriptor_sha256": sha(desc_text.encode("utf8")),
            "edit_count": len(windows), "patch_bytes": len(blob),
        })
        print("LOCAL_RECIPE_READY", mask, len(windows), len(blob), flush=True)

    registry = {
        "schema": "NIKON_Z8_RUST_NATIVE_RECIPE_SET_V051_V1",
        "source_commit": UPSTREAM, "stock_sha256": EXPECTED,
        "stock_eg_sha256": EG_SHA, "mask_count": 15, "compiled_units": 20,
        "combinations": compiled, "flash_approved": False,
        "source": "NikonFirmwarePatch v0.5.1 reviewed-combinations, compiled unit SHA pinned",
    }
    data = (json.dumps(registry, indent=2) + "\n").encode("utf8")
    if sha(data) != RECIPE_SHA:
        raise ValueError("Locally prepared recipe set differs from pinned reference")
    (args.output / "recipe-set.json").write_bytes(data)
    if sha(args.stock.read_bytes()) != EXPECTED:
        raise ValueError("Original firmware changed during bootstrap")
    print("LOCAL_RECIPE_SET_COMPLETE", len(compiled), sha(data), flush=True)


if __name__ == "__main__":
    main()
