{ rustPlatform, fetchFromGitHub, stdenv, lib, ... }:

rustPlatform.buildRustPackage rec {
  name = "taskwarrior-hooks-${version}";
  version = "0.2.3";
  src = ./.;
  cargoSha256 = "sha256-O3ui3TmkqLhYorL7zsOLawfWpIZOXb2CBG4ei8OBRGY=";
  verifyCargoDeps = true;
  meta = with lib; {
    description = "taskwarrior hook collecton";
    homepage = "https://github.com/mrvandalo/taskwarrior-hooks";
    license = licenses.gpl3Plus;
    platforms = platforms.all;
  };
}

