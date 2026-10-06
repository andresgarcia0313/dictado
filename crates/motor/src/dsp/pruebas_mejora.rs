//! Pruebas de la cadena completa `Mejora` (VAD, compuerta, SNR y costo).
use super::pruebas::bloque;
use super::Mejora;
use crate::audio::rms;

#[test]
fn el_silencio_no_se_amplifica_hasta_nivel_de_voz() {
    // el ruido de fondo amplificado debe quedar muy por debajo de -34 dBFS (0.02).
    let mut m = Mejora::nueva(true, false);
    for i in 0..80 {
        m.procesar(&bloque(-35.0, i * 1600));
    }
    let sal = m.procesar(&bloque(-55.0, 0));
    assert!(rms(&sal) < 0.01, "{}", rms(&sal));
}

#[test]
fn mejora_apagada_no_toca_el_audio() {
    let mut m = Mejora::nueva(false, false);
    let x = bloque(-30.0, 0);
    assert_eq!(m.procesar(&x), x);
    assert_eq!(m.ganancia_db(), 0.0);
}

#[test]
fn compuerta_sobre_audio_limpio_solo_retrasa_512_muestras() {
    let mut m = Mejora::nueva(false, true);
    let (mut ent, mut sal) = (vec![], vec![]);
    for i in 0..30 {
        let x = bloque(-20.0, i * 1600);
        sal.extend(m.procesar(&x));
        ent.extend(x);
    }
    assert_eq!(sal.len(), ent.len());
    let err = (4000..ent.len() - 512)
        .map(|i| (sal[i + 512] - ent[i]).abs())
        .fold(0.0f32, f32::max);
    assert!(err < 1e-3, "error {err}");
}

#[test]
fn snr_medido_coincide_con_la_construccion() {
    use super::medida::Medida;
    let mut med = Medida::nueva();
    let (voz, ruido) = (vec![0.1f32; 256], vec![0.01f32; 256]); // potencias 1e-2 y 1e-4: 20 dB de diferencia
    assert_eq!(med.snr(), None);
    for _ in 0..40 {
        med.trama(&ruido, 0.0);
        med.trama(&voz, 0.9);
    }
    let snr = med.snr().unwrap();
    assert!((snr - 20.0).abs() < 0.5, "snr {snr}");
}

/// Costo del DSP completo por segundo de audio: `cargo test --release -p dictado-motor costo_dsp -- --ignored --nocapture`.
#[test]
#[ignore]
fn costo_dsp() {
    for (nombre, ruido) in [("vad+agc", false), ("vad+agc+compuerta", true)] {
        let mut m = Mejora::nueva(true, ruido);
        let t = std::time::Instant::now();
        for i in 0..600 {
            m.procesar(&bloque(-30.0, i * 1600));
        }
        let cpu = t.elapsed().as_secs_f64() / 60.0 * 100.0;
        println!("COSTO {nombre}: {cpu:.2} % de un nucleo (60 s de audio)");
    }
}

#[test]
fn expone_una_p_por_trama_completada() {
    let mut m = Mejora::nueva(true, false);
    m.procesar(&bloque(-30.0, 0)[..1024]);
    assert_eq!(m.ps().len(), 4);
    assert!(m.ps().iter().all(|p| (0.0..=1.0).contains(p)));
    m.procesar(&[0.0; 100]);
    assert!(m.ps().is_empty(), "sin trama completa no hay p nueva");
}
