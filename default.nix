let
  pkgs = import <nixpkgs> {};
in (pkgs.rustPlatform.buildRustPackage rec {
  pname = "lockjaw";
  version = "0.1.0";
  src = ./.;
  buildInputs = [ pkgs.keyutils ];
  cargoSha256 = "sha256:16d38s9wxlb2mxbnfvwm7kmwlpcgdq3pzkyxgwfk9vifpbirkh1m";
  verifyCargoDeps = true;
  meta = with pkgs.stdenv.lib; {
    description = "CLI Secret Manager with Linux Keyring Support";
    platforms = platforms.linux;
    license = licenses.bsd3;
  };
})
