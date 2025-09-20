{ pkgs, crane, rust-overlay, env }:

let
  inherit (pkgs) lib;
  craneLib = crane.mkLib pkgs;
  rustToolchain = pkgs.rust-bin.nightly.latest.default;

  src = craneLib.cleanCargoSource ../backend;

  nativeBuildDeps = with pkgs; [
    clang
    cmake
    nasm
    rustToolchain
    llvmPackages_latest.llvm
    llvmPackages_latest.lld
  ];

  buildDeps = with pkgs; [
    nodejs_24
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

  commonArgs = {
    inherit src;
    strictDeps = true;

    env = env;
    nativeBuildInputs = nativeBuildDeps ++ buildDeps;
    buildInputs = buildDeps;
  };

  # Build just the cargo dependencies of the entire workspace.
  cargoArtifacts = craneLib.buildDepsOnly commonArgs;

  # Individual crate builds.
  individualCrateArgs = commonArgs // {
    inherit cargoArtifacts;
    inherit (craneLib.crateNameFromCargoToml { inherit src; }) version;
    doCheck = false;
  };

  fileSetForCrate =
    crate:
    lib.fileset.toSource {
      root = ../backend;
      fileset = lib.fileset.unions [
        (lib.fileset.maybeMissing ../Cargo.toml)
        (lib.fileset.maybeMissing ../Cargo.lock)
        (craneLib.fileset.commonCargoSources crate)
      ];
    };

  # Individual crate packages
  core = craneLib.buildPackage (
    individualCrateArgs
    // {
      pname = "core";
      cargoExtraArgs = "-p core";
      src = fileSetForCrate ../backend/core;
    }
  );

  decoders = craneLib.buildPackage (
    individualCrateArgs
    // {
      pname = "decoders";
      cargoExtraArgs = "-p decoders";
      src = fileSetForCrate ../backend/decoders;
    }
  );

  encoders = craneLib.buildPackage (
    individualCrateArgs
    // {
      pname = "encoders";
      cargoExtraArgs = "-p encoders";
      src = fileSetForCrate ../backend/encoders;
    }
  );

  generators = craneLib.buildPackage (
    individualCrateArgs
    // {
      pname = "generators";
      cargoExtraArgs = "-p generators";
      src = fileSetForCrate ../backend/generators;
    }
  );

  shared = craneLib.buildPackage (
    individualCrateArgs
    // {
      pname = "shared";
      cargoExtraArgs = "-p shared";
      src = fileSetForCrate ../backend/shared;
    }
  );

  # Build the entire backend workspace as one package
  backend = craneLib.buildPackage (individualCrateArgs // {
    pname = "core";
    # Build all workspace members
    cargoExtraArgs = "--workspace";
  });
in
{
  inherit backend nativeBuildDeps buildDeps;
  inherit core decoders encoders generators shared;
}
