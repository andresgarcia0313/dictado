use super::*;
use crate::canal_regla::*;

#[test]
fn canal_muerto_usa_solo_el_fuerte() {
    assert_eq!(
        decidir(Modo::Promedio, &[0.04, 0.0009], &[0.3]),
        Modo::Solo(0)
    );
    assert_eq!(
        decidir(Modo::Promedio, &[0.001, 0.05], &[0.3]),
        Modo::Solo(1)
    );
}

#[test]
fn parecidos_y_correlados_promedian() {
    assert_eq!(
        decidir(Modo::Promedio, &[0.04, 0.035], &[0.9]),
        Modo::Promedio
    );
    assert_eq!(
        decidir(Modo::Solo(0), &[0.04, 0.035], &[0.9]),
        Modo::Promedio
    );
}

#[test]
fn histeresis_no_salta_en_la_zona_media() {
    // brecha de 8 dB: ni entra al modo solo desde promedio correlado ni vuelve al promedio
    assert_eq!(
        decidir(Modo::Promedio, &[0.04, 0.016], &[0.9]),
        Modo::Promedio
    );
    assert_eq!(
        decidir(Modo::Solo(0), &[0.04, 0.016], &[0.9]),
        Modo::Solo(0)
    );
    // cambio de canal solo con más de 3 dB de ventaja
    assert_eq!(decidir(Modo::Solo(0), &[0.04, 0.05], &[0.2]), Modo::Solo(0));
    assert_eq!(decidir(Modo::Solo(0), &[0.02, 0.05], &[0.2]), Modo::Solo(1));
}

#[test]
fn silencio_conserva_el_modo() {
    assert_eq!(decidir(Modo::Solo(1), &[0.0, 0.0], &[0.0]), Modo::Solo(1));
}

#[test]
fn procesa_estereo_con_canal_derecho_mudo() {
    let mut c = Canales::nuevo(2, 1000);
    let bloque: Vec<f32> = (0..1000)
        .flat_map(|i| [(i as f32 * 0.3).sin() * 0.1, 0.0])
        .collect();
    let primero = c.procesar(&bloque);
    assert!(
        (primero[10] - (10.0f32 * 0.3).sin() * 0.05).abs() < 1e-6,
        "al inicio promedia"
    );
    let segundo = c.procesar(&bloque);
    assert!(
        (segundo[10] - (10.0f32 * 0.3).sin() * 0.1).abs() < 1e-6,
        "luego usa solo el fuerte"
    );
}

#[test]
fn mono_pasa_igual() {
    assert_eq!(
        Canales::nuevo(1, 16_000).procesar(&[0.1, 0.2]),
        vec![0.1, 0.2]
    );
}
