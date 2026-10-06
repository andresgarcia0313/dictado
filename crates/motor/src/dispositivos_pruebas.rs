use super::*;

#[test]
fn listar_no_entra_en_panico_y_no_repite() {
    let l = listar();
    eprintln!("microfonos: {l:?}");
    let mut u = l.clone();
    u.sort();
    u.dedup();
    assert_eq!(u.len(), l.len());
}

#[test]
fn las_salidas_no_se_ofrecen_como_microfono() {
    for n in [
        "Monitor Output",
        "ALSA PLAYBACK",
        "Discard all samples",
        "Speaker output 2",
    ] {
        assert!(es_salida(n), "{n}");
    }
    assert!(!es_salida("Built-in Audio Analog Stereo") && !es_salida("Default Audio Device"));
}

#[test]
fn nombre_inexistente_da_error() {
    assert!(buscar(Some("no-existe-xyz")).is_err());
}

#[test]
fn auto_equivale_al_primero() {
    let a = buscar(Some("auto")).map(|x| x.0);
    let b = buscar(None).map(|x| x.0);
    assert_eq!(a, b);
}
