use quote::{format_ident, quote};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    truce_build::emit_plugin_env();

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let dest_path = out_dir.join("samples_data.rs");
    let samples_root = Path::new("samples");

    let mut kits = BTreeSet::new();
    let mut sample_names = BTreeSet::new();

    // --- データ収集 ---
    if let Ok(entries) = fs::read_dir(samples_root) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                let kit_name = entry.file_name().into_string().unwrap();
                kits.insert(kit_name);
                if let Ok(sample_entries) = fs::read_dir(entry.path()) {
                    for sample_entry in sample_entries.flatten() {
                        if sample_entry
                            .path()
                            .extension()
                            .is_some_and(|ext| ext == "wav")
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

    // --- 構造体定義のトークン生成 ---
    let sample_idents: Vec<_> = sample_names
        .iter()
        .map(|n| format_ident!("{}", n))
        .collect();
    let kit_idents: Vec<_> = kits.iter().map(|k| format_ident!("{}", k)).collect();

    let struct_definitions = quote! {
        pub struct DrumKitData {
            #( pub #sample_idents: &'static [f32], )*
        }

        #[repr(C, align(4))]
        pub struct DrumsSamplesData {
            #( pub #kit_idents: DrumKitData, )*
        }
    };

    // --- 実体データのトークン生成 ---
    let mut kit_fields = Vec::new();

    for kit in &kits {
        let mut sample_fields = Vec::new();
        for name in &sample_names {
            let field_ident = format_ident!("{}", name);
            let wav_path = samples_root.join(kit).join(format!("{}.wav", name));

            if wav_path.exists() {
                let mut reader = hound::WavReader::open(&wav_path).expect("Wav open error");
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

                // 無音カット（ロジックはそのまま維持）
                let threshold = 1e-6;
                let silence_limit = 1024;
                let mut last_active_index = 0;
                let mut consecutive_silence = 0;
                for (i, &sample) in samples.iter().enumerate() {
                    if sample.abs() > threshold {
                        last_active_index = i;
                        consecutive_silence = 0;
                    } else {
                        consecutive_silence += 1;
                        if consecutive_silence >= silence_limit {
                            break;
                        }
                    }
                }
                if last_active_index == 0 && samples.first().is_none_or(|&s| s.abs() <= threshold) {
                    samples.clear();
                } else {
                    samples.truncate(last_active_index + 1);
                }

                // バイナリ書き出し (ここは従来通り。build.rs自身が書き込むためには絶対パスが必要)
                let bin_filename = format!("{}_{}.bin", kit, name);
                let bin_path = out_dir.join(&bin_filename);
                let bytes: &[u8] = unsafe {
                    std::slice::from_raw_parts(samples.as_ptr() as *const u8, samples.len() * 4)
                };
                fs::write(&bin_path, bytes).unwrap();

                // --- ここから修正 ---
                let sample_count = samples.len();

                // 生成されるソースコード内で env!("OUT_DIR") を使わせる
                sample_fields.push(quote! {
                    #field_ident: unsafe {
                        #[repr(C, align(4))]
                        struct Aligned<const N: usize>([u8; N]);

                        // コンパイル時に環境変数からパスを組み立てるように出力
                        static ALIGNED: &Aligned<{ include_bytes!(concat!(env!("OUT_DIR"), "/", #bin_filename)).len() }> =
                            &Aligned(*include_bytes!(concat!(env!("OUT_DIR"), "/", #bin_filename)));

                        std::slice::from_raw_parts(ALIGNED.0.as_ptr() as *const f32, #sample_count)
                    }
                });
            } else {
                sample_fields.push(quote! { #field_ident: &[] });
            }
        }

        let kit_ident = format_ident!("{}", kit);
        kit_fields.push(quote! {
            #kit_ident: DrumKitData {
                #(#sample_fields,)*
            }
        });
    }

    // --- 全体の組み立て ---
    let final_token_stream = quote! {
        #struct_definitions

        pub const SAMPLES: DrumsSamplesData = DrumsSamplesData {
            #(#kit_fields,)*
        };
    };

    // --- Pretty Print & Write ---
    let syntax_tree = syn::parse2(final_token_stream).unwrap();
    let formatted_code = prettyplease::unparse(&syntax_tree);

    fs::write(&dest_path, formatted_code).unwrap();

    println!("cargo:rerun-if-changed=samples");
    println!("cargo:rerun-if-changed=build.rs");
}
