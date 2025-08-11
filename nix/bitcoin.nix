{
  perSystem = { pkgs, lib, ... }: {
    options = {
      dependencies = with lib; mkOption {
        type = types.listOf types.package;
        description = "Required packages for Bitcoind crate";
        default = with pkgs; [
          bitcoin
          openssl
          pkg-config
        ];
      };
      hook = with lib; mkOption {
        type = types.lines;
        description = "Required shell hook for Bitcoind download";
        default = ''
          echo -e "\\033[1;31m"Skipping bitcoind download..."\\033[0;m"
          export BITCOIND_SKIP_DOWNLOAD=true
          export BITCOIND_EXE=${pkgs.bitcoin}/bin/bitcoind
          test ! -e certs/key.pem && test ! -e certs/cert.pem && (cd certs && sh generate_certs.sh)
          export ROXYD_TLS_CERT_PATH=certs/cert.pem
          export ROXYD_TLS_KEY_PATH=certs/key.pem
          export ROXYCLI_TLS_CERT_PATH=certs/cert.pem
          export ROXYCLI_TLS_KEY_PATH=certs/key.pem
        '';
      };
    };
  };
}
