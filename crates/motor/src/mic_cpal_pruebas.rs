use super::*;
use std::sync::mpsc::sync_channel;

#[test]
fn cola_llena_descarta_lo_nuevo_y_cuenta() {
    let (tx, rx) = sync_channel(2);
    let (cola, mut descartes) = (AtomicUsize::new(0), 0);
    for i in 0..5 {
        encolar(&tx, vec![i as f32], &cola, &mut descartes);
    }
    assert_eq!((cola.load(Relaxed), descartes), (2, 3));
    assert_eq!(rx.recv().unwrap(), vec![0.0], "se conserva lo mas viejo");
}
