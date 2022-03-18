with import <nixpkgs> {};
stdenv.mkDerivation {
  name = "omnidict";
  buildInputs = with pkgs; [bintools-unwrapped openssl pkg-config];
}
