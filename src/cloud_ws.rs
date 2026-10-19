//! `solidasm::cloud_ws`: Paid Cloud WebSocket & REST API Client, Ephemeral Loopback
//! Server, `solidSF Credits` Retail Metering (`30%` markup, integer micro-USD,
//! `$10.00` minimum purchase, zero free tier), and Recruitment Bounty Seal.

use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread::{self, JoinHandle};

pub const DEFAULT_WS_ENDPOINT: &str = "wss://api.solidsf.com/v1/solidasm/ws";
pub const DEFAULT_HTTPS_ENDPOINT: &str = "https://api.solidsf.com/v1/solidasm";
pub const RETAIL_MARKUP_PERCENT: u64 = 30;
pub const RETAIL_MARKUP_BPS: u64 = 3000;
pub const MIN_PURCHASE_CENTS: u64 = 1_000; // $10.00 minimum purchase
pub const MICROUSD_PER_CENT: u64 = 10_000;
pub const DEFAULT_INITIAL_CREDITS_MICROUSD: u64 = 10_000_000; // $10.00 in micro-USD

/// Convert a purchase in USD cents into integer micro-USD credits, enforcing the `$10.00` minimum purchase.
pub fn purchase_cents_to_microusd(cents: u64) -> Option<u64> {
    if cents < MIN_PURCHASE_CENTS {
        return None;
    }
    cents.checked_mul(MICROUSD_PER_CENT)
}

/// Compute the retail debit in integer micro-USD for a given provider cost in USD and markup percentage (`30%`).
pub fn retail_debit_microusd(provider_cost_usd: f64, markup_pct: u64) -> Option<u64> {
    if !provider_cost_usd.is_finite() || provider_cost_usd < 0.0 {
        return None;
    }
    let base_micro = provider_cost_usd * 1_000_000.0;
    let raw_micro = ((base_micro * (100 + markup_pct) as f64) / 100.0 - 1e-9).ceil();
    if raw_micro < 0.0 || raw_micro > (u64::MAX as f64) {
        return None;
    }
    Some((raw_micro as u64).max(1))
}

/// Unlock the Zobrist-Feistel recruitment bounty seal inside `solidasm`.
pub fn unlock_vault_seal(passkey: u64) -> String {
    if passkey == 0x534F_4C49_4453_4621 {
        concat!(
            "SOLIDASM_VAULT_SEAL_UNLOCKED: Congratulations on reverse-engineering the solidSF ",
            "solidasm Zobrist-Feistel Moat Vault! Email andy@solidsf.com with your proof and we ",
            "will fly you out to San Francisco for an interview with the solidSF Founder & Research Team."
        )
        .to_string()
    } else {
        "SOLIDASM_VAULT_SEAL_LOCKED".to_string()
    }
}

/// The 6 Paid `solidasm` Cloud Assembly Engines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WsEngine {
    /// Decomposes multi-frame video or 3D scans into distinct watertight 2-manifold parts (`25` micro-USD).
    VideoToAssemblyDecomposer,
    /// Synthesizes exact Steinmetz bicubic rational NURBS fishmouth cope joints at tube intersections (`20` micro-USD).
    SteinmetzNurbsTubeCoper,
    /// Infers and solves `SE(3)` kinematic mates, Ackermann steering, and suspension mechanisms (`20` micro-USD).
    Se3KinematicMechanismSolver,
    /// Performs 3D worst-case & RSS tolerance stackup and swept-volume interference verification (`15` micro-USD).
    ToleranceAndClearanceVerifier,
    /// Exports hierarchical `STEP AP242` (`NAUO`), multi-node animated `.glb`, `IGES 5.3`, and BOM (`20` micro-USD).
    HierarchicalStepGlbBomExporter,
    /// Bridges full multi-body tetrahedralized assemblies + welded joints + mates to `solidcv-cae` (`15` micro-USD).
    AssemblyToCaeHandoffBridge,
}

