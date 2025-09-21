{ pkgs, env }:

let
  # Install node_modules.
  node_modules = pkgs.stdenv.mkDerivation {
    pname = "frontend-node-modules";
    version = "0.0.0";
    src = ../frontend;

    nativeBuildInputs = [ pkgs.bun ];
    buildInputs = [ pkgs.nodejs-slim_latest ];

    dontConfigure = true;
    dontFixup = true;

    buildPhase = ''
      runHook preBuild
      export HOME=$TMPDIR
      bun install --frozen-lockfile
      runHook postBuild
    '';

    installPhase = ''
      runHook preInstall
      mkdir -p $out/node_modules
      mv node_modules $out/
      runHook postInstall
    '';

    outputHash = "sha256-RJT4PRnMbVeFpdCw0IFkPlw+rK99LMS70O+bSKL93ow=";
    outputHashAlgo = "sha256";
    outputHashMode = "recursive";
  };

  # Frontend build.
  frontend = pkgs.stdenv.mkDerivation {
    pname = "frontend";
    version = "0.0.0";
    src = ../frontend;

    env = env;
    nativeBuildInputs = [
        pkgs.bun
        pkgs.nodejs-slim_latest
        node_modules
    ];

    configurePhase = ''
      runHook preConfigure

      cp -a ${node_modules}/node_modules ./node_modules
      chmod -R u+rw node_modules
      chmod -R u+x node_modules/.bin
      patchShebangs node_modules

      export HOME=$TMPDIR
      export PATH="$PWD/node_modules/.bin:$PATH"

      runHook postConfigure
    '';

    buildPhase = ''
      runHook preBuild
      bun run build
      runHook postBuild
    '';

    installPhase = ''
      runHook preInstall
      mkdir -p $out
      mv ./build $out
      runHook postInstall
    '';

    outputHash = "sha256-SzvV5B1Vdzj701wmRlVADQYoNVz4Rc/ErvqHKHR3TtY=";
    outputHashAlgo = "sha256";
    outputHashMode = "recursive";
  };
in
{
  inherit frontend node_modules;
}
