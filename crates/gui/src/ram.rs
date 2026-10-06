//! Medición de memoria residente (RSS) por PID con sysinfo.
use crate::controlador::qobject;
use crate::{motor, nativo};
use cxx_qt::CxxQtThread;
use std::{thread, time::Duration};
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System};

pub struct Medidor(System);

impl Medidor {
    pub fn new() -> Self {
        Self(System::new())
    }

    /// RSS en MB del proceso `pid`; 0.0 si no existe.
    pub fn mb(&mut self, pid: u32) -> f64 {
        if pid == 0 {
            return 0.0;
        }
        let pid = Pid::from_u32(pid);
        self.0.refresh_processes_specifics(
            ProcessesToUpdate::Some(&[pid]),
            true,
            ProcessRefreshKind::nothing().with_memory(),
        );
        self.0
            .process(pid)
            .map_or(0.0, |p| p.memory() as f64 / 1_048_576.0)
    }
}

/// Hilo que mide la RAM de la app y del motor cada segundo y la publica en el Controlador.
pub fn vigilar(hilo: CxxQtThread<qobject::Controlador>) {
    thread::spawn(move || {
        let (mut medidor, yo) = (Medidor::new(), std::process::id());
        let limite = std::env::var("DICTADO_AUTOSALIR")
            .ok()
            .and_then(|v| v.parse::<u64>().ok());
        for segundo in 0.. {
            let (app, mot) = (medidor.mb(yo), medidor.mb(motor::pid()));
            nativo::traza_ram(app, mot);
            if segundo % 30 == 0 {
                let pid = motor::pid();
                registro::info!("ram app_mb={app:.1} motor_mb={mot:.1} pid_motor={pid}");
            }
            if limite.is_some_and(|l| segundo >= l) {
                let _ = hilo.queue(|_| nativo::salir());
            }
            let vivo = hilo.queue(move |mut o| {
                o.as_mut().set_ram_app(app);
                o.set_ram_motor(mot);
            });
            if vivo.is_err() {
                return;
            }
            thread::sleep(Duration::from_secs(1));
        }
    });
}
