//! Registro a archivo con rotacion por tamano; sin dependencias pesadas.
//! Nivel por `DICTADO_LOG` (error|aviso|info|depura, por defecto info); `DICTADO_LOG_STDERR=1` repite en stderr.
use std::fmt::Arguments;
use std::fs::{self, File};
use std::io::Write;
use std::sync::{Mutex, OnceLock};

mod archivo;
mod macros;
pub use archivo::carpeta as carpeta_registros;
use archivo::{abrir, carpeta, rotar};

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub enum Nivel {
    Error,
    Aviso,
    Info,
    Depura,
}

const MAX_BYTES: u64 = 1_000_000;

pub(crate) struct Reg {
    pub(crate) nombre: &'static str,
    nivel: Nivel,
    pub(crate) archivo: File,
    pub(crate) bytes: u64,
    stderr: bool,
}

static REG: OnceLock<Mutex<Reg>> = OnceLock::new();

/// Abre `<nombre>.log` y deja constancia de panicos. Llamar una vez al arrancar.
pub fn iniciar(nombre: &'static str) {
    let dir = carpeta();
    let _ = fs::create_dir_all(&dir);
    let nivel = match std::env::var("DICTADO_LOG").as_deref() {
        Ok("error") => Nivel::Error,
        Ok("aviso") => Nivel::Aviso,
        Ok("depura") => Nivel::Depura,
        _ => Nivel::Info,
    };
    let ruta = dir.join(format!("{nombre}.log"));
    let Ok(archivo) = abrir(&ruta) else { return };
    let bytes = archivo.metadata().map(|m| m.len()).unwrap_or(0);
    let stderr = std::env::var("DICTADO_LOG_STDERR").is_ok_and(|v| v == "1");
    let _ = REG.set(Mutex::new(Reg {
        nombre,
        nivel,
        archivo,
        bytes,
        stderr,
    }));
    std::panic::set_hook(Box::new(|i| {
        escribir(Nivel::Error, "panico", format_args!("{i}"))
    }));
    escribir(
        Nivel::Info,
        "registro",
        format_args!(
            "inicio pid={} version={}",
            std::process::id(),
            env!("CARGO_PKG_VERSION")
        ),
    );
}

pub fn escribir(nivel: Nivel, modulo: &str, msg: Arguments) {
    let Some(m) = REG.get() else { return };
    let Ok(mut r) = m.lock() else { return };
    if nivel > r.nivel {
        return;
    }
    let t = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%.3f");
    let linea = format!("{t} {nivel:<6?} {modulo} {msg}\n");
    if r.bytes + linea.len() as u64 > MAX_BYTES {
        rotar(&mut r);
    }
    if r.archivo.write_all(linea.as_bytes()).is_ok() {
        r.bytes += linea.len() as u64;
    }
    if r.stderr {
        eprint!("{linea}");
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn niveles_ordenados_y_carpeta_fuera_del_repo() {
        crate::info!("prueba {}", 1);
        assert!(Nivel::Error < Nivel::Depura);
        assert!(carpeta().ends_with("dictado/registros"));
    }
}
