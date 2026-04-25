{
  description = "Minimial viable version of shell to build MacOs targets without XCode/CLT installed";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      ...
    }:
    let
      system = "aarch64-darwin";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };
      rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      macosDeploymentTarget = "15";
      appleSdk = pkgs."apple-sdk_${macosDeploymentTarget}";
      sdkRoot = "${appleSdk}/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk";
      llvmVersion = "19";
      llvmPackages = pkgs."llvmPackages_${llvmVersion}";
      clang = llvmPackages.clang-unwrapped;
      clangResourceDir = "${clang.lib}/lib/clang/${llvmVersion}";
      compilerRt = llvmPackages.compiler-rt-libc;
      lld = llvmPackages.lld;
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          rustToolchain
          rust-analyzer
          pkg-config
          clang
          lld
          appleSdk
          darwin.cctools
          libiconv
        ];
        MACOSX_DEPLOYMENT_TARGET = "${macosDeploymentTarget}.0";
        RUSTFLAGS = "-C link-arg=-isysroot -C link-arg=${sdkRoot} -C link-arg=-L${compilerRt}/lib/darwin -C link-arg=-mmacosx-version-min=${macosDeploymentTarget}.0 -C link-arg=-fuse-ld=${lld}/bin/ld64.lld";
        SDKROOT = sdkRoot;
        shellHook = ''
          export DEVELOPER_DIR="${appleSdk}"; # something overrides it on activation, so set here
          export CC="''${CIDRE_CC:-${clang}/bin/clang}"
          export CXX="''${CIDRE_CXX:-${clang}/bin/clang++}"
          export AR="''${CIDRE_AR:-${pkgs.darwin.cctools}/bin/ar}"
          export CFLAGS="''${CIDRE_CFLAGS:--resource-dir ${clangResourceDir} -isysroot $SDKROOT -mmacosx-version-min=${macosDeploymentTarget}.0}"
          export CXXFLAGS="''${CIDRE_CXXFLAGS:-$CFLAGS}"
          export CARGO_TARGET_AARCH64_APPLE_DARWIN_LINKER="$CC"
        '';
      };
    };
}
