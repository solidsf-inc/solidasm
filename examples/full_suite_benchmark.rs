use solidasm::{
    cloud_ws, evolve_assembly_skills_recursively, synthesize_vehicle_assembly,
    YOUTUBE_GOKART_DUNE_BUGGY_CORPUS,
};

fn main() {
    let asm = synthesize_vehicle_assembly(&YOUTUBE_GOKART_DUNE_BUGGY_CORPUS[0], 4);
    let report = evolve_assembly_skills_recursively();
    println!(
        "solidasm benchmark: parts={} fishmouth_joints={} mates={} dof={} mass={:.2}kg rss_tol={:.4}mm step={}B glb={}B corpus={} fitness_ascent={} ops_descent={} min_topup_microusd={}",
        asm.part_count,
        asm.fishmouth_joints.len(),
        asm.kinematic_mates.len(),
        asm.total_mobility_dof,
        asm.bom.total_mass_kg,
        asm.tolerance_stackup.rss_3sigma_stackup_mm,
        asm.step_ap242_assembly.len(),
        asm.glb_assembly_bytes.len(),
        report.corpus_video_count,
        report.strict_fitness_ascent,
        report.strict_ops_descent,
        cloud_ws::purchase_cents_to_microusd(1_000).unwrap()
    );
}
