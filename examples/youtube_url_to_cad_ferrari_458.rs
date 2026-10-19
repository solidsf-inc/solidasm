use solidasm::youtube_lod::{reconstruct_youtube_url_to_cad, SolidAsmLevelOfDetail};
use std::fs;
use std::path::PathBuf;

fn main() {
    let url = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "https://www.youtube.com/watch?v=vWIJdsYnO9E".to_string());
    let out_dir = PathBuf::from(
        std::env::args()
            .nth(2)
            .unwrap_or_else(|| "/Users/stu/solidsf-scratch/solidcv-cad-ferrari-458".to_string()),
    );
    fs::create_dir_all(&out_dir).expect("create output directory");

    for lod in [
        SolidAsmLevelOfDetail::Toy,
        SolidAsmLevelOfDetail::Fast,
        SolidAsmLevelOfDetail::Deep,
        SolidAsmLevelOfDetail::Max,
    ] {
        let bundle = reconstruct_youtube_url_to_cad(&url, lod).expect("reconstruct YouTube URL");
        println!(
            "[LOD={:<4} | {:<34}] parts={:2} features={:4} rpc_calls={:2} ready_2d={:<5} score={:.2} sheets={:2} step={}B glb={}B",
            bundle.level_of_detail,
            bundle.lod_tier_id,
            bundle.tree_doc.total_parts,
            bundle.tree_doc.total_feature_nodes,
            bundle.total_rpc_calls,
            bundle.model_self_audit.ready_for_2d_drawing_package,
            bundle.model_self_audit.drawing_package_readiness_score,
            bundle.drawing_2d_package.total_sheets,
            bundle.step_ap242.len(),
            bundle.glb_bytes.len()
        );

        let prefix = format!("Ferrari_458_Italia_vWIJdsYnO9E_{}", lod.as_str());
        if lod == SolidAsmLevelOfDetail::Max {
            fs::write(out_dir.join(format!("{}.step", prefix)), &bundle.step_ap242).unwrap();
            fs::write(out_dir.join(format!("{}.glb", prefix)), &bundle.glb_bytes).unwrap();
            fs::write(out_dir.join(format!("{}.igs", prefix)), &bundle.iges_5_3).unwrap();
            fs::write(
                out_dir.join(format!("{}_editable_feature_tree.json", prefix)),
                serde_json::to_string_pretty(&bundle.tree_doc).unwrap(),
            )
            .unwrap();
            fs::write(
                out_dir.join(format!("{}_model_self_audit.json", prefix)),
                serde_json::to_string_pretty(&bundle.model_self_audit).unwrap(),
            )
            .unwrap();
            fs::write(
                out_dir.join(format!("{}_drawing_2d_package.json", prefix)),
                serde_json::to_string_pretty(&bundle.drawing_2d_package).unwrap(),
            )
            .unwrap();
            fs::write(
                out_dir.join(format!("{}_bom.json", prefix)),
                serde_json::to_string_pretty(&bundle.bom).unwrap(),
            )
            .unwrap();
            fs::write(
                out_dir.join(format!("{}_tolerance_stackup.json", prefix)),
                serde_json::to_string_pretty(&bundle.tolerance_stackup).unwrap(),
            )
            .unwrap();
            fs::write(
                out_dir.join(format!("{}_rpc_receipt.json", prefix)),
                serde_json::to_string_pretty(&bundle.rpc_calls).unwrap(),
            )
            .unwrap();
        }
    }
}
