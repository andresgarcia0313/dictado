//! Vocabulario propio: corrige términos que el modelo confunde (p. ej. "cu a" -> "QA").
//! Glosario en `<config>/dictado/vocabulario.txt`, recargado solo cuando cambia el archivo.
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{Duration, Instant, SystemTime};
use tabla::{parsear, reemplazar, Termino};

const SEMILLA: &str = "# Glosario: TERMINO = variante, variante (el sonido parecido se corrige solo)\n\
QA = cu a, cuba, kiu ei\nREQ = req, rec, rek\nINT\nSAP = sap, zap\nAPI = a pe i\nRedmine\nKubernetes\nGitHub\nPostgreSQL\n";
const REVISAR_CADA: Duration = Duration::from_secs(1);

struct Estado {
    modif: Option<SystemTime>,
    revisado: Option<Instant>,
    terminos: Vec<Termino>,
}

static ESTADO: Mutex<Estado> = Mutex::new(Estado {
    modif: None,
    revisado: None,
    terminos: vec![],
});

/// Ruta del glosario del usuario.
pub fn ruta() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_default()
        .join("dictado")
        .join("vocabulario.txt")
}

fn recargar(e: &mut Estado) {
    let ruta = ruta();
    if !ruta.exists() {
        let _ = ruta.parent().map(std::fs::create_dir_all);
        let _ = std::fs::write(&ruta, SEMILLA);
    }
    let modif = std::fs::metadata(&ruta).and_then(|m| m.modified()).ok();
    e.revisado = Some(Instant::now());
    if modif.is_some() && modif == e.modif {
        return;
    }
    e.modif = modif;
    e.terminos = std::fs::read_to_string(&ruta)
        .map(|c| parsear(&c))
        .unwrap_or_default();
}

fn corregir(texto: &str) -> (String, usize) {
    let mut e = ESTADO.lock().unwrap_or_else(|p| p.into_inner());
    if e.revisado.is_none_or(|t| t.elapsed() >= REVISAR_CADA) {
        recargar(&mut e);
    }
    reemplazar(texto, &e.terminos)
}

/// Para parciales: corrige sin registrar nada (se llama ~15 veces por segundo).
pub fn aplicar_parcial(texto: &str) -> String {
    corregir(texto).0
}

/// Para finales: corrige y registra solo la cantidad de reemplazos, nunca el texto.
pub fn aplicar(texto: &str) -> String {
    let (sal, n) = corregir(texto);
    if n > 0 {
        registro::info!("vocabulario reemplazos={n}");
    }
    sal
}

#[path = "vocabulario_tabla.rs"]
mod tabla;

#[cfg(test)]
#[path = "vocabulario_pruebas.rs"]
mod pruebas;
