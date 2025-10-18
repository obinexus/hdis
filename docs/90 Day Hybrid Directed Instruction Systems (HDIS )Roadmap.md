# 90-day starter plan (get a working prototype)

Goal: have a reproducible dev system that demonstrates *state awareness* and *self-repair* on simple faults.

Hardware

* Small modular dev board(s):

  * RISC-V dev board (SiFive or similar) *or* ARM SBC (RockPro64 / Raspberry Pi 4 for cheap prototyping).
  * FPGA dev board (Xilinx/Intel/Arty/Alchitry) for hardware experiments (custom D-RAM controller / error detector).
  * NVMe SSD with power-loss protection.
  * ECC-capable DIMM (if board supports) or emulate via FPGA.
  * External watchdog, TPM (hardware root of trust), and a small BMC/management MCU (e.g., STM32) if you plan remote management.
* Peripherals: logic analyzer, JTAG debugger, multimeter, USB-UART, SD cards.

Software/firmware

* Use Rust for critical components + C for low-level drivers.
* Linux (your SBC) or a minimal unikernel on RISC-V for deterministic experiments (Buildroot, Yocto, or a tiny RTOS like Zephyr).
* FPGA: write a small D-RAM controller that tags writes with a state checksum / sequence number (Verilog/Chisel).
* Implement a tiny “state-watcher” daemon that:

  * Journals key memory regions (checksums + timestamps).
  * Detects divergence, triggers checkpoint/rollback.
  * Bubbles errors upward via structured logs and observable events.

Experiments to run in first 90 days

* Fault injection: flip bits in RAM (via FPGA or software) and show detection + self-repair (rollback to last good checkpoint).
* Error bubbling demo: forced kernel panic -> capture causal chain and map to source code line/time.
* Reproducibility test: replicate a “corruption” event across two identical boards to show deterministic recovery.

Deliverable after 90 days: **README + demo video + reproducible script** that injects a fault and shows the system correcting itself.

---

# 12-month plan (robust prototype + initial evaluation)

Goal: move from toy to credible system and start building measurable claims.

Architecture

* Design a small hybrid stack:

  * **Active Memory**: the RAM/SSD layer that watches and annotates state (a.k.a. D-RAM concept).
  * **Observer Kernel**: runs message-passing services, bubbles errors to root cause.
  * **Phenomenology Bridge**: an API to map system states to human-observable artifacts (logs → images/sounds) to test your 95.4% coherence idea.

Components & tasks

* Build FPGA D-RAM with metadata tags (age, origin, checksum).
* Implement ECC + journaling + background repair thread.
* Create a reproducible error-bubbling pipeline (instrumentation + structured trace).
* Implement a test harness that runs workloads and records recovery metrics.

Measurement & validation

* Define clear metrics (mean time to detect, mean time to repair, percent successful recovery).
* Start small human experiments for coherence mapping: map small system states to emotional cues (for your phenot pipeline)—collect listener/observer data and run basic stats.
* Instrument telemetry to produce a dashboard (simple JS + time-series DB) tracking “coherence” (system-level) and “transfer” (human-level).

Deliverables after 12 months:

* Prototype board(s) with source code on GitHub.
* Paper-style technical report + reproducible benchmarks.
* Short UX study (n=10–30) showing early phenomenological mapping.

---

# 3-year / PhD path (publishable, defensible thesis)

Goal: produce a PhD thesis around HDIS: “Hybrid Directed Instruction Systems — active memory and phenomenological coherence”.

Thesis structure (high level)

1. **Introduction & Motivation** — systems fail; need active, self-aware infra.
2. **Related work** — self-healing hardware, ECC, checkpoint/rollback, fault-tolerant systems, observability, cyber-physical systems.
3. **HDIS architecture** — formal spec for D-RAM, error-bubbling semantics, observer kernel, phenot bridge.
4. **Implementation** — FPGA D-RAM, RISC-V/ARM prototype, kernel and userland.
5. **Evaluation** — fault injection, recovery metrics, usability/phenomenology experiments (coherence measurement).
6. **Discussion** — limitations, ethics (safety-critical), future work (quantum hybrids?).
7. **Conclusion**.

Research contributions to aim for

* Novel active-memory design that annotates and self-repairs with low overhead.
* Formal model for error-bubbling (proofs about causality/locality).
* A reproducible method to measure feeling-transfer coherence (the phenot metric) linking technical states to human data.

Suggested research methods

* Formal verification of the observer kernel components (model checking).
* Hardware-in-loop tests; large fault matrices.
* Human-subjects work for coherence: preregister experiments, IRB if needed, standardized questionnaires + physiological measures (optional).

---

# Experiments & methods to measure your 95.4% coherence idea

You want a measurable pipeline — here’s a practical approach.

1. **Define micro-feelings**: pick a small set (e.g., calm, tension, wonder — 3 states). For each, design signal mappings (audio frequencies, color palettes, rhythm).
2. **Controlled creation**: the creator performs/produces art with the encoded mapping; record the creator’s declared state.
3. **Blind test**: participants (n≥30 per feeling) observe stimuli and rate on Likert scales & forced-choice classification.
4. **Metric**: coherence = proportion of participants who choose the creator’s declared state. Bootstrap CI. Your target of 0.954 is high — start with achievement baseline, iterate mapping.
5. **Physiological signals** (optional): heart rate variability, GSR — correlate with declared feeling to strengthen claim.
6. **Reproducibility**: repeat across different audiences, cultures, mediums.

---

# Tools / projects to use right away

* FPGA: Vivado (Xilinx) or open tools (SymbioticEDA / Yosys) + Arty/Alchitry boards.
* RISC-V: SiFive boards, QEMU for emulation.
* OS: Buildroot / Yocto / Zephyr / minimal Linux.
* Languages: Rust (memory-safety), C for drivers, Verilog/Chisel for FPGA.
* Monitoring: Prometheus + Grafana for telemetry + a simple HTTP API for phenot bridge.
* Tests: fault-injection frameworks, chaos engineering principles.

---

# Practical BOM (minimal)

* RISC-V devboard or RockPro64 / Raspberry Pi 4 — £50–150
* FPGA devboard (Arty A7 / Nexys / Alchitry) — £80–200
* ECC DIMM (if supported) or standard DDR with FPGA emulation — £30–100
* NVMe SSD with power-loss protection — £50–150
* STM32 devboard for BMC/watchdog — £10–20
* Logic analyzer / JTAG adapter — £40–120
* Misc cables / power / SD cards — £30

---

# Quick rules from experience (don’t waste time)

* Prototype on commodity boards first — design complexity kills momentum.
* Keep a reproducible script: a single `make demo` that runs your fault-inject -> recovery demo.
* Log everything with deterministic timestamps; provenance is gold for PhD defenses.
* Use Rust for the observer parts — less undefined behaviour, easier to verify.
* Be ruthless about scope: prove one core claim (self-detect & self-repair) before selling the entire manifesto.
