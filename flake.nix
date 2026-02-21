{
  inputs = { nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11"; };

  outputs = { self, nixpkgs }:
    let
      inherit (nixpkgs.lib) genAttrs;

      forAllSystems =
        genAttrs [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" ];
      forAllPkgs = function: forAllSystems (system: function pkgs.${system});

      pkgs = forAllSystems (system:
        (import nixpkgs {
          inherit system;
          overlays = [ ];
        }));
    in {
      formatter = forAllPkgs (pkgs: pkgs.nixpkgs-fmt);

      devShells = forAllPkgs (pkgs:
        with pkgs.lib; {
          default = pkgs.mkShell rec {
            nativeBuildInputs = with pkgs; [ bun nodejs_24 ];
            buildInputs = with pkgs; [ prisma prisma-engines ];
            shellHook = with pkgs; ''
              export PRISMA_SCHEMA_ENGINE_BINARY="${prisma-engines}/bin/schema-engine"
              export PRISMA_QUERY_ENGINE_BINARY="${prisma-engines}/bin/query-engine"
              export PRISMA_QUERY_ENGINE_LIBRARY="${prisma-engines}/lib/libquery_engine.node"
              export PRISMA_FMT_BINARY="${prisma-engines}/bin/prisma-fmt"
            '';
            LD_LIBRARY_PATH = makeLibraryPath buildInputs;
          };
        });

    };
}