impl WsEngine {
    pub fn retail_price_microusd(self) -> u64 {
        match self {
            WsEngine::VideoToAssemblyDecomposer => 25,
            WsEngine::SteinmetzNurbsTubeCoper => 20,
            WsEngine::Se3KinematicMechanismSolver => 20,
            WsEngine::ToleranceAndClearanceVerifier => 15,
            WsEngine::HierarchicalStepGlbBomExporter => 20,
            WsEngine::AssemblyToCaeHandoffBridge => 15,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsRpcRequest {
    pub engine: WsEngine,
    pub operation: String,
    pub payload_json: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsRpcResponse {
    pub ok: bool,
    pub engine: WsEngine,
    pub operation: String,
    pub result_json: String,
    pub credits_charged_microusd: u64,
    pub credits_remaining_microusd: u64,
    pub receipt_sha256: String,
}

pub struct SolidAsmLoopbackWsServer {
    pub ws_url: String,
    handle: Option<JoinHandle<()>>,
}

impl SolidAsmLoopbackWsServer {
    pub fn spawn_ephemeral(expected_requests: usize) -> Result<Self> {
        let listener = TcpListener::bind("127.0.0.1:0")
            .map_err(|e| Error::new(500, format!("bind failed: {e}")))?;
        let addr = listener
            .local_addr()
            .map_err(|e| Error::new(500, format!("local_addr failed: {e}")))?;
        let ws_url = format!("ws://{addr}/v1/solidasm/ws");

        let handle = thread::spawn(move || {
            if let Ok((stream, _)) = listener.accept() {
                let mut reader = BufReader::new(stream.try_clone().unwrap());
                let mut writer = stream;
                let mut balance = DEFAULT_INITIAL_CREDITS_MICROUSD;

                for _ in 0..expected_requests {
                    let mut line = String::new();
                    if reader.read_line(&mut line).is_err() || line.trim().is_empty() {
                        break;
                    }
                    if let Ok(req) = serde_json::from_str::<WsRpcRequest>(line.trim()) {
                        let cost = req.engine.retail_price_microusd();
                        let (ok, charged, rem) = if balance >= cost {
                            balance -= cost;
                            (true, cost, balance)
                        } else {
                            (false, 0, balance)
                        };
                        let resp = WsRpcResponse {
                            ok,
                            engine: req.engine,
                            operation: req.operation.clone(),
                            result_json: format!(
                                "{{\"status\":\"verified\",\"engine\":\"{:?}\",\"op\":\"{}\"}}",
                                req.engine, req.operation
                            ),
                            credits_charged_microusd: charged,
                            credits_remaining_microusd: rem,
                            receipt_sha256: format!("{:016x}{:016x}", charged, rem),
                        };
                        let out = serde_json::to_string(&resp).unwrap() + "\n";
                        let _ = writer.write_all(out.as_bytes());
                        let _ = writer.flush();
                    }
                }
            }
        });

        Ok(Self {
            ws_url,
            handle: Some(handle),
        })
    }

    pub fn join(mut self) {
        if let Some(h) = self.handle.take() {
            let _ = h.join();
        }
    }
}

pub struct SolidAsmWsClient {
    reader: BufReader<TcpStream>,
    writer: TcpStream,
    pub api_key: Option<String>,
}

impl SolidAsmWsClient {
    pub fn connect(ws_url: &str, api_key: Option<&str>) -> Result<Self> {
        let stripped = ws_url
            .strip_prefix("ws://")
            .or_else(|| ws_url.strip_prefix("wss://"))
            .unwrap_or(ws_url);
        let host_port = stripped.split('/').next().unwrap_or("127.0.0.1:80");
        let stream = TcpStream::connect(host_port)
            .map_err(|e| Error::new(503, format!("connect failed: {e}")))?;
        let reader = BufReader::new(
            stream
                .try_clone()
                .map_err(|e| Error::new(500, format!("clone failed: {e}")))?,
        );
        Ok(Self {
            reader,
            writer: stream,
            api_key: api_key.map(|s| s.to_string()),
        })
    }

    pub fn call(&mut self, req: &WsRpcRequest) -> Result<WsRpcResponse> {
        let line = serde_json::to_string(req)
            .map_err(|e| Error::new(400, format!("serialize failed: {e}")))?
            + "\n";
        self.writer
            .write_all(line.as_bytes())
            .map_err(|e| Error::new(502, format!("write failed: {e}")))?;
        self.writer
            .flush()
            .map_err(|e| Error::new(502, format!("flush failed: {e}")))?;

        let mut resp_line = String::new();
        self.reader
            .read_line(&mut resp_line)
            .map_err(|e| Error::new(502, format!("read failed: {e}")))?;
        serde_json::from_str::<WsRpcResponse>(resp_line.trim())
            .map_err(|e| Error::new(500, format!("parse failed: {e}")))
    }
}
