{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane.url = "github:ipetkov/crane";
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay, crane, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        craneLib = crane.mkLib pkgs;
        rustToolchain = pkgs.rust-bin.nightly."2025-06-04".default;

        config = builtins.fromTOML (builtins.readFile ./config.toml);

        env = {
          PKG_CONFIG_PATH = "${pkgs.openslide}/lib/pkgconfig";
          LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
          RUSTC_LINKER = "${pkgs.llvmPackages.clangUseLLVM}/bin/clang";
          RUSTFLAGS = "-Z threads=8";
        } // config.env;

        devDeps = with pkgs; [
          bun
          cargo
          rustfmt
        ];

        nativeBuildDeps = with pkgs; [
          clang
          cmake
          nasm
          rustToolchain
          llvmPackages_latest.llvm
          llvmPackages_latest.lld
        ];

        buildDeps = with pkgs; [
          libjpeg
          pkg-config
          openslide
          sqlite
          # OpenSlide dependencies.
          cairo
          expat
          gdk-pixbuf
          glib
          lerc
          libdicom
          libselinux
          libsepol
          libsysprof-capture
          libxml2
          openjpeg
          pcre2
          util-linux.dev
          xorg.libXdmcp
        ];

        # Install node_modules.
        node_modules = pkgs.stdenv.mkDerivation {
          pname = "frontend-node-modules";
          version = "0.0.0";
          src = ./frontend;

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

          outputHash = "sha256-zRzvj7xK5GKqpSbxPdyGm0JY/k+BtCxAZEbSCzJoZ2E=";
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

          outputHash = "sha256-j6Fscztb/MmiiO8+1X62Cdqxu6iMEQNx6oPqevVXC5g=";
          outputHashAlgo = "sha256";
          outputHashMode = "recursive";
        };

        # Backend build.
        backend = craneLib.buildPackage {
          pname = "backend";
          version = "0.0.0";
          src = craneLib.cleanCargoSource ./backend;

          env = env;
          nativeBuildInputs = nativeBuildDeps ++ buildDeps;
          buildInputs = buildDeps;

          cargoHash = "sha256-oC7BeeffeV8pdJlS+/yOJ8XLrdZaWHoBZyrL1GXglSg=";
        };

        runScript = pkgs.writeShellScriptBin "run" ''
          rm -rf ./_static
          ln -s ${self.packages.${system}.default}/_static ./_static
          ${pkgs.lib.concatStringsSep "\n" (pkgs.lib.mapAttrsToList (k: v: "export ${k}=${pkgs.lib.escapeShellArg v}") env)}
          echo ""
          if [ -n "$FRONTEND_PORT" ]; then
            echo "> Frontend ............. http://localhost:$FRONTEND_PORT"
            echo "> Backend  ............. http://localhost:$PUBLIC_PORT"
          else
            echo "> Running ............. http://localhost:$PUBLIC_PORT"
          fi
          exec ${self.packages.${system}.default}/core "$@"
        '';

        podmanRunScript = pkgs.writeShellScriptBin "podman" ''
          echo "Loading podman container..."
          podman load < ${self.packages.${system}.container}
          podman run --rm -it -p 3000:3000 -e CONTAINER=true localhost/magie:latest
        '';

        dockerRunScript = pkgs.writeShellScriptBin "docker" ''
          echo "Loading docker container..."
          docker load < ${self.packages.${system}.container}
          docker run --rm -it -p 3000:3000 -e CONTAINER=true localhost/magie:latest
        '';

        devRunScript = pkgs.writeShellScriptBin "dev" ''
          cd backend && cargo run & \
          cd backend/geometry-computer && bun install & \
          cd frontend && bun install && bun run dev
        '';
      in
      {
        # nix develop
        devShells.default = pkgs.mkShell {
          env = env;
          buildInputs = devDeps ++ nativeBuildDeps ++ buildDeps;

          shellHook = ''
            echo ""
            echo "Development environment ready."
            echo "Run: nix run .#dev"
          '';
        };

        # nix build
        packages.default = pkgs.stdenv.mkDerivation {
          pname = "magie";
          version = "0.0.0";
          buildCommand = ''
            mkdir -p $out
            mkdir -p $out/_static/
            cp ${backend}/bin/* $out
            cp -r ${frontend}/build/* $out/_static/
          '';
        };

        # nix build .#container
        packages.container = pkgs.dockerTools.buildLayeredImage {
          name = "magie";
          tag = "latest";
          contents = [pkgs.coreutils];
          config = {
            Cmd = ["${runScript}/bin/run"];
            ExposedPorts = {
              "3000/tcp" = {};
            };
            Volumes = {
              "/_databases" = { };
              "/_stores" = { };
            };
          };
        };

        apps = {
          # nix run
          default = {
            type = "app";
            program = "${runScript}/bin/run";
          };
          # nix run .#podman
          podman = {
            type = "app";
            program = "${podmanRunScript}/bin/podman";
          };
          # nix run .#docker
          docker = {
            type = "app";
            program = "${dockerRunScript}/bin/docker";
          };
          # nix run .#dev
          dev = {
            type = "app";
            program = "${devRunScript}/bin/dev";
          };
        };
      }
    );
 }
