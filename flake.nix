{
  description = "Tauri BLE Example – dev environment";

  inputs = {
    nixpkgs.url     = "github:NixOS/nixpkgs/release-25.11";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url    = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, fenix }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { 
          inherit system; 
          config = {
            allowUnfree = true;
            android_sdk.accept_license = true;
          };
        };

        rustToolchain = with fenix.packages.${system}; combine [
          stable.cargo
          stable.rustc
          stable.rust-src
          stable.rust-analyzer
          stable.clippy
          stable.rustfmt
          targets.aarch64-linux-android.stable.rust-std
        ];

        androidSdk = pkgs.androidenv.composeAndroidPackages {
          buildToolsVersions = [ "34.0.0" ];
          platformVersions   = [ "34" "36" ];
          abiVersions        = [ "arm64-v8a" ];
          includeNDK         = true;
          ndkVersions        = [ "26.1.10909125" ];
          includeEmulator    = false;
          includeSources     = false;
          includeSystemImages = false;
        };

      in {
        devShells.default = pkgs.mkShell {
          name = "tauri-ble-example";

          buildInputs = with pkgs; [
            rustToolchain
            nodejs_22
            cargo-tauri
            git
            webkitgtk_4_1
            gtk3
            libsoup_3
            dbus
            bluez
            # General build helpers
            glib
            pkg-config
            pango
            cairo
            atk
            gdk-pixbuf
            libayatana-appindicator
            librsvg
            openssl
            xdg-utils
            jdk17
            androidSdk.androidsdk

            (pkgs.python3.withPackages (ps: [
              ps.bleak
            ]))
          ];

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [
            dbus
            webkitgtk_4_1
            gtk3
            glib
            openssl
            gdk-pixbuf
            pango
            cairo
            atk
            libsoup_3
            libayatana-appindicator
          ]);

          JAVA_HOME        = "${pkgs.jdk17}/lib/openjdk";
          ANDROID_HOME     = "${androidSdk.androidsdk}/libexec/android-sdk";
          ANDROID_SDK_ROOT = "${androidSdk.androidsdk}/libexec/android-sdk";

          shellHook = ''
            export RUST_LOG="''${RUST_LOG:-debug}"
            export RUST_BACKTRACE="''${RUST_BACKTRACE:-1}"

            export PKG_CONFIG_PATH="${pkgs.webkitgtk_4_1.dev}/lib/pkgconfig:${pkgs.gtk3.dev}/lib/pkgconfig:${pkgs.openssl.dev}/lib/pkgconfig:${pkgs.dbus.dev}/lib/pkgconfig:''${PKG_CONFIG_PATH:-}"
            
            # Required by Tauri's AppImage bundler on Linux
            export XDG_DATA_DIRS="${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:''${XDG_DATA_DIRS:-}"

            cat > src-tauri/gen/android/local.properties <<EOF
              sdk.dir=$ANDROID_HOME
              keyAlias=debug
              keyPassword=android
              storeFile=$(echo $HOME)/.android/debug.keystore
              storePassword=android
            EOF

            export GRADLE_OPTS="-Dorg.gradle.project.android.aapt2FromMavenOverride=${androidSdk.androidsdk}/libexec/android-sdk/build-tools/34.0.0/aapt2"
          '';
        };
      }
    );
}
