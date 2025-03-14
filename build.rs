fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
         .out_dir("src/pb")
        .compile_protos(
            &["proto/docx_service.proto"],
            &["proto"],
        )?;
    Ok(())
}
