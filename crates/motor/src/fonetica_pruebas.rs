use super::*;

#[test]
fn claves_del_espanol() {
    assert_eq!(clave("cu a"), clave("cua"));
    assert_eq!(clave("Vaca"), clave("baca"));
    assert_eq!(clave("hola"), "ola");
    assert_eq!(clave("queso"), "keso");
    assert_eq!(clave("cena"), clave("sena"));
    assert_eq!(clave("gente"), "jente");
    assert_eq!(clave("guerra"), "gera");
    assert_eq!(clave("llave"), "yabe");
}

#[test]
fn normaliza_tildes_y_signos() {
    assert_eq!(normalizar("Cuá, ¡Ya!"), "cuaya");
}

#[test]
fn jaro_winkler_conocidos() {
    assert!((jaro_winkler("martha", "marhta") - 0.961).abs() < 0.01);
    assert_eq!(jaro_winkler("abc", "abc"), 1.0);
    assert_eq!(jaro_winkler("abc", "xyz"), 0.0);
    assert_eq!(jaro_winkler("", ""), 1.0);
}
