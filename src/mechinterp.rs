//! `solidasm::mechinterp`: First-Class Mechanistic Interpretability (`mechinterp`),
//! 64D L2-Normalized Glass-Box Assembly Signatures, 256-Bit Binary SAE Assembly
//! Circuit Matching, and Counterfactual Ablation Hooks.

use crate::{Result, SolidAsmBrepSolid};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AblationConfig {
    pub disable_oh_symmetry_completion: bool,
    pub disable_steinmetz_fishmouth_coping: bool,
    pub disable_ackermann_caster_constraint: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MindSummary {
    pub total_events: usize,
    pub last_theorem: String,
    pub active_ablation: AblationConfig,
}

static MIND_STATE: Mutex<MindSummary> = Mutex::new(MindSummary {
    total_events: 0,
    last_theorem: String::new(),
    active_ablation: AblationConfig {
        disable_oh_symmetry_completion: false,
        disable_steinmetz_fishmouth_coping: false,
        disable_ackermann_caster_constraint: false,
    },
});

pub fn reset_mind() {
    if let Ok(mut g) = MIND_STATE.lock() {
        g.total_events = 0;
        g.last_theorem.clear();
        g.active_ablation = AblationConfig::default();
    }
}

pub fn record_theorem_event(theorem_id: &str) {
    if let Ok(mut g) = MIND_STATE.lock() {
        g.total_events += 1;
        g.last_theorem = theorem_id.to_string();
    }
}

pub fn current_ablation() -> AblationConfig {
    MIND_STATE
        .lock()
        .map(|g| g.active_ablation.clone())
        .unwrap_or_default()
}

pub fn with_counterfactual_ablation<F, R>(cfg: AblationConfig, f: F) -> (R, MindSummary)
where
    F: FnOnce() -> R,
{
    if let Ok(mut g) = MIND_STATE.lock() {
        g.active_ablation = cfg;
    }
    let out = f();
    let summary = MIND_STATE
        .lock()
        .map(|mut g| {
            let s = g.clone();
            g.active_ablation = AblationConfig::default();
            s
        })
        .unwrap_or_default();
    (out, summary)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlassBoxAssemblySignature64D {
    pub vector: Vec<f32>,
}

impl GlassBoxAssemblySignature64D {
    pub fn from_brep(brep: &SolidAsmBrepSolid) -> Result<Self> {
        let mut raw = vec![0.0f32; 64];
        raw[0] = brep.vertex_count as f32;
        raw[1] = brep.edge_count as f32;
        raw[2] = brep.face_count as f32;
        raw[3] = brep.euler_characteristic as f32;
        raw[4] = if brep.is_closed_manifold { 1.0 } else { 0.0 };
        raw[5] = brep.volume_m3 as f32;
        raw[6] = brep.surface_area_m2 as f32;
        for (i, v) in brep.vertices.iter().take(19).enumerate() {
            raw[7 + i * 3] = v[0] as f32;
            raw[7 + i * 3 + 1] = v[1] as f32;
            raw[7 + i * 3 + 2] = v[2] as f32;
        }
        let norm = raw
            .iter()
            .map(|&x| (x as f64) * (x as f64))
            .sum::<f64>()
            .sqrt()
            .max(1e-12);
        for x in &mut raw {
            *x = (*x as f64 / norm) as f32;
        }
        Ok(Self { vector: raw })
    }

    pub fn cosine_similarity(&self, other: &Self) -> f32 {
        self.vector
            .iter()
            .zip(other.vector.iter())
            .map(|(a, b)| a * b)
            .sum()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaeAssemblyBinaryAtom {
    pub atom_id: usize,
    pub label: String,
    pub bits: [u64; 4],
}

impl SaeAssemblyBinaryAtom {
    pub fn from_f32_slice(atom_id: usize, label: &str, values: &[f32], threshold: f32) -> Self {
        let mut bits = [0u64; 4];
        for (i, &v) in values.iter().take(256).enumerate() {
            if v > threshold {
                bits[i / 64] |= 1u64 << (i % 64);
            }
        }
        Self {
            atom_id,
            label: label.to_string(),
            bits,
        }
    }

    pub fn hamming_distance(&self, other: &Self) -> u32 {
        (0..4)
            .map(|k| (self.bits[k] ^ other.bits[k]).count_ones())
            .sum()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitMatchReceipt {
    pub best_atom_id: usize,
    pub best_label: String,
    pub hamming_distance: u32,
}

pub struct SaeAssemblyCircuitMatcher {
    atoms: Vec<SaeAssemblyBinaryAtom>,
}

impl SaeAssemblyCircuitMatcher {
    pub fn new(atoms: Vec<SaeAssemblyBinaryAtom>) -> Self {
        Self { atoms }
    }

    pub fn match_residual_batch(&self, queries: &[SaeAssemblyBinaryAtom]) -> Vec<CircuitMatchReceipt> {
        queries
            .iter()
            .map(|q| {
                let mut best_id = 0usize;
                let mut best_label = String::new();
                let mut best_dist = u32::MAX;
                for a in &self.atoms {
                    let d = q.hamming_distance(a);
                    if d < best_dist {
                        best_dist = d;
                        best_id = a.atom_id;
                        best_label = a.label.clone();
                    }
                }
                CircuitMatchReceipt {
                    best_atom_id: best_id,
                    best_label,
                    hamming_distance: best_dist,
                }
            })
            .collect()
    }
}
