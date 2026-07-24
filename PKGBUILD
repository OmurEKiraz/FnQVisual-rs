# Maintainer: Omur <omrkrz0678@gmail.com>
pkgname=fnqvisual
pkgver=0.1.0
pkgrel=1
pkgdesc="FnQVisual Rust daemon and UI"
arch=('x86_64')
url="https://github.com/OmurEKiraz/FnQVisual-rs" # GitHub reponun adresini yaz
license=('GPL3.0')
depends=('gcc-libs' 'glibc')
makedepends=('cargo')
provides=('fnqvisual')
conflicts=('fnqvisual')
source=("${pkgname}-${pkgver}.tar.gz::https://github.com/OmurEKiraz/FnQVisual-rs/archive/refs/tags/v${pkgver}.tar.gz")
sha256sums=('SKIP') # veya oluşturduğun release tar.gz dosyasının sha256 çıktısı

prepare() {
  cd "FnQVisual-rs-${pkgver}"
  export CARGO_HOME="${srcdir}/cargo-home"
  cargo fetch --locked --target "$CARCH"
}

build() {
  cd "FnQVisual-rs-${pkgver}"
  export CARGO_HOME="${srcdir}/cargo-home"
  cargo build --frozen --release --all-targets
}

package() {
  cd "FnQVisual-rs-${pkgver}"
  
  # Binary dosyasını sistem dizinine yükle
  install -Dm755 "target/release/fnqvisual" "${pkgdir}/usr/bin/fnqvisual"
  
  # Systemd servisini yükle
  if [ -f "fnq-visual.service" ]; then
    install -Dm644 "fnq-visual.service" "${pkgdir}/usr/lib/systemd/system/fnq-visual.service"
  fi

  # Lisans dosyasını yükle
  if [ -f "LICENSE" ]; then
    install -Dm644 "LICENSE" "${pkgdir}/usr/share/licenses/${pkgname}/LICENSE"
  fi
}
