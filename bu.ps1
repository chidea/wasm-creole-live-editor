. ./env
trunk build --release
if ($?) {
  rm $trg_path/index.html
  rm $trg_path/*.js
  rm $trg_path/*.wasm
  cp dist/* $trg_path/
}