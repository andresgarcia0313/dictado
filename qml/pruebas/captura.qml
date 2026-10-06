import QtQuick
import ".."

// Captura de Main a un ancho: qml6 qml/pruebas/captura.qml -- ANCHO DIR_SALIDA
// Deja captura-<ancho>.png en DIR_SALIDA (corre uno por ancho: 480, 800, 1280).
Main {
    id: m
    readonly property var _a: Qt.application.arguments
    readonly property int _ancho: parseInt(_a[_a.length - 2])
    readonly property string _dir: _a[_a.length - 1]
    ctl: Simulado {
        animar: false; nivel: 0.7; ganancia: 12.3
        parcial: "hola como estas quiero revisar el informe del mes"
        historial: "Hola, como estas.\nQuiero revisar el informe.\nPor favor agenda la reunion del martes.\nRecuerda enviar las actas antes del viernes."
    }
    property real _f: 0
    Timer {
        interval: 15; repeat: true; running: true
        onTriggered: {
            m._f += 0.05
            m.ctl.espectro = Array.from({ length: 32 }, (_, i) => Math.max(0, Math.min(255, Math.round(
                230 * Math.exp(-Math.pow((i - 6 - 4 * Math.sin(m._f)) / 4, 2))
                + 150 * Math.exp(-Math.pow((i - 18 - 5 * Math.sin(m._f * 0.7)) / 5, 2)) + 30 * Math.random() - 10)))).join(",")
        }
    }
    Timer { interval: 400; running: true; onTriggered: { m.width = m._ancho; m.height = m._ancho < 640 ? 800 : 640 } }
    Timer {
        interval: 3500; running: true
        onTriggered: m.contentItem.grabToImage(r => {
            console.log(r.saveToFile(m._dir + "/captura-" + m._ancho + ".png") ? "ok " + m._ancho : "fallo")
            Qt.quit()
        })
    }
}
