with import <nixpkgs> {};
stdenv.mkDerivation {
  name = "the-project";
  buildInputs = with pkgs; [bintools-unwrapped openssl pkg-config];
}
