pkgname=fnq-visual
pkgver=0.1.0
pkgrel=1
pkgdesc="A zero-overhead, highly customizable Lenovo Fn+Q OSD overlay for Linux"
arch=('x86_64')
url="https://github.com/OmurEKiraz/FnQVisual-rs"
license=('MIT')
depends=('gtk4' 'gtk4-layer-shell')
makedepends=('cargo')
install="fnq-visual.install"
source=("${pkgname}-${pkgver}.tar.gz::https://github.com/OmurEKiraz/FnQVisual-rs/archive/refs/tags/v${pkgver}.tar.gz")
sha256sums=('faa88615b89c2215d81175fd3aa27c947adcb7ea81eca69f51746b6bf0c3f354')

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
