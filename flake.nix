{
  description = "Flake for gh-labels-cli";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crate2nix = {
      url = "github:kolloch/crate2nix";
      flake = false;
    };
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, crate2nix, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        name = "gh-labels";

        overlays = [
          (import rust-overlay)
          (self: super: {
            # Because rust-overlay bundles multiple rust packages into one
            # derivation, specify that mega-bundle here, so that crate2nix
            # will use them automatically.
            rustc = self.rust-bin.stable.latest.default;
            cargo = self.rust-bin.stable.latest.default;
          })
        ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        inherit (import "${crate2nix}/tools.nix" { inherit pkgs; }) generatedCargoNix;

        project = pkgs.callPackage
          (generatedCargoNix {
            inherit name;
            src = ./.;
          })
          {
            # Override crates here...
          };
      in
      rec
      {
        # `nix build`
        packages.${name} = project.rootCrate.build;
        defaultPackage = packages.${name};

        # `nix run`
        apps.${name} = flake-utils.lib.mkApp {
          inherit name;
          drv = packages.${name};
        };
        defaultApp = apps.${name};

        # `nix develop`
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust toolchain.
            rust-bin.stable.latest.default

            # Additional cargo subcommands.
            cargo-edit
            cargo-expand
            cargo-outdated

            nixpkgs-fmt
          ];
        };
      }
    );
}
