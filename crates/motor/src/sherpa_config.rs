//! Configuracion del reconocedor sherpa-onnx: zipformer transducer streaming, busqueda greedy, endpoint.
use sherpa_onnx::{OnlineModelConfig, OnlineRecognizerConfig, OnlineTransducerModelConfig};
use std::path::Path;

pub const HILOS: i32 = 2;

fn ruta(dir: &Path, f: &str) -> Option<String> {
    Some(dir.join(f).to_string_lossy().into_owned())
}

pub fn configuracion(dir: &Path) -> OnlineRecognizerConfig {
    OnlineRecognizerConfig {
        model_config: OnlineModelConfig {
            transducer: OnlineTransducerModelConfig {
                encoder: ruta(dir, "encoder.onnx"),
                decoder: ruta(dir, "decoder.onnx"),
                joiner: ruta(dir, "joiner.onnx"),
            },
            tokens: ruta(dir, "tokens.txt"),
            num_threads: HILOS,
            ..Default::default()
        },
        decoding_method: Some("greedy_search".into()),
        enable_endpoint: true,
        rule1_min_trailing_silence: 2.4,
        rule2_min_trailing_silence: 0.9,
        rule3_min_utterance_length: 20.0,
        ..Default::default()
    }
}
