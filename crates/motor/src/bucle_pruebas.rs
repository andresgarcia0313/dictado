use super::*;
use crate::dsp::{Vad, TRAMA};
use crate::fuente::{Fuente, Lectura};
use std::sync::atomic::{AtomicBool, Ordering::SeqCst};
use std::sync::mpsc::channel;
use std::time::Instant;

struct Siempre;
impl Vad for Siempre {
    fn p(&mut self, _: &[f32]) -> f32 {
        0.95
    }
}

static SOLTADA: AtomicBool = AtomicBool::new(false);

/// Bloques de 1024 muestras sin fin, al ritmo de un micro; soltarla se detecta con `SOLTADA`.
struct Eterna;
impl Drop for Eterna {
    fn drop(&mut self) {
        SOLTADA.store(true, SeqCst);
    }
}
impl Fuente for Eterna {
    fn leer(&mut self) -> Lectura {
        std::thread::sleep(Duration::from_millis(20));
        Lectura::Muestras(vec![0.3; 4 * TRAMA])
    }
    fn dispositivo(&self) -> String {
        "falso".into()
    }
}

/// Reconocedor lento: cada bloque tarda 150 ms en decodificarse.
struct Lento(usize);
impl Reconocedor for Lento {
    fn alimentar(&mut self, m: &[f32]) {
        self.0 += m.len();
        std::thread::sleep(Duration::from_millis(150));
    }
    fn texto(&self) -> String {
        "hola".into()
    }
    fn fin_de_frase(&self) -> bool {
        false
    }
    fn cerrar(&mut self) -> String {
        "hola".into()
    }
}

#[test]
fn salir_con_una_frase_abierta_termina_tras_el_bloque_en_curso() {
    let (tx, rx) = channel();
    let mut rec = Lento(0);
    let mut abrir: Box<Abrir> = Box::new(|_| Ok(Box::new(Eterna) as Box<dyn Fuente>));
    let aj = Ajustes {
        modelo: "x",
        mejora: true,
        ruido: false,
        mics: false,
    };
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(500));
        tx.send(Orden::Salir).unwrap();
    });
    let mejora = Mejora::con_vad(true, false, Box::new(Siempre));
    let t = Instant::now();
    let codigo = ciclo(&mut *abrir, &mut rec, rx, aj, mejora);
    let dur = t.elapsed();
    assert_eq!(codigo, 0);
    assert!(rec.0 > 0, "la frase estaba abierta y decodificando");
    assert!(dur < Duration::from_millis(900), "tardo {dur:?}");
    assert!(
        !SOLTADA.load(SeqCst),
        "salir no debe esperar a cerrar el micro"
    );
}
