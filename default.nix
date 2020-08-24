let
  pkgs = import <nixpkgs> {};
in (pkgs.rustPlatform.buildRustPackage rec {
  pname = "lockjaw";
  version = "0.1.0";
  src = ./.;
  buildInputs = [ pkgs.keyutils ];
  cargoSha256 = "sha256:1gj46681w6h3qksmjzryg17gds9lvvy71x77xz7i2gkzzqksjb5b";
  verifyCargoDeps = true;
  meta = with pkgs.stdenv.lib; {
    description = "CLI Secret Manager with Linux Keyring Support";
    platforms = platforms.linux;
    license = licenses.bsd3;
  };
})
