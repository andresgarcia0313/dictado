import QtQuick
import ".."

// Cascada tiempo x frecuencia (graves abajo, tiempo hacia la izquierda). `bandas`: enteros 0-255.
// Canvas por software: cada trama desplaza `paso` px y pinta una columna (sin shaders).
Item {
    id: raiz
    property var bandas: []
    property bool activo: true
    property int paso: 2
    property var _cola: []
    property bool _limpiar: true
    readonly property var _paleta: {
        const p = []
        const a = [11, 13, 18], m = [232, 121, 249], c = [34, 211, 238]
        for (let v = 0; v < 256; v++) {
            const t = Math.pow(v / 255, 1.3) * 2, d = t < 1 ? a : m, h = t < 1 ? m : c, k = t < 1 ? t : t - 1
            p.push("rgb(" + [0, 1, 2].map(i => Math.round(d[i] + (h[i] - d[i]) * k)).join(",") + ")")
        }
        return p
    }
    implicitWidth: Tema.u * 30
    implicitHeight: Tema.u * 10
    opacity: activo ? 1 : 0.4
    Behavior on opacity { NumberAnimation { duration: Tema.suave } }

    onBandasChanged: if (activo && bandas.length > 0) {
        if (_cola.length < 8) _cola.push(bandas)
        lienzo.requestPaint()
    }

    Rectangle {
        anchors.fill: parent; radius: Tema.radioChico; color: Tema.fondo
        border { width: 1; color: Tema.borde }
    }
    Canvas {
        id: lienzo
        anchors { fill: parent; margins: 1 }
        renderTarget: Canvas.Image
        renderStrategy: Canvas.Cooperative
        onPaint: {
            const ctx = getContext("2d"), n = raiz._cola.length
            if (raiz._limpiar) { ctx.fillStyle = raiz._paleta[0]; ctx.fillRect(0, 0, width, height); raiz._limpiar = false }
            if (n === 0) { ctx.fillStyle = raiz._paleta[0]; ctx.fillRect(0, 0, width, height); return }
            const dx = n * raiz.paso
            ctx.drawImage(lienzo, -dx, 0)
            ctx.fillStyle = raiz._paleta[0]; ctx.fillRect(width - dx, 0, dx, height)
            raiz._cola.forEach((b, j) => {
                const x = width - dx + j * raiz.paso, hb = height / b.length
                for (let i = 0; i < b.length; i++) {
                    const y1 = Math.round(height - i * hb), y0 = Math.round(height - (i + 1) * hb)
                    ctx.fillStyle = raiz._paleta[Math.max(0, Math.min(255, b[i]))]
                    ctx.fillRect(x, y0, raiz.paso, y1 - y0)
                }
            })
            raiz._cola = []
        }
        onWidthChanged: { raiz._limpiar = true; requestPaint() }
        onHeightChanged: { raiz._limpiar = true; requestPaint() }
    }
    Accessible.role: Accessible.Graphic
    Accessible.name: "Espectrograma del micrófono"
}
