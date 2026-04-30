use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    truce_build::emit_plugin_env();

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("samples_data.rs");

    let samples_root = Path::new("samples");

    // Discover all kits and sample names
    let mut kits = BTreeSet::new();
    let mut sample_names = BTreeSet::new();

    if let Ok(entries) = fs::read_dir(samples_root) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                let kit_name = entry.file_name().into_string().unwrap();
                kits.insert(kit_name.clone());

                if let Ok(sample_entries) = fs::read_dir(entry.path()) {
                    for sample_entry in sample_entries.flatten() {
                        if sample_entry
                            .path()
                            .extension()
                            .map_or(false, |ext| ext == "wav")
                        {
                            let stem = sample_entry
                                .path()
                                .file_stem()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_string();
                            sample_names.insert(stem);
                        }
                    }
                }
            }
        }
    }

    let mut code = String::new();

    // Generate DrumKitData struct
    code.push_str("pub struct DrumKitData {\n");
    for name in &sample_names {
        code.push_str(&format!("    pub {}: &'static [f32],\n", name));
    }
    code.push_str("}\n\n");

    // Generate DrumsSamplesData struct
    code.push_str("pub struct DrumsSamplesData {\n");
    for kit in &kits {
        code.push_str(&format!("    pub {}: DrumKitData,\n", kit));
    }
    code.push_str("}\n\n");

    // Generate SAMPLES constant
    code.push_str("pub const SAMPLES: DrumsSamplesData = DrumsSamplesData {\n");
    for kit in &kits {
        code.push_str(&format!("    {}: DrumKitData {{\n", kit));
        for name in &sample_names {
            let wav_path = samples_root.join(kit).join(format!("{}.wav", name));
            if wav_path.exists() {
                let mut reader = hound::WavReader::open(&wav_path)
                    .expect(&format!("Failed to open {:?}", wav_path));
                let spec = reader.spec();
                let mut samples: Vec<f32> = match spec.sample_format {
                    hound::SampleFormat::Float => {
                        reader.samples::<f32>().map(|s| s.unwrap()).collect()
                    }
                    hound::SampleFormat::Int => {
                        let max_val = (1 << (spec.bits_per_sample - 1)) as f32;
                        reader
                            .samples::<i32>()
                            .map(|s| s.unwrap() as f32 / max_val)
                            .collect()
                    }
                };

                // Enhanced trimming: handle -0.0 and extremely small values
                // We search for the last position where the absolute value is significantly above zero.
                if let Some(last_pos) = samples.iter().rposition(|&s| s.abs() > 1e-6) {
                    let end = (last_pos + 1).saturating_sub(100);
                    samples.truncate(end);
                } else {
                    // If the entire sample is silence, empty it
                    samples.clear();
                }

                code.push_str(&format!("        {}: &[\n", name));
                for s in samples {
                    code.push_str(&format!("            {:?},\n", s));
                }
                code.push_str("        ],\n");
            } else {
                // If a sample is missing in a kit, provide an empty slice
                code.push_str(&format!("        {}: &[],\n", name));
            }
        }
        code.push_str("    },\n");
    }
    code.push_str("};\n");

    fs::write(&dest_path, code).unwrap();
    println!("cargo:rerun-if-changed=samples");
    println!("cargo:rerun-if-changed=build.rs");
}
