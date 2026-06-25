pkgname=fnq-visual
pkgver=0.1.0
pkgrel=1
pkgdesc="A zero-overhead, highly customizable Lenovo Fn+Q OSD overlay for Linux (Wayland/X11)"
arch=('x86_64')
url="https://github.com/omrkrz/FnQVisual-rs"
license=('MIT')
depends=('gtk4' 'gtk4-layer-shell')
makedepends=('cargo')
install="fnq-visual.install"
source=("${pkgname}-${pkgver}.tar.gz::https://github.com/omrkrz/FnQVisual-rs/archive/refs/tags/v${pkgver}.tar.gz")
sha256sums=('SKIP') # Update this with the actual sha256sum once you release on GitHub

build() {
    cd "FnQVisual-rs-${pkgver}"
    export RUSTUP_TOOLCHAIN=stable
    cargo build --release --locked
}

package() {
    cd "FnQVisual-rs-${pkgver}"
    
    # Install the executable
    install -Dm755 "target/release/fnq-visual" "${pkgdir}/usr/bin/fnq-visual"
    
    # Install the systemd user service
    install -Dm644 "fnq-visual.service" "${pkgdir}/usr/lib/systemd/user/fnq-visual.service"
}