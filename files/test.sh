
#!/usr/bin/env bash

echo 'System toolchain versions:'
ldd --version | head -n1 | sed 's/^ldd //'
ld --version | head -n1

function test_pr() {
  cd rust-proc-macro-panic-inside-panic-expample
  rustc +$1 -V
  cargo +$1 test -- --nocapture
  cd ..
}

echo 'Test before PR'
echo '----------------------------------'
test_pr nightly-2022-08-01
echo 'Test after PR'
echo '----------------------------------'
test_pr nightly-2022-08-02
