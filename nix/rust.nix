{ inputs, ... }:
{
  imports = [
    inputs.rust-flake.flakeModules.default
    inputs.rust-flake.flakeModules.nixpkgs
  ];
  perSystem = { config, hook, self', pkgs, lib, ... }: {
    rust-project.crates."roxy".crane.args = {
      preBuild = config.hook;
      doCheck = false;
      buildInputs = config.dependencies ++ lib.optionals pkgs.stdenv.isDarwin (
        with pkgs.darwin.apple_sdk.frameworks; [
          IOKit
        ]
      );
    };
    packages.default = self'.packages.roxy;
    apps = {
      roxyd = {
        type = "app";
        program = "${self'.packages.roxy}/bin/roxyd";
      };
      roxy-cli = {
        type = "app";
        program = "${self'.packages.roxy}/bin/roxy-cli";
      };
    };
  };
}
