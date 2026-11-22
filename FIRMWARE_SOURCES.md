# Nikon Z8 firmware download sources

This repository **does not host or redistribute Nikon firmware**. Obtain original Nikon Z8 update files on your own computer, keeping the filename, exact version and checksum recorded before experimenting.

## Firmware links

1. **Softpedia Nikon Z8 search** — the project owner's working discovery/download route, which they report redirects to the official Nikon source:
   https://drivers.softpedia.com/dyn-search.php?search_term=z+8&p_category=2
2. **Nikon Download Center — Z8** — official Nikon firmware and update instructions:
   https://downloadcenter.nikonimglib.com/
   Search for **Z8**, then select the firmware/version required.

Softpedia is included **exactly as supplied by the project owner**. When accessing a download redirected to Nikon, check the final address and the file version; prefer preserving the Nikon-supplied original rather than repackaged files.

## SHA-256 check for experimental Rust builder

The current v0.5.1-based Rust recipe set accepts **only this exact original C3.11 image**:

`Z_8_0311.bin` — SHA-256:

`d67413c2ede6b9ce8887175f20700bd9fbad2a973a4820968c45503d15d80d96`

Windows PowerShell:

```powershell
Get-FileHash -Algorithm SHA256 -LiteralPath 'D:\Z_8_0311.bin'
```

Ubuntu / WSL2:

```bash
sha256sum '/path/to/Z_8_0311.bin'
```

**A different hash or firmware version must not be forced into the C3.11 recipe.** Other firmware versions need separate validated source profiles and offsets. Matching SHA-256 is a file-identity check, **not proof that an experimental patched firmware is safe to install**.

## Original software author

- https://github.com/XinmiaoZhao/NikonFirmwarePatch
- https://github.com/XinmiaoZhao/NikonFirmwarePatch/releases/tag/v0.5.0
- https://github.com/XinmiaoZhao/NikonFirmwarePatch/releases/tag/v0.5.1
