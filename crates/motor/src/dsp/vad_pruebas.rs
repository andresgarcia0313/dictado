use super::*;

#[test]
fn earshot_separa_silencio_de_ruido_fuerte_sin_romperse() {
    let mut v = neuronal().expect("earshot");
    let p = (0..40).map(|_| v.p(&[0.0; TRAMA])).fold(0.0f32, f32::max);
    assert!(p < 0.5, "silencio digital p={p}");
    assert!(v.p(&[9.0; TRAMA]).is_finite()); // fuera de rango se recorta
}

#[test]
fn energia_distingue_nivel() {
    let mut v = Energia::nuevo();
    assert_eq!(v.p(&[0.001; TRAMA]), 0.0);
    assert_eq!(v.p(&[0.3; TRAMA]), 1.0);
}
