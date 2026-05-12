# Maintainer: Marcelo Antonio | https://github.com/MarceloAntonio
pkgname=lazyarch-bin
pkgver=0.1.0
pkgrel=1
pkgdesc="Tool that automates boring or time-consuming installations and configurations"
arch=('x86_64')
url="https://github.com/MarceloAntonio/LazyArch"
license=('MIT')

depends=()
makedepends=()

provides=("lazyarch")
conflicts=("lazyarch")

source=("lazy-arch::https://github.com/MarceloAntonio/LazyArch/releases/download/v${pkgver}/lazy-arch")
sha256sums=('SKIP')

package() {
  install -Dm755 "$srcdir/lazy-arch" "$pkgdir/usr/bin/lazy-arch"
}