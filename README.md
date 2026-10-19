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
