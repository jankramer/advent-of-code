{
  description = "Advent of Code";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
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
      overlays = [ (import rust-overlay) ];
      mkPkgs = system: import nixpkgs { inherit system overlays; };

      mkShellForSystem =
        system:
        with (mkPkgs system);
        mkShell {
          buildInputs = [
            (rust-bin.stable.latest.default.override { extensions = [ "rust-src" ]; })
            nixfmt-rfc-style
          ];
        };
    in
    {
      devShells.aarch64-darwin.default = mkShellForSystem "aarch64-darwin";
      devShells.x86_64-linux.default = mkShellForSystem "x86_64-linux";
    };
}
