{
  inputs = {
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
    flake-utils.follows = "cargo2nix/flake-utils";
    nixpkgs.follows = "cargo2nix/nixpkgs";
  };

  outputs = inputs: with inputs; # pass through all inputs and bring them into scope

    # Build the output set for each default system and map system sets into
    # attributes, resulting in paths such as:
    # nix build .#packages.x86_64-linux.<name>
    flake-utils.lib.eachDefaultSystem (system:

      # let-in expressions, very similar to Rust's let bindings.  These names
      # are used to express the output but not themselves paths in the output.
      let

        # create nixpkgs that contains rustBuilder from cargo2nix overlay
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ cargo2nix.overlays.default ];
        };

        # create the workspace & dependencies package set
        rustPkgs = pkgs.rustBuilder.makePackageSet {
          rustVersion = "1.75.0";
          packageFun = import ./Cargo.nix;
          packageOverrides = pkgs: pkgs.rustBuilder.overrides.all ++ [    
            (pkgs.rustBuilder.rustLib.makeOverride {
              name = "openssl-sys";
              overrideAttrs = drv: {
                propagatedBuildInputs = drv.propagatedBuildInputs or [ ] ++ [
                  pkgs.openssl
                  pkgs.pkg-config
                  pkgs.bitcoin
                ];
              };
            })
          ];
          extraRustComponents = ["clippy" "rust-analyzer" "rustfmt"];
        };

        workspaceShell = (rustPkgs.workspaceShell {
          packages = with pkgs; [ 
            just
          ];
          shellHook = ''
             echo -e "\\033[1;31m"Skipping bitcoind download..."\\033[0;m"
             export BITCOIND_SKIP_DOWNLOAD=true
          '';
        });

      in rec {
        # this is the output (recursive) set (expressed for each system)

        # the packages in `nix build .#packages.<system>.<name>`
        packages = {
          # nix build .#rust-bitcoin-proxy
          # nix build .#packages.x86_64-linux.rust-bitcoin-proxy
          roxy = (rustPkgs.workspace.roxy {});
          # nix build
          default = packages.roxy;
        };

        apps = {
          inherit system;
          roxyd = {
            type = "app";
            program ="${self.packages.${system}.roxy}/bin/roxyd";
          };
          roxy-cli = {
            type = "app";
            program ="${self.packages.${system}.roxy}/bin/roxy-cli";
          };
        };
        
        devShell = workspaceShell;       
      }
    );
}
