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
        craneLib = crane.mkLib pkgs;
        pkgs = import nixpkgs { inherit system overlays; };

        config = builtins.fromTOML (builtins.readFile ./config.toml);
        env = {
          LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
          BINDGEN_EXTRA_CLANG_ARGS = "-isystem ${pkgs.llvmPackages.libclang.lib}/lib/clang/${pkgs.llvmPackages.libclang.version}/include";
          PKG_CONFIG_PATH = "${pkgs.openslide}/lib/pkgconfig";
        } // config.env;

        backendNativeBuildInputs = with pkgs; [
          clang
          cmake
          nasm
          rust-bin.stable."1.90.0".default
          llvmPackages.libclang
          pkg-config
        ];

        backendBuildInputs = with pkgs; [
          nodejs_24
          libjpeg
          openslide
          sqlite
          # OpenSlide dependencies.
          cairo
          expat
          gdk-pixbuf
          glib
          lerc
          libdicom
          libdeflate
          libselinux
          libsepol
          libsysprof-capture
          libwebp
          libxml2
          openjpeg
          pcre2
          util-linux.dev
          xorg.libXdmcp
          xz
          zstd
        ];

        devDeps = with pkgs; [
          bun
          cargo
          rustfmt
        ];

        geometry-computer = pkgs.buildNpmPackage {
          pname = "geometry-computer";
          version = "0.0.0";
          src = ./backend/geometry-computer;
          nodejs = pkgs.nodejs_24;

          env = env;
          npmDeps = pkgs.importNpmLock {
            npmRoot = ./backend/geometry-computer;
          };
          npmConfigHook = pkgs.importNpmLock.npmConfigHook;

          installPhase = ''
            runHook preInstall
            mkdir -p $out
            mv ./** $out
            runHook postInstall
          '';
        };

        backend = craneLib.buildPackage {
          pname = "backend";
          src = craneLib.cleanCargoSource ./backend;
          cargoExtraArgs = "--workspace";

          strictDeps = true;
          env = env;

          nativeBuildInputs = backendNativeBuildInputs;
          buildInputs = backendBuildInputs;
        };

        frontend = pkgs.buildNpmPackage {
          pname = "frontend";
          version = "0.0.0";
          src = ./frontend;
          nodejs = pkgs.nodejs_24;

          env = env;
          npmDeps = pkgs.importNpmLock {
            npmRoot = ./frontend;
          };
          npmConfigHook = pkgs.importNpmLock.npmConfigHook;

          installPhase = ''
            runHook preInstall
            mkdir -p $out
            mv ./build $out
            runHook postInstall
          '';
        };

        # Combined application
        magie = pkgs.stdenv.mkDerivation {
          pname = "magie";
          version = "0.0.0";
          buildCommand = ''
            mkdir -p $out
            mkdir -p $out/_static/
            mkdir -p $out/geometry-computer/
            cp ${backend}/bin/* $out
            cp -r ${geometry-computer}/* $out/geometry-computer/
            cp -r ${frontend}/build/* $out/_static/
          '';
        };

        # Runtime scripts
        runScript = pkgs.writeShellScriptBin "run" ''
          rm -rf ./_static
          ln -s ${self.packages.${system}.default}/_static ./_static
          ${pkgs.lib.concatStringsSep "\n" (pkgs.lib.mapAttrsToList (k: v: "export ${k}=${pkgs.lib.escapeShellArg v}") env)}
          echo "> Running ............. http://localhost:$PUBLIC_PORT"
          exec ${self.packages.${system}.default}/core "$@"
        '';

        podmanRunScript = pkgs.writeShellScriptBin "podman" ''
          echo "Loading podman container..."
          podman load < ${self.packages.${system}.container}
          podman run --rm -it -p $PUBLIC_PORT:$PUBLIC_PORT -e CONTAINER=true localhost/magie:latest
        '';

        dockerRunScript = pkgs.writeShellScriptBin "docker" ''
          echo "Loading docker container..."
          docker load < ${self.packages.${system}.container}
          docker run --rm -it -p $PUBLIC_PORT:$PUBLIC_PORT -e CONTAINER=true localhost/magie:latest
        '';

        devRunScript = pkgs.writeShellScriptBin "dev" ''
          cd backend && cargo run & \
          cd backend/geometry-computer && bun install & \
          cd frontend && npm install && npm run dev
        '';
      in
      {
        # nix develop
        devShells.default = pkgs.mkShell {
          env = env;
          buildInputs = devDeps ++ backendNativeBuildInputs ++ backendBuildInputs;

          shellHook = ''
            echo ""
            echo "Development environment ready."
            echo "Run: nix run .#dev"
          '';
        };

        # nix build
        packages = {
          default = magie;
          backend = backend;
          frontend = frontend;

          # nix build .#container
          container = pkgs.dockerTools.buildLayeredImage {
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
