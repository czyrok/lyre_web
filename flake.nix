{
  description = "lyre_web";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      supported_systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" ];
      for_each_supported_system = nixpkgs.lib.genAttrs supported_systems;
    in
    {
      devShells = for_each_supported_system (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlays.default ];
          };

          rust_toolchain = (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml).override {
            extensions = [ "rust-src" "rust-analyzer" ];
          };

          wasm_bindgen_version = (pkgs.lib.findFirst
            (package: package.name == "wasm-bindgen")
            (throw "wasm-bindgen is missing from Cargo.lock")
            (builtins.fromTOML (builtins.readFile ./Cargo.lock)).package).version;

          ## cargo-leptos refuses a wasm-bindgen CLI whose version differs from Cargo.lock's
          wasm_bindgen_cli = pkgs."wasm-bindgen-cli_${builtins.replaceStrings [ "." ] [ "_" ] wasm_bindgen_version}";
        in
        {
          agent = pkgs.mkShellNoCC {
            packages = [
              rust_toolchain
              pkgs.cargo-leptos
              wasm_bindgen_cli
              pkgs.cargo-make
              pkgs.cargo-all-features
              pkgs.bun
              pkgs.binaryen
              pkgs.gcc
            ];
          };
        });
    };
}
