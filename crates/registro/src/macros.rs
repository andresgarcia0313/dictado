//! Macros `error!`, `aviso!`, `info!` y `depura!` (se usan como `registro::info!`).
#[macro_export]
macro_rules! error {
    ($($a:tt)*) => {
        $crate::escribir($crate::Nivel::Error, module_path!(), format_args!($($a)*))
    };
}
#[macro_export]
macro_rules! aviso {
    ($($a:tt)*) => {
        $crate::escribir($crate::Nivel::Aviso, module_path!(), format_args!($($a)*))
    };
}
#[macro_export]
macro_rules! info {
    ($($a:tt)*) => {
        $crate::escribir($crate::Nivel::Info, module_path!(), format_args!($($a)*))
    };
}
#[macro_export]
macro_rules! depura {
    ($($a:tt)*) => {
        $crate::escribir($crate::Nivel::Depura, module_path!(), format_args!($($a)*))
    };
}
