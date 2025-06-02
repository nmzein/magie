{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };

        env = {
          PKG_CONFIG_PATH = "${pkgs.openslide}/lib/pkgconfig";
          LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
        };

        devDeps = with pkgs; [
          bun
          cargo
          rustc
          rustfmt
        ];

        buildDeps = with pkgs; [
          # Direct dependencies.
          libjpeg
          openslide
          pkg-config
          sqlite
          # OpenSlide dependencies.
          cairo
          clang
          cmake
          expat
          gdk-pixbuf
          glib
          lerc
          libdicom
          libselinux
          libsepol
          libsysprof-capture
          libxml2
          nasm
          openjpeg
          pcre2
          util-linux.dev
          xorg.libXdmcp
        ];

        node_modules = pkgs.stdenv.mkDerivation {
          pname = "frontend-node-modules";
          version = "0.0.0";
          src = ./frontend;

          nativeBuildInputs = [ pkgs.bun ];
          buildInputs = [ pkgs.nodejs-slim_latest ];

          dontConfigure = true;
          dontFixup = true; # patchShebangs produces illegal path references in FODs

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

          outputHash = "sha256-hLnFv2niHuu4ZMsp5qHwQgdosv5B90l9587UgEXcw4s=";
          outputHashAlgo = "sha256";
          outputHashMode = "recursive";
        };

        # Frontend build.
        frontend = pkgs.stdenv.mkDerivation {
          pname = "frontend";
          version = "0.0.0";
          src = ./frontend;

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
            echo "OUT IS $out"
            eval ls -al
            mkdir -p $out
            ln -s ${node_modules}/node_modules $out
            mv ./build $out
            runHook postInstall
          '';
        };

        # Backend build.
        backend = pkgs.rustPlatform.buildRustPackage {
          pname = "backend";
          version = "0.0.0";
          src = ./backend;

          nativeBuildInputs = buildDeps;
          buildInputs = buildDeps;
          env = env;

          cargoHash = "sha256-4Pvkj32JcEVQwGbMalZWyY/8JVJWXUALL/3YvAb8wFI=";
        };
      in
      {
        # nix develop
        devShells.default = pkgs.mkShell {
          buildInputs = devDeps ++ buildDeps;
          env = env;

          shellHook = ''
            echo "Environment ready."
            echo "Run: nix build"
          '';
        };

        # nix build
        packages = {
          # Separate packages.
          frontend = frontend;
          backend = backend;

          # Combined package (default).
          default = pkgs.stdenv.mkDerivation {
            pname = "full";
            version = "0.0.0";

            buildCommand = ''
              mkdir -p $out/bin
              mkdir -p $out/share/frontend

              # Copy backend binary
              cp ${backend}/bin/* $out/bin/

              # Copy frontend assets
              cp -r ${frontend}/* $out/share/frontend/
            '';
          };
        };

        # nix run
        apps.default = {
          type = "app";
          program = "${self.packages.${system}.backend}/bin/core";
        };
      }
    );
 }
