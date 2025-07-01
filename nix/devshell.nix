{ inputs, ... }:
{
  perSystem = { config, self', pkgs, lib, ... }: {
    devShells.default = pkgs.mkShell {
      name = "rust-bitcoin-proxy";
      inputsFrom = [
        self'.devShells.rust
        config.pre-commit.devShell
      ] ++ config.dependencies;
      shellHook = config.hook;
      packages = with pkgs; [
        just
        nixd
        bacon
      ] ++ config.dependencies;
    };
  };
}
