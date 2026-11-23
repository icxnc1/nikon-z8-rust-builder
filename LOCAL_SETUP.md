# Local setup and firmware provenance

The Windows/Linux release contains a self-contained compiled **Rust application**. It does not contain copyrighted original Nikon firmware, firmware-derived patch windows, or compiled Nikon patch payloads. These inputs must be prepared **privately and locally**.

1. Obtain your own legally acquired unmodified Nikon Z8 firmware **C3.11** update file.
2. Obtain the official *upstream project* release toolkit for **NikonFirmwarePatch v0.5.1** from https://github.com/XinmiaoZhao/NikonFirmwarePatch/releases/tag/v0.5.1 . Extract the toolkit, preserving its "features.py", "z8prores" modules, "resources/compiled.json", "resources/combinations.json" and associated resource files. Do not put those binary files into this public GitHub repository.
3. Install and configure a **real Python interpreter**, not only the WindowsApps execution aliases. The upstream v0.5.1 release includes `requirements.txt` with **`capstone==5.0.7`**. For example, in Ubuntu/WSL, in a private local directory:

    python3 -m venv .venv
    source .venv/bin/activate
    python -m pip install -r /absolute/path/to/NikonFirmwarePatch-v0.5.1/requirements.txt
    python --version
    python -c "import capstone; print(capstone.__version__)"

   The optional local generator uses Python once, and the native Rust application has **no Python runtime dependency**. For Windows, use a genuine Python installation and an equivalent activated venv; the local WindowsApps `python.exe` alias does **not** demonstrate Python is installed.
4. Run the optional bootstrap script, providing local absolute paths:

    python3 tools/generate_local_recipes.py --stock /path/to/Z_8_0311.bin --toolkit /path/to/upstream-v0.5.1 --output /private/generated-recipes

5. Run the Rust GUI using "--toolkit" and "--recipes" as shown in README. Supply original C3.11 input in the UI, choose the four feature groups and a **new, nonexistent output directory**.

Pinned original C3.11 SHA-256:
d67413c2ede6b9ce8887175f20700bd9fbad2a973a4820968c45503d15d80d96

Pinned generated 15-recipe manifest SHA-256:
ac5536b557dec974c742ba70e3ef50de5892a7a2ff66296bc2bea3c4b7b6dbed

The bootstrap refuses modified C3.11 input and overwriting an existing output folder. The Rust application must validate the upstream toolkit manifest and all 20 compiled units before building. No stock firmware, modified firmware or generated patch blobs should be uploaded to GitHub.

Even with all SHA/CRC checks passed, **firmware flashing is not authorized**; physical camera safety is not established by successful file-level validation.
