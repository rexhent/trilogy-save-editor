# Maintainer: rexhent <dhannah10@outlook.com>
pkgname=trilogy-save-editor-bin
pkgver=2.2.1
pkgrel=1
pkgdesc="Save editor for the Mass Effect Trilogy"
arch=('x86_64')
url="https://github.com/rexhent/trilogy-save-editor"
license=('CECILL-2.1')
depends=('gtk3' 'glib2')

# Source URL and checksum
source=("https://github.com/rexhent/trilogy-save-editor/releases/download/linux/trilogy-save-editor")
sha256sums=('a7afa3afaa5f99cceba830dd66b5e9342ac67c37bc3c0336a777aaf7ecb11aa7')
package() {
	# Create necessary directories
	mkdir -p "${pkgdir}/usr/bin"

	# Copy the binary to /usr/bin
	install -Dm755 "${srcdir}/trilogy-save-editor" "${pkgdir}/usr/bin/trilogy-save-editor"
}
