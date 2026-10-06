//! Microfonos de entrada disponibles: nombres cpal sin duplicados, el de por defecto primero.
use cpal::traits::{DeviceTrait, HostTrait};
use cpal::Device;

fn nombre(d: &Device) -> Option<String> {
    d.description().ok().map(|x| x.name().to_string())
}

/// Salidas y sumideros que algunos servidores de sonido listan también como entradas.
fn es_salida(n: &str) -> bool {
    let n = n.to_lowercase();
    ["output", "playback", "discard"]
        .iter()
        .any(|k| n.contains(k))
}

fn utilizable(d: &Device) -> bool {
    d.default_input_config().is_ok()
        || d.supported_input_configs()
            .is_ok_and(|mut r| r.next().is_some())
}

/// Dispositivos de entrada utilizables con su nombre; el de por defecto va primero.
pub fn entradas() -> Vec<(String, Device)> {
    let host = cpal::default_host();
    let defecto = host.default_input_device();
    let resto = host.input_devices().into_iter().flatten();
    let mut salida: Vec<(String, Device)> = Vec::new();
    for d in defecto.into_iter().chain(resto) {
        if let Some(n) = nombre(&d) {
            if !n.is_empty()
                && !es_salida(&n)
                && utilizable(&d)
                && salida.iter().all(|(x, _)| *x != n)
            {
                salida.push((n, d));
            }
        }
    }
    salida
}

/// Nombres utilizables; el de por defecto va primero.
pub fn listar() -> Vec<String> {
    entradas().into_iter().map(|(n, _)| n).collect()
}

/// Busca por nombre; `None` o "auto" devuelve el de por defecto.
pub fn buscar(pedido: Option<&str>) -> Result<(String, Device), String> {
    let todos = entradas();
    match pedido.filter(|n| !n.eq_ignore_ascii_case("auto") && !n.is_empty()) {
        None => todos
            .into_iter()
            .next()
            .ok_or_else(|| "no hay microfono".into()),
        Some(n) => todos.into_iter().find(|(x, _)| x == n).ok_or_else(|| {
            registro::aviso!("el microfono pedido no existe pedido=\"{n}\"");
            format!("no existe el microfono \"{n}\"")
        }),
    }
}

#[cfg(test)]
#[path = "dispositivos_pruebas.rs"]
mod pruebas;
