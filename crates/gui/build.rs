//! Compila el puente cxx-qt y embebe `qml/**` (raíz del repo) en el binario.
use cxx_qt_build::{CxxQtBuilder, QmlFile, QmlModule};
use std::path::{Path, PathBuf};
use std::{env, fs};

/// Archivos de solo desarrollo: no viajan en el binario.
fn sobra(nombre: &str) -> bool {
    matches!(nombre, "Prueba.qml" | "Simulado.qml" | "captura.qml") || nombre.starts_with("vista-")
}

fn recoger(origen: &Path, destino: &Path, rel: &Path, qml: &mut Vec<PathBuf>) {
    fs::create_dir_all(destino).expect("crear carpeta de preparación");
    let Ok(entradas) = fs::read_dir(origen) else {
        return;
    };
    for e in entradas.flatten() {
        let (ruta, nombre) = (e.path(), e.file_name().to_string_lossy().into_owned());
        println!("cargo::rerun-if-changed={}", ruta.display());
        if ruta.is_dir() && nombre == "pruebas" {
            continue;
        }
        if ruta.is_dir() {
            recoger(&ruta, &destino.join(&nombre), &rel.join(&nombre), qml);
        } else if ruta.extension().is_some_and(|x| x == "qml") && !sobra(&nombre) {
            fs::copy(&ruta, destino.join(&nombre)).expect("copiar qml");
            qml.push(rel.join(&nombre));
        }
    }
}

fn main() {
    let raiz = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../qml");
    let prep = PathBuf::from(env::var("OUT_DIR").unwrap()).join("qml-embebido");
    let mut qml = Vec::new();
    recoger(&raiz, &prep, Path::new(""), &mut qml);
    // El alias dentro del recurso es la ruta tal cual se pase: se trabaja desde la carpeta preparada.
    env::set_current_dir(&prep).expect("entrar a la carpeta preparada");
    let archivos = qml.iter().map(|p| {
        let f = QmlFile::from(p);
        f.singleton(p.file_name().is_some_and(|n| n == "Tema.qml"))
    });
    let fuente = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/controlador.rs");
    let nativo = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/nativo.cpp");
    CxxQtBuilder::new_qml_module(QmlModule::new("dictado.app").qml_files(archivos))
        .qt_module("Gui")
        .qt_module("Quick")
        .file(fuente)
        .cpp_file(nativo)
        .build();
}
