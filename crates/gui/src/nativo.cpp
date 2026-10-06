// Utilidades Qt mínimas sin equivalente en cxx-qt-lib: portapapeles, salida, carpeta y geometría.
#include <QClipboard>
#include <QCoreApplication>
#include <QGuiApplication>
#include <QIcon>
#include <QImage>
#include <QPixmap>
#include <QDesktopServices>
#include <QSettings>
#include <QString>
#include <QUrl>
#include <algorithm>
#include <cstddef>
#include <cstring>

extern "C" {
void dictado_copiar(const char *texto, size_t largo) {
    if (auto *portapapeles = QGuiApplication::clipboard())
        portapapeles->setText(QString::fromUtf8(texto, static_cast<qsizetype>(largo)));
}
void dictado_salir() { QCoreApplication::quit(); }
// Icono de ventana (PNG embebido) y nombre del .desktop, que KDE en Wayland usa para la barra de título.
void dictado_icono(const unsigned char *datos, size_t largo) {
    QGuiApplication::setDesktopFileName(QStringLiteral("dictado"));
    const QImage img = QImage::fromData(datos, static_cast<int>(largo), "PNG");
    if (!img.isNull())
        QGuiApplication::setWindowIcon(QIcon(QPixmap::fromImage(img)));
}
void dictado_abrir_carpeta(const char *ruta, size_t largo) {
    QDesktopServices::openUrl(QUrl::fromLocalFile(QString::fromUtf8(ruta, static_cast<qsizetype>(largo))));
}
// Geometría que guarda Main.qml en el grupo "ventana": "x=1 y=2 ..." (UTF-8, truncado a la capacidad).
size_t dictado_geometria(char *buf, size_t capacidad) {
    QSettings ajustes;
    ajustes.beginGroup("ventana");
    QStringList partes;
    for (const auto &clave : ajustes.childKeys())
        partes << clave + "=" + ajustes.value(clave).toString();
    const QByteArray bytes = partes.join(' ').toUtf8();
    const size_t n = std::min(static_cast<size_t>(bytes.size()), capacidad);
    std::memcpy(buf, bytes.constData(), n);
    return n;
}
}
