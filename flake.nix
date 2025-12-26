{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }: flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [rust-overlay.overlays.default];
      };

      toolchain = pkgs.rust-bin.stable.latest.default.override {
        extensions = [
          "rust-src"
        ];
      };
    in
    {
      devShells.default = pkgs.mkShell {
        buildInputs =  [
          toolchain
        ];
      };

      formatter = pkgs.alejandra;
    }
  );
}