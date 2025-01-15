## broken - requires finishing (use development shell and `develop` script)

{ pkgs, lib, rustPlatform, fenix }:

let
  cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
in

rustPlatform.buildRustPackage {
  pname = cargoToml.package.name;
  version = cargoToml.package.version;
  src = lib.sources.cleanSource ./.;
  cargoHash = "sha256-bUSs2l6yZHBIX2Ok85730jsfykE+d36Z0ZSUP1t8sYM=";
  nativeBuildInputs = with pkgs; with llvmPackages; [ fenix bintools wasm-pack ];
  CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER = "lld";
}
