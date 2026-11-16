{
  description = "queen-sweep";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    { self, nixpkgs, rust-overlay }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      forAllSystems = nixpkgs.lib.genAttrs systems;

      mkPkgs =
        system:
        import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

      mkToolchain =
        pkgs:
        pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src" # rust-analyzer needs this
            "rust-analyzer"
            "clippy"
            "rustfmt"
          ];
          targets = [ "wasm32-unknown-unknown" ];
        };
    in
    {
      formatter = forAllSystems (system: (import nixpkgs { inherit system; }).nixpkgs-fmt);

      devShells = forAllSystems (
        system:
        let
          pkgs = mkPkgs system;
        in
        {
          default = pkgs.mkShell {
            packages = [
              (mkToolchain pkgs)
              pkgs.wasm-pack
              pkgs.binaryen # provides wasm-opt
              pkgs.nodejs # for `wasm-pack test --node`
            ];
          };
        }
      );

      packages = forAllSystems (
        system:
        let
          pkgs = mkPkgs system;
          toolchain = mkToolchain pkgs;
          rustPlatform = pkgs.makeRustPlatform {
            cargo = toolchain;
            rustc = toolchain;
          };
        in
        {
          default = rustPlatform.buildRustPackage {
            pname = "queen-sweep-core";
            version = "0.1.0";
            src = self;
            cargoLock.lockFile = ./Cargo.lock;
          };
        }
      );
    };
}
