//! Example: 18-Part, 1,934-Feature UCI 56cm Aero Carbon Road Bicycle
//! (`SWorks_Look795_Aero_RoadBike_DuraAce_R9270`) WS-RPC Part Finalization & Live DAG Mutation.

use solidasm::roadbike_dag::execute_roadbike_rpc_finalization_and_mutation;

fn main() {
    let bundle = execute_roadbike_rpc_finalization_and_mutation()
        .expect("execute_roadbike_rpc_finalization_and_mutation");
    println!(
        "ROADBIKE RPC FINALIZED: {} parts, {} editable features, {} RPC calls ({} uUSD), STEP={} B ({}), GLB={} B ({}), Rev-A->Rev-B 52T->54T verified ({} -> {} verts)",
        bundle.tree_doc.total_parts,
        bundle.tree_doc.total_feature_nodes,
        bundle.total_rpc_calls,
        bundle.total_credits_billed_microusd,
        bundle.step_ap242.len(),
        &bundle.step_sha256[..12],
        bundle.glb_bytes.len(),
        &bundle.glb_sha256[..12],
        bundle.rev_a_crank_vertices,
        bundle.rev_b_crank_vertices
    );
}
