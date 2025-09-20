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

        config = builtins.fromTOML (builtins.readFile ./config.toml);

        env = {
          PKG_CONFIG_PATH = "${pkgs.openslide}/lib/pkgconfig";
          LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
          RUSTC_LINKER = "${pkgs.llvmPackages.clangUseLLVM}/bin/clang";
          RUSTFLAGS = "-Z threads=8";
        } // config.env;

        backend_module = import ./nix/backend.nix { inherit pkgs crane rust-overlay env; };
        frontend_module = import ./nix/frontend.nix { inherit pkgs env; };

        backend = backend_module.backend;
        frontend = frontend_module.frontend;

        devDeps = with pkgs; [
          bun
          cargo
          rustfmt
        ];

        # Combined application
        magie = pkgs.stdenv.mkDerivation {
          pname = "magie";
          version = "0.0.0";
          buildCommand = ''
            mkdir -p $out
            mkdir -p $out/_static/
            cp ${backend}/bin/* $out
            cp -r ${frontend}/build/* $out/_static/
          '';
        };

        # Runtime scripts
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
          buildInputs = devDeps ++ backend_module.nativeBuildDeps ++ backend_module.buildDeps;

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
