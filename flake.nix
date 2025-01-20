{
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  inputs.fenix.url = "github:nix-community/fenix";
  inputs.fenix.inputs.nixpkgs.follows = "nixpkgs";
  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { nixpkgs, fenix, flake-utils, ... }:

    with flake-utils.lib;

    eachDefaultSystem (system:

      let
        fenix-system = with fenix.packages.${system}; combine [ stable.toolchain targets.wasm32-unknown-unknown.stable.rust-std ];
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (final: _: { map = final.callPackage ./default.nix { fenix = fenix-system; }; }) ];
        };
      in

      {
        formatter = pkgs.nixpkgs-fmt;
        packages.map = pkgs.map;
        packages.default = pkgs.map;
        devShells.default = pkgs.callPackage ./shell.nix { };
      }
    );
}
