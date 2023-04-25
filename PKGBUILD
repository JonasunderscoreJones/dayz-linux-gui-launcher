# Maintainer: Jonas_Jones https://jonasjones.me
pkgname="dayz-linux-gui-launcher-git"
pkgver="0.0.1"
pkgrel=1
pkgdesc="A gui using the dayz-linux-cli-launcher"
arch=('any')
depends=("dayz-linux-cli-launcher")
makedepends=("cargo")
license=("Right to modify")

prepare() {
    cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features
}

check() {
    export RUSTUP_TOOLCHAIN=stable
    cargo test --frozen --all-features
}

package() {
    install -Dm0755 -t "$pkgdir/usr/bin/" "target/release/$pkgname"
}