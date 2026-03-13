{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    nci.url = "github:90-008/nix-cargo-integration";
    nci.inputs.nixpkgs.follows = "nixpkgs";

    parts.url = "github:hercules-ci/flake-parts";
    parts.inputs.nixpkgs-lib.follows = "nixpkgs";

    tfmt.url = "github:numtide/treefmt-nix";
    tfmt.inputs.nixpkgs.follows = "nixpkgs";
    pch.url = "github:cachix/pre-commit-hooks.nix";
    pch.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    inputs@{
      parts,
      nci,
      tfmt,
      pch,
      ...
    }:
    parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" ];
      imports = [
        nci.flakeModule
        tfmt.flakeModule
        pch.flakeModule
        ./crates.nix
      ];
      perSystem =
        {
          pkgs,
          config,
          ...
        }:
        let
          # shorthand for accessing this crate's outputs
          # you can access crate outputs under `config.nci.outputs.<crate name>` (see documentation)
          crateOutputs = config.nci.outputs."yarp";
        in
        {
          treefmt = {
            projectRootFile = "flake.nix";
            programs.nixfmt.enable = true;
            programs.rustfmt.enable = true;
          };

          formatter = config.treefmt.build.wrapper;

          pre-commit.settings = {
            hooks.rustfmt.enable = true;
            hooks.nixfmt.enable = true;
          };

          # export the crate devshell as the default devshell
          devShells.default = crateOutputs.devShell.overrideAttrs (old: {
            packages = (old.packages or [ ]);
            shellHook = ''
              ${old.shellHook or ""}
              ${config.pre-commit.installationScript}
            '';
          });
          # export the release package of the crate as default package
          packages.default = crateOutputs.packages.release;
        };
    };
}
