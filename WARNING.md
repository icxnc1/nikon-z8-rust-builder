# WARNING — Experimental Nikon Z8 Firmware Research / AVERTISMENT — Cercetare experimentală Nikon Z8

> **WORK IN PROGRESS — EXPERTS ONLY — NOT APPROVED FOR CAMERA INSTALLATION.**
>
> **PROIECT ÎN DEZVOLTARE — EXCLUSIV PENTRU SPECIALIȘTI — INSTALAREA PE APARAT NU ESTE AUTORIZATĂ.**

## English

This is independent, unfinished firmware and imaging-processor research, **not an official Nikon product or a safe, ready-to-install upgrade**. Firmware builds, emulator passes, successful compilation and integrity checks **do not establish safety on physical camera hardware**. No experimental image is approved for flashing merely because it appears in a repository or passes tests.

**Danger: permanent camera failure.** Installing or executing modified firmware may result in boot loops, a camera that will not start, corrupted data, lost functionality, hardware damage, expensive repair, or an **irrecoverably unusable camera**. Recovery cannot be guaranteed. Anyone who independently chooses to experiment does so **knowingly and at their own risk**, subject to applicable law; no warranty or recovery assistance is promised.

**Only for professionals or deeply experienced enthusiasts** who already understand low-level programming, ARM64/AArch64, embedded and RTOS architecture, bootloaders, memory mapping, DMA, concurrency, camera image-processing pipelines, RAW/NEF data, reverse engineering, debugging and recovery. General programming knowledge or following a tutorial is insufficient. **Beginners and ordinary camera users should not attempt firmware modifications.**

**Support boundary:** The project owner does **not** provide introductory teaching, beginner advice, step-by-step installation guidance, personal troubleshooting, or prolonged discussions with participants lacking the necessary technical background. Time is reserved for rigorous research and substantive expert contributions. This is a scope and safety restriction, not an insult.

**Evidence policy:** Suggestions, improvements and bug reports must be solidly documented and substantiated: exact firmware/tool version and commit, relevant file:line and addresses where applicable, reproducible commands and test inputs, actual versus expected results, baseline comparison, traces/logs or disassembly, technical root-cause analysis or explicitly labeled hypothesis, proposed fix and regression risks, and independent/adversarial validation where feasible. Clearly distinguish **proven**, **reproduced experimentally**, **hypothesized**, and **unverified**. Unsupported "it doesn't work", "I think this is better", or speculative advice will not be reviewed.

## Română

Acesta este un proiect independent, **neterminat**, de cercetare a firmware-ului și procesoarelor de imagine, **nu un produs oficial Nikon și nu o actualizare sigură, gata de instalare**. Compilarea, verificările binare și testele în emulator **nu demonstrează siguranța pe aparatul real**. Publicarea unui cod sau a unei imagini experimentale nu înseamnă autorizarea instalării.

**Pericol: blocarea definitivă a aparatului foto.** Firmware-ul modificat poate provoca imposibilitatea pornirii, bucle de boot, pierderea funcțiilor ori a datelor, deteriorarea hardware, reparații costisitoare sau **compromiterea ireversibilă a aparatului**. Recuperarea nu este garantată. Cine experimentează independent o face **în cunoștință de cauză, pe propria răspundere**, în limitele legii aplicabile, fără garanții de funcționare sau recuperare.

**Exclusiv pentru profesioniști și pasionați cu experiență tehnică profundă** în programare de nivel jos, ARM64/AArch64, sisteme încorporate și RTOS, bootloadere, memorie, DMA, concurență, procesoare de imagine, RAW/NEF, inginerie inversă și depanare hardware. Cunoștințele primare ori urmarea unui tutorial **nu sunt suficiente**. **Începătorii și utilizatorii obișnuiți nu trebuie să încerce modificarea firmware-ului.**

**Limita asistenței:** Proprietarul proiectului **nu oferă instruire elementară, sfaturi pentru începători, tutoriale de instalare pas cu pas, depanare individuală sau dialoguri îndelungate cu persoane fără baza tehnică necesară**. Timpul este rezervat cercetării riguroase și contribuțiilor de specialitate. Delimitarea privește domeniul proiectului și siguranța, nu valoarea persoanelor.

**Politica dovezilor:** Orice sugestie, îmbunătățire sau raportare de bug trebuie **documentată riguros și susținută cu probe**: versiuni exacte și commit, fișier:linie și adrese relevante, comenzi și date reproductibile, rezultat obținut versus așteptat, comparație cu referința, jurnale/dezasamblare, cauza tehnică demonstrată ori ipoteza explicită, corecția propusă, riscuri de regresie și verificare independentă/adversarială când este posibil. Se separă strict **demonstrat**, **reprodus experimental**, **ipoteză** și **neverificat**. Afirmații de tipul „nu merge”, „așa cred eu” sau recomandări speculative, fără dovezi, nu vor fi analizate.

---

Independent research. Not affiliated with, endorsed by or supported by Nikon. / Cercetare independentă, fără afiliere ori aprobare Nikon.
