//! Carpeta de registros, apertura y rotacion de archivos.
use crate::Reg;
use std::fs::{self, File, OpenOptions};
use std::path::PathBuf;

const COPIAS: u32 = 3;

/// Carpeta de registros: `<estado del usuario>/dictado/registros`.
pub fn carpeta() -> PathBuf {
    let base = dirs::state_dir()
        .or_else(dirs::data_local_dir)
        .unwrap_or_else(std::env::temp_dir);
    base.join("dictado").join("registros")
}

pub(crate) fn abrir(ruta: &PathBuf) -> std::io::Result<File> {
    OpenOptions::new().create(true).append(true).open(ruta)
}

pub(crate) fn rotar(r: &mut Reg) {
    let ruta = carpeta().join(format!("{}.log", r.nombre));
    for i in (1..COPIAS).rev() {
        let _ = fs::rename(
            format!("{}.{i}", ruta.display()),
            format!("{}.{}", ruta.display(), i + 1),
        );
    }
    let _ = fs::rename(&ruta, format!("{}.1", ruta.display()));
    if let Ok(f) = abrir(&ruta) {
        r.archivo = f;
        r.bytes = 0;
    }
}
