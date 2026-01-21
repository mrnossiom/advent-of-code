{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs?ref=nixos-unstable";

    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
    }:
    let
      inherit (nixpkgs.lib) genAttrs;

      forAllSystems = genAttrs [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];
      forAllPkgs = function: forAllSystems (system: function pkgs.${system});

      pkgs = forAllSystems (
        system:
        (import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        })
      );
    in
    {
      formatter = forAllPkgs (pkgs: pkgs.nixfmt-tree);

      devShells = forAllPkgs (
        pkgs:
        let
          file-rust-toolchain = pkgs.rust-bin.stable.latest.default;
          rust-toolchain = file-rust-toolchain.override { extensions = [ "rust-analyzer" ]; };
        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              pkg-config
              rust-toolchain
              aoc-cli
            ];

            RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
          };
        }
      );
    };
}
