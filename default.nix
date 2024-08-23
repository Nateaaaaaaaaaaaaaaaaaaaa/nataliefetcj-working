let
    pkgs = import <nixpkgs> {};
in
pkgs.rustPlatform.buildRustPackage {
    name = "nataliefetch";
    src = pkgs.fetchFromGitHub {
        owner = "Nateaaaaaaaaaaaaaaaaaaaa";
        repo = "nataliefetcj-working";
        rev = "235ad89";
        sha256 = "sha256-GJXFNwFnmm/FVoam631GOr8o5YJKuvcdOd1+sRe/8F0=";
    };
    cargoLock.lockFile = ./Cargo.lock;
}