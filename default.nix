{ rustPlatform, fetchFromGitHub, stdenv, ... }:

rustPlatform.buildRustPackage rec {
  name = "taskwarrior-hooks-${version}";
  version = "0.1.0";
  src = fetchFromGitHub {
    owner = "mrVanDalo";
    repo = "taskwarrior-hooks";
    rev = "${version}";
    sha256 = "1vlfnp2ib009zb8j6m0l47qnh9ipsikcm00cjjcbnsgknihwcn6r";
  };

  cargoSha256 = "1wx974dy7wac541nb3ci8jl5zr96q0qx0b2w1kv0802dmkkk4rsp";
  verifyCargoDeps = true;

  meta = with stdenv.lib; {
    description =
      "A fast line-oriented regex search tool, similar to ag and ack";
    homepage = "https://github.com/mrvandalo/taskwarrior-hooks";
    license = licenses.gplv3;
    maintainers = [ maintainers.mrVanDalo ];
    platforms = platforms.all;
  };
}

