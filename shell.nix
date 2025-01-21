{ lib, writers, mkShell, map, nodePackages, ... }:

let
  develop = writers.writeBashBin "develop" ''
    PATH=${lib.makeBinPath [ nodePackages.http-server ]}:$PATH
    set -efu -o pipefail
    RUSTFLAGS=--cfg=web_sys_unstable_apis wasm-pack build --dev --target web || exit 1
    http-server -p 8080
  '';
in

mkShell {
  inputsFrom = [ map ];
  buildInputs = with nodePackages; [ prettier ];
  CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER = "lld";
  packages = [ develop ];
}
