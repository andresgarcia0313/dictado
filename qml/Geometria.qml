import QtQuick
import QtCore

// Recuerda tamaño, posición y modo de la ventana `ventana` entre ejecuciones.
Item {
    id: raiz
    required property Window ventana
    readonly property bool _normal: ventana.visibility === Window.Windowed

    Settings {
        id: cfg
        category: "ventana"
        property int x: -1; property int y: -1
        property int ancho: 960; property int alto: 640
        property int modo: Window.Windowed
    }
    Component.onCompleted: {
        ventana.width = cfg.ancho; ventana.height = cfg.alto
        if (cfg.x >= 0 && cfg.x + 120 < Screen.desktopAvailableWidth
            && cfg.y >= 0 && cfg.y + 80 < Screen.desktopAvailableHeight) { ventana.x = cfg.x; ventana.y = cfg.y }
        if (cfg.modo === Window.Maximized || cfg.modo === Window.FullScreen) ventana.visibility = cfg.modo
    }
    Connections {
        target: raiz.ventana
        function onWidthChanged() { if (raiz._normal) cfg.ancho = raiz.ventana.width }
        function onHeightChanged() { if (raiz._normal) cfg.alto = raiz.ventana.height }
        function onXChanged() { if (raiz._normal) cfg.x = raiz.ventana.x }
        function onYChanged() { if (raiz._normal) cfg.y = raiz.ventana.y }
        function onVisibilityChanged() {
            const v = raiz.ventana.visibility
            if (v === Window.Windowed || v >= Window.Maximized) cfg.modo = v
        }
    }
}
