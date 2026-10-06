//! Reconocedor de voz en streaming sobre muestras mono a 16 kHz.
pub trait Reconocedor {
    /// Alimenta audio y decodifica lo que este listo.
    fn alimentar(&mut self, muestras: &[f32]);
    /// Texto parcial de la frase en curso.
    fn texto(&self) -> String;
    /// El motor detecto silencio suficiente para cerrar la frase.
    fn fin_de_frase(&self) -> bool;
    /// Cierra la frase: añade la cola de silencio, devuelve el texto completo y reinicia.
    fn cerrar(&mut self) -> String;
}
