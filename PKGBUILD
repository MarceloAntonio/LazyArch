# Maintainer: Marcelo Antonio <seu-email@aqui.com>
pkgname=lazyarch-git
pkgver=r1.1234567
pkgrel=1
pkgdesc="Tool that automates boring or time-consuming installations and configurations"
arch=('any')
url="https://github.com/MarceloAntonio/LazyArch"
license=('MIT')

depends=('bash' 'python' 'python-pip')
makedepends=('git')

provides=("lazyarch")
conflicts=("lazyarch")

source=("${pkgname}::git+https://github.com/MarceloAntonio/LazyArch.git")
sha256sums=('SKIP')

pkgver() {
  cd "$srcdir/${pkgname}"
  printf "r%s.%s" "$(git rev-list --count HEAD)" "$(git rev-parse --short=7 HEAD)"
}

package() {
  cd "$srcdir/${pkgname}"

  python -m pip install -r LazyArch/requirements.txt \
    --root="$pkgdir" \
    --prefix=/usr \
    --break-system-packages \
    --no-warn-script-location

  install -d "$pkgdir/usr/share/LazyArch_files"
  install -d "$pkgdir/usr/bin"
  
  cp -a LazyArch/. "$pkgdir/usr/share/LazyArch_files/"
  
  chmod +x "$pkgdir/usr/share/LazyArch_files/LazyArch.sh"

  ln -s /usr/share/LazyArch_files/LazyArch.sh "$pkgdir/usr/bin/LazyArch"
}