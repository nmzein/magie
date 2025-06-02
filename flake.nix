{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };

        config = builtins.fromTOML (builtins.readFile ./config.toml);

        env = {
          PKG_CONFIG_PATH = "${pkgs.openslide}/lib/pkgconfig";
          LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
        } // config;

        devDeps = with pkgs; [
          bun
          cargo
          rustc
          rustfmt
        ];

        buildDeps =  [
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

          outputHash = "sha256-gZj1SUelNyd650+WH4k0qkhbPjXU/Kerlu6jQwVjXgw=";
          outputHashAlgo = "sha256";
          outputHashMode = "recursive";
        };

        # Backend build.
        backend = pkgs.rustPlatform.buildRustPackage {
          pname = "backend";
          version = "0.0.0";
          src = ./backend;

          env = env;
          nativeBuildInputs = buildDeps;
          buildInputs = buildDeps;

          cargoHash = "sha256-2hjStRGO83euf6OW0qQgzon6DBIrg1O8FbyH+Lw9bPk=";
        };
      in
      {
        # nix develop
        devShells.default = pkgs.mkShell {
          env = env;
          buildInputs = devDeps ++ buildDeps;

          shellHook = ''
            echo "Development environment ready."
          '';
        };

        # nix build
        packages.default = pkgs.stdenv.mkDerivation {
            pname = "MAGIE";
            version = "0.0.0";
            buildCommand = ''
              mkdir -p $out
              mkdir -p $out/_static/
              cp ${backend}/bin/* $out
              cp -r ${frontend}/build/* $out/_static/
            '';
        };

        # nix run
        apps.default = {
          type = "app";
          program = "${self.packages.${system}.default}/core";
        };
      }
    );
 }
