{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      inherit (nixpkgs.lib) genAttrs;

      forAllSystems =
        genAttrs [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" ];
      forAllPkgs = function: forAllSystems (system: function pkgs.${system});

      pkgs = forAllSystems (system:
        (import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        }));
    in {
      formatter = forAllPkgs (pkgs: pkgs.nixpkgs-fmt);

      devShells = forAllPkgs (pkgs:
        with pkgs.lib;
        let
          rust-toolchain = pkgs.rust-bin.stable.latest.default.override {
            extensions = [ "rust-analyzer" "rust-src" ];
          };
        in {
          default = pkgs.mkShell rec {
            nativeBuildInputs = with pkgs; [ pkg-config rust-toolchain ];

            buildInputs = with pkgs; [ openssl ];

            RUST_SRC_PATH = "${rust-toolchain}/lib/rustlib/src/rust/library";
            LD_LIBRARY_PATH = makeLibraryPath buildInputs;
          };
        });
    };
}
