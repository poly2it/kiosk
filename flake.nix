{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs =
    inputs@{
      self,
      nixpkgs,
      treefmt-nix,
      rust-overlay,
    }:
    let
      pkgsFor = system: import nixpkgs {
        inherit system;
        overlays = [
          (import rust-overlay)
        ];
      };
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forAllSystems =
        let
          lib = nixpkgs.lib;
        in
        f:
        systems
        |> lib.map (system: f system (pkgsFor system))
        |> lib.foldl (a: b: lib.recursiveUpdate a b) { };
      forAllSystemAttrs = f: forAllSystems (system: pkgs: { ${system} = f system pkgs; });
      treefmtEval = forAllSystemAttrs (system: pkgs: treefmt-nix.lib.evalModule pkgs ./treefmt.nix);
    in
    forAllSystems (
      system: pkgs: {
        formatter = forAllSystemAttrs (system: pkgs: treefmtEval.${pkgs.system}.config.build.wrapper);
        checks = forAllSystemAttrs (
          system: pkgs: {
            formatting = treefmtEval.${pkgs.system}.config.build.check self;
          }
        );
        devShells.${system}.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            (rust-bin.nightly.latest.default.override {
              targets = [ "x86_64-unknown-linux-gnu" "wasm32-unknown-unknown" ];
              extensions = [ "rustc-codegen-cranelift-preview" ];
            })
            dioxus-cli
            cargo
            pkg-config
            openssl
            mold
            clang
            llvmPackages.lld
            wasm-bindgen-cli
          ];
          shellHook = ''
            export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath (with pkgs; [ openssl ])}
          '';
        };
      }
    );
}
