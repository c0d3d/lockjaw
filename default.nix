let
  timeOfWriting = "2020-09-06";
  pkgs = import <nixpkgs> {};
  nightlyPlatform = let
    mozillaOverlay = pkgs.fetchFromGitHub {
      owner = "mozilla";
      repo = "nixpkgs-mozilla";
      rev = "efda5b357451dbb0431f983cca679ae3cd9b9829";
      sha256 = "11wqrg86g3qva67vnk81ynvqyfj0zxk83cbrf0p9hsvxiwxs8469";
    };
    mozilla = pkgs.callPackage "${mozillaOverlay.out}/package-set.nix" {};
    rustNightly = (mozilla.rustChannelOf { date = timeOfWriting; channel = "nightly"; }).rust;
  in pkgs.makeRustPlatform {
    cargo = rustNightly;
    rustc = rustNightly;
  };
in (nightlyPlatform.buildRustPackage rec {
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
