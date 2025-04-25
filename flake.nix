{
  description = "Build a cargo project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane.url = "github:ipetkov/crane";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-analyzer-src.follows = "";
    };

    flake-utils.url = "github:numtide/flake-utils";

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      fenix,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        craneLib = crane.mkLib pkgs;
        src = ./.;

        cargoToml = builtins.fromTOML (builtins.readFile (self + /Cargo.toml));
        inherit (cargoToml.package) name version;

        tailwindcss = pkgs.nodePackages.tailwindcss.overrideAttrs (oa: {
          plugins = [
            pkgs.nodePackages."@tailwindcss/aspect-ratio"
            pkgs.nodePackages."@tailwindcss/forms"
            pkgs.nodePackages."@tailwindcss/language-server"
            pkgs.nodePackages."@tailwindcss/line-clamp"
            pkgs.nodePackages."@tailwindcss/typography"
          ];
        });

        # Crane builder for cargo-leptos projects
        craneBuild = rec {
          args = {
            inherit src;
            pname = name;
            version = version;
            buildInputs = [
              pkgs.cargo-leptos
              pkgs.binaryen # Provides wasm-opt
              pkgs.pkg-config
              pkgs.openssl.dev
              pkgs.gcc
              pkgs.lld
              tailwindcss
            ];
          };
          cargoArtifacts = craneLib.buildDepsOnly args;
          buildArgs = args // {
            inherit cargoArtifacts;
            buildPhaseCargoCommand = ''
              cargoBuildLog=$(mktemp cargoBuildLogXXXX.json)
              cargo leptos build --release -vvv > "$cargoBuildLog"
            '';
            nativeBuildInputs = [
              pkgs.pkg-config
              pkgs.makeWrapper
            ];
            installPhaseCommand = ''
              mkdir -p $out/bin
              cp target/release/${name} $out/bin/
              cp -r target/site/ $out/bin/
              wrapProgram $out/bin/${name} \
                --set LEPTOS_SITE_ROOT $out/bin/site
            '';
          };
          package = craneLib.buildPackage buildArgs;
        };
      in
      {
        packages = {
          default = craneBuild.package;
        };

        devShells.default = craneLib.devShell {
          packages = [
            pkgs.cargo
            pkgs.gcc
            pkgs.lld
          ];
        };
      }
    );
}
