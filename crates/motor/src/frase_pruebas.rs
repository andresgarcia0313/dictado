use super::*;
use crate::dsp::TRAMA;

struct Falso {
    alimentado: usize,
    cierres: u32,
}

impl Reconocedor for Falso {
    fn alimentar(&mut self, m: &[f32]) {
        self.alimentado += m.len();
    }
    fn texto(&self) -> String {
        "hola".into()
    }
    fn fin_de_frase(&self) -> bool {
        false
    }
    fn cerrar(&mut self) -> String {
        self.cierres += 1;
        "hola".into()
    }
}

fn tramas(rec: &mut Falso, f: &mut Frase, p: f32, n: usize) {
    f.procesar(rec, &vec![0.0; n * TRAMA], &vec![p; n]);
}

#[test]
fn capitaliza() {
    assert_eq!(capitalizar("ñandú rápido"), "Ñandú rápido");
    assert_eq!(capitalizar(""), "");
}

#[test]
fn no_decodifica_silencio_y_agrega_relleno_previo() {
    let (mut r, mut f) = (
        Falso {
            alimentado: 0,
            cierres: 0,
        },
        Frase::nueva(),
    );
    tramas(&mut r, &mut f, 0.0, 50);
    assert_eq!(r.alimentado, 0);
    tramas(&mut r, &mut f, 0.9, 1);
    assert_eq!(
        r.alimentado,
        1600 + TRAMA,
        "100 ms previos + la trama de entrada"
    );
}

#[test]
fn cierra_tras_el_hangover_de_400_ms() {
    let (mut r, mut f) = (
        Falso {
            alimentado: 0,
            cierres: 0,
        },
        Frase::nueva(),
    );
    tramas(&mut r, &mut f, 0.9, 30);
    tramas(&mut r, &mut f, 0.0, 24);
    assert_eq!(r.cierres, 0);
    tramas(&mut r, &mut f, 0.0, 1);
    assert_eq!(r.cierres, 1);
}

#[test]
fn voz_menor_de_250_ms_se_descarta_sin_final() {
    let (mut r, mut f) = (
        Falso {
            alimentado: 0,
            cierres: 0,
        },
        Frase::nueva(),
    );
    tramas(&mut r, &mut f, 0.9, 8);
    tramas(&mut r, &mut f, 0.0, 30);
    assert_eq!(r.cierres, 1, "el reconocedor se reinicia igual");
    assert!(!f.hist.es_valido());
}

#[test]
fn quita_puntuacion_inicial() {
    assert_eq!(
        super::cierre::limpiar(". No hay problema"),
        "No hay problema"
    );
    assert_eq!(super::cierre::limpiar(", ¿Listo?"), "¿Listo?");
    assert_eq!(super::cierre::limpiar("hola."), "hola.");
}
