# `solidasm`: Multi-Body CAD Assembly Synthesis, Steinmetz NURBS Coping, SE(3) Kinematics & Paid Cloud API (`solidSF`)

`solidasm` is solidSF's pure-Rust, zero-FFI multi-body CAD assembly synthesis, Steinmetz bicubic rational NURBS tube coping, `SE(3)` kinematic mechanism solver, 3D tolerance stackup verifier, Bill of Materials (`BOM`) generator, and paid Cloud API package (`wss://api.solidsf.com/v1/solidasm/ws` and `https://api.solidsf.com/v1/solidasm/*`).

## Paid API & `solidSF Credits` Retail Metering
- **Retail Markup**: `30%` over base compute cost (`RETAIL_MARKUP_PERCENT = 30`, `RETAIL_MARKUP_BPS = 3000`)
- **Accounting Precision**: Exact integer micro-USD (`1 USD = 1,000,000 micro-USD`)
- **Minimum Purchase**: `$10.00` (`1,000` cents = `10,000,000` micro-USD), zero free tier
- **6 Metered `solidasm` Cloud Engines**:
  1. `VideoToAssemblyDecomposer` (`25` micro-USD)
  2. `SteinmetzNurbsTubeCoper` (`20` micro-USD)
  3. `Se3KinematicMechanismSolver` (`20` micro-USD)
  4. `ToleranceAndClearanceVerifier` (`15` micro-USD)
  5. `HierarchicalStepGlbBomExporter` (`20` micro-USD)
  6. `AssemblyToCaeHandoffBridge` (`15` micro-USD)

## Formula SAE Championship Starter Project (`SSF-FSAE-26`)
- **Synthesizer Example (`1,000` Chain Deep x `10` Wide, `1,000x` Complexity, Zero Efficient Sampling):**
  - `cargo run --release --example fsae_solidsf_2026_jesse_swarm`
- **Reference Fastener & OEM COTS `STEP AP242` Catalog (`src/fasteners.rs` & `assets/reference_cad/`):**
  - `NAS6604-14` close-tolerance shear bolts, `ISO 4762 M8x1.25-10.9` socket head cap screws, `MS21042-L4` all-metal prevailing-torque jet nuts, `DIN 25201` Nord-Lock & `DIN 463` tab washers, `Aurora PRXM-8T` PTFE spherical rod ends, and `Click Bond CB4000` composite nutplates (`FSAE Rule T.9` compliant).
  - Reference `STEP AP242` envelopes for `Ohlins TTX25 MkII` dampers, `Drexler` Salisbury LSD, `ISR 22-048` 4-piston monobloc calipers, `AP Racing CP6465` twin master cylinders, `SKF 61810-2RS1` thin-section wheel bearings, and `MoTeC M150` / `C185` enclosures.
- **`60fps` WebGL2 3D Studio & Video Renderer:**
  - `python3 crates/solidasm/examples/render_fsae_solidsf_2026.py`

## Aston Martin Valkyrie AMR 100,000-Loop × 3-Agent Triad (`NO SSBX`)
- **Synthesizer Example (`100,000` Loops × `3` Triad Agents `Edit -> Review -> Analyze -> Review -> Test -> Publish`, `80,000` Watertight $\chi = 2$ BRep Solids, `16,000,000` FeatureTree DAG Nodes, Zero Early-Exit Pruning):**
  - `cargo run --release --example valkyrie_100k_loop_team3_no_ssbx`
  - `cargo run --release --example valkyrie_1000passes_team10_no_ssbx`
- **Interactive `app.solidsf` Neutral-Lit 3D Studio & Half-Opaque/Half-X-Ray Video Renderer:**
  - `python3 crates/solidasm/examples/render_valkyrie_100k_loop_team3_no_ssbx.py`

