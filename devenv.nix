{ pkgs, lib, ... }:

{
  packages = [
    pkgs.cargo-binstall
    pkgs.cargo-run-bin
    pkgs.coreutils
    pkgs.dprint
    pkgs.edgedb
    pkgs.nil
    pkgs.nixpkgs-fmt
    pkgs.rustup
    pkgs.shfmt
    pkgs.taplo
    pkgs.yamlfmt
  ] ++ lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin.apple_sdk; [
    pkgs.libiconv
    frameworks.Security
    frameworks.System
  ]);

  # disable dotenv since it breaks the variable interpolation supported by `direnv`
  dotenv.disableHint = true;

  scripts."install:all".exec = ''
    set -e
    install:cargo:bin
  '';
  scripts."install:cargo:bin".exec = ''
    cargo bin --install
  '';
  scripts."db:destroy".exec = ''
    set -e
    edgedb instance destroy -I $EDGEDB_INSTANCE --non-interactive --force
  '';
  scripts."db:setup".exec = ''
    set -e
    edgedb instance create --non-interactive $EDGEDB_INSTANCE $EDGEDB_BRANCH || true
    edgedb instance start --instance $EDGEDB_INSTANCE
    edgedb migrate
  '';
  scripts."db:up".exec = ''
    set -e
    edgedb watch --instance $EDGEDB_INSTANCE
  '';
  scripts."update:deps".exec = ''
    set -e
    cargo update
  '';
  scripts."fix:all".exec = ''
    set -e
    fix:clippy
    fix:format
  '';
  scripts."fix:format".exec = ''
    set -e
    dprint fmt --config "$DEVENV_ROOT/dprint.json"
  '';
  scripts."fix:clippy".exec = ''
    set -e
    cargo clippy --fix --allow-dirty --allow-staged --all-features
  '';
  scripts."lint:all".exec = ''
    set -e
    lint:clippy
    lint:format
  '';
  scripts."lint:format".exec = ''
    set -e
    dprint check
  '';
  scripts."lint:clippy".exec = ''
    set -e
    cargo clippy --all-features
  '';
  scripts."setup:vscode".exec = ''
    set -e
    rm -rf .vscode
    cp -r $DEVENV_ROOT/setup/editors/vscode .vscode
  '';
  scripts."setup:helix".exec = ''
    set -e
    rm -rf .helix
    cp -r $DEVENV_ROOT/setup/editors/helix .helix
  '';
  scripts."setup:ci".exec = ''
    set -e
    # update github ci path
    echo "$DEVENV_PROFILE/bin" >> $GITHUB_PATH

    # update github ci environment
    echo "DEVENV_PROFILE=$DEVENV_PROFILE" >> $GITHUB_ENV

    # prepend common compilation lookup paths
    echo "PKG_CONFIG_PATH=$PKG_CONFIG_PATH" >> $GITHUB_ENV
    echo "LD_LIBRARY_PATH=$LD_LIBRARY_PATH" >> $GITHUB_ENV
    echo "LIBRARY_PATH=$LIBRARY_PATH" >> $GITHUB_ENV
    echo "C_INCLUDE_PATH=$C_INCLUDE_PATH" >> $GITHUB_ENV

    # these provide shell completions / default config options
    echo "XDG_DATA_DIRS=$XDG_DATA_DIRS" >> $GITHUB_ENV
    echo "XDG_CONFIG_DIRS=$XDG_CONFIG_DIRS" >> $GITHUB_ENV

    echo "DEVENV_DOTFILE=$DEVENV_DOTFILE" >> $GITHUB_ENV
    echo "DEVENV_PROFILE=$DEVENV_PROFILE" >> $GITHUB_ENV
    echo "DEVENV_ROOT=$DEVENV_ROOT" >> $GITHUB_ENV
    echo "DEVENV_STATE=$DEVENV_STATE" >> $GITHUB_ENV
  '';
  scripts."setup:docker".exec = ''
    set -e
    # update path
    echo "export PATH=$DEVENV_PROFILE/bin:\$PATH" >> /etc/profile

    echo "export DEVENV_PROFILE=$DEVENV_PROFILE" >> /etc/profile
    echo "export PKG_CONFIG_PATH=$PKG_CONFIG_PATH" >> /etc/profile
    echo "export LD_LIBRARY_PATH=$LD_LIBRARY_PATH" >> /etc/profile
    echo "export LIBRARY_PATH=$LIBRARY_PATH" >> /etc/profile
    echo "export C_INCLUDE_PATH=$C_INCLUDE_PATH" >> /etc/profile
    echo "export XDG_DATA_DIRS=$XDG_DATA_DIRS" >> /etc/profile
    echo "export XDG_CONFIG_DIRS=$XDG_CONFIG_DIRS" >> /etc/profile

    echo "export DEVENV_DOTFILE=$DEVENV_DOTFILE" >> /etc/profile
    echo "export DEVENV_PROFILE=$DEVENV_PROFILE" >> /etc/profile
    echo "export DEVENV_ROOT=$DEVENV_ROOT" >> /etc/profile
    echo "export DEVENV_STATE=$DEVENV_STATE" >> /etc/profile
  '';
}
