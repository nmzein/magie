{ pkgs, env }:
let
  frontend = pkgs.buildNpmPackage {
    pname = "frontend";
    version = "0.0.0";
    src = ../frontend;
    nodejs = pkgs.nodejs_24;

    env = env;
    npmDeps = pkgs.importNpmLock {
      npmRoot = ../frontend;
    };
    npmConfigHook = pkgs.importNpmLock.npmConfigHook;

    installPhase = ''
      runHook preInstall
      mkdir -p $out
      mv ./build $out
      runHook postInstall
    '';
  };
in
{
  inherit frontend;
}
