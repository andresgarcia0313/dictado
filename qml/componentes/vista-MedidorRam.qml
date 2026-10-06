import QtQuick
import ".."

// Vista previa aislada: qml6 qml/componentes/vista-MedidorRam.qml
Rectangle {
    width: 320; height: 240; color: Tema.fondo
    Column {
        anchors { centerIn: parent }
        spacing: 16
        MedidorRam { etiqueta: "Motor"; valor: 287; maximo: 320 }
        MedidorRam { etiqueta: "App"; valor: 96; maximo: 120; acento: Tema.magenta }
        MedidorRam { etiqueta: "Excedido"; valor: 340; maximo: 320 }
    }
}
