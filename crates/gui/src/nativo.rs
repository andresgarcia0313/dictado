//! Puente C del archivo `nativo.cpp` (portapapeles y salida) más la traza opcional de eventos.
use crate::evento::Evento;

extern "C" {
    fn dictado_copiar(texto: *const u8, largo: usize);
    fn dictado_salir();
    fn dictado_icono(datos: *const u8, largo: usize);
    fn dictado_abrir_carpeta(ruta: *const u8, largo: usize);
    fn dictado_geometria(buf: *mut u8, capacidad: usize) -> usize;
}

/// Abre una carpeta con el explorador de archivos del sistema.
pub fn abrir_carpeta(ruta: &str) {
    // SAFETY: puntero y largo provienen de un &str válido durante la llamada.
    unsafe { dictado_abrir_carpeta(ruta.as_ptr(), ruta.len()) }
}

/// Geometría de ventana guardada por QML (`clave=valor` separados por espacios).
pub fn geometria() -> String {
    let mut buf = [0u8; 256];
    // SAFETY: el búfer es válido y C++ no escribe más de `capacidad` bytes.
    let n = unsafe { dictado_geometria(buf.as_mut_ptr(), buf.len()) };
    String::from_utf8_lossy(&buf[..n.min(buf.len())]).into_owned()
}

pub fn copiar(texto: &str) {
    // SAFETY: puntero y largo provienen de un &str válido durante la llamada.
    unsafe { dictado_copiar(texto.as_ptr(), texto.len()) }
}

/// Icono de la ventana, embebido en el binario para no depender de archivos externos.
pub fn fijar_icono() {
    const PNG: &[u8] = include_bytes!("../../../recursos/dictado-256.png");
    // SAFETY: el slice es estático y C++ solo lo lee durante la llamada.
    unsafe { dictado_icono(PNG.as_ptr(), PNG.len()) }
}

pub fn salir() {
    // SAFETY: función C++ sin argumentos; se invoca desde el hilo de Qt.
    unsafe { dictado_salir() }
}

/// Con `DICTADO_DEPURA=1` escribe en stderr cada evento recibido (lo usa la prueba offscreen).
pub fn traza(ev: &Evento) {
    if std::env::var_os("DICTADO_DEPURA").is_none() {
        return;
    }
    match ev {
        Evento::Listo { modelo } => eprintln!("[gui] listo {modelo}"),
        Evento::Parcial(t) => eprintln!("[gui] parcial {t}"),
        Evento::Final(t) => eprintln!("[gui] final {t}"),
        Evento::Error(m) => eprintln!("[gui] error {m}"),
        Evento::Fin(l) => eprintln!("[gui] fin limpio={l}"),
        Evento::Mics { lista, actual } => eprintln!("[gui] mics {lista} actual={actual}"),
        Evento::Espectro(_) => eprintln!("[gui] espectro"),
        Evento::Ganancia(db) => eprintln!("[gui] ganancia {db}"),
        Evento::Entorno { estado, snr } => eprintln!("[gui] entorno {estado} snr={snr}"),
        Evento::Iniciando | Evento::Nivel(_) => {}
    }
}

pub fn traza_ram(app: f64, motor: f64) {
    if std::env::var_os("DICTADO_DEPURA").is_some() {
        eprintln!("[gui] ram app={app:.1} motor={motor:.1}");
    }
}
