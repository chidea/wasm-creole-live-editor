. ./env
trunk build --release
if ($?) {
  rm $trg_path/*.js
  rm $trg_path/*.wasm
  cp dist/* $trg_path/
  cp dist/index.html $trg_path/404.html
}