{ rustPlatform, fetchFromGitHub, stdenv, ... }:

rustPlatform.buildRustPackage rec {
  name = "taskwarrior-hooks-${version}";
  version = "0.2.2";
  src = fetchFromGitHub {
    owner = "mrVanDalo";
    repo = "taskwarrior-hooks";
    rev = "${version}";
    sha256 = "1mj0k6ykac332667315kqrvg37j8r8078g48nafv7ini6lw8djas";
  };

  cargoSha256 = "1ijnh2ank9slmfglw4yhnycl11x26m94m2hiq3hcasmbs6c39zj5";
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

