{ pkgs, mkShell, fenix }:

let
  develop = pkgs.writeShellScriptBin "develop" ''
    wasm-pack build --target web
    http-server -p 8080
  '';
in

mkShell {
  inputsFrom = [ pkgs.map ];
  buildInputs = with pkgs.nodePackages; [
    http-server
    prettier
  ];
  CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER = "lld";
  RUST_BACKTRACE = 1;
  packages = [ develop ];
}
