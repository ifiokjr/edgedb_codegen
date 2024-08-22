{ pkgs, lib, ... }:

{
  packages = [
    pkgs.cargo-binstall
    pkgs.cargo-run-bin
    pkgs.coreutils
    pkgs.curl
    pkgs.dprint
    pkgs.edgedb
    pkgs.fnm
    pkgs.jq
    pkgs.libiconv # needed for prisma to run
    pkgs.nil
    pkgs.nixpkgs-fmt
    pkgs.rustup
    pkgs.shfmt
    pkgs.taplo
  ] ++ lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin.apple_sdk; [
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
  scripts."update:deps".exec = ''
    set -e
    cargo update
    pnpm update --latest --recursive -i
    copy:public
  '';
  scripts."fix:all".exec = ''
    set -e
    fix:clippy
    fix:es
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
  scripts."fix:es".exec = ''
    set -e
    pnpm eslint --fix .
  '';
  scripts."lint:all".exec = ''
    set -e
    lint:clippy
    lint:es
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
  scripts."lint:es".exec = ''
    set -e
    pnpm eslint .
  '';
  scripts."snapshot:review".exec = ''
    cargo insta review
  '';
  scripts."snapshot:update".exec = ''
    cargo nextest run
    cargo insta accept
  '';
  scripts."test:docs".exec = ''
    set -e
    cargo test --doc
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
    echo "$GITHUB_WORKSPACE/node_modules/.bin" >> $GITHUB_PATH
    echo "$GITHUB_WORKSPACE/.local-cache/solana-release/bin" >> $GITHUB_PATH
    echo "$GITHUB_WORKSPACE/.local-cache/pulumi" >> $GITHUB_PATH
    echo "$GITHUB_WORKSPACE/.local-cache/esc" >> $GITHUB_PATH

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

    fnm_env=$(fnm env --json)

    # Parse the JSON file contents
    PARSED_FNM_ENV=$(jq -r '.' <<< "$fnm_env")
    FNM_MULTISHELL_PATH=$(jq -r '.FNM_MULTISHELL_PATH' <<< "$PARSED_FNM_ENV")

    # Add fnm to the path
    echo "$FNM_MULTISHELL_PATH/bin" >> $GITHUB_PATH

    # add fnm environment variables
    for key in $(jq -r 'keys[]' <<< "$PARSED_FNM_ENV"); do
      value=$(jq -r ".$key" <<< "$PARSED_FNM_ENV")
      echo "$key=$value" >> $GITHUB_ENV
    done
  '';
  scripts."setup:docker".exec = ''
    set -e
    # update path
    echo "export PATH=$DEVENV_PROFILE/bin:\$PATH" >> /etc/profile
    echo "export PATH=$DEVENV_ROOT/node_modules/.bin:\$PATH" >> /etc/profile
    echo "export PATH=$DEVENV_ROOT/.local-cache/solana-release/bin:\$PATH" >> /etc/profile
    echo "export PATH=$DEVENV_ROOT/.local-cache/pulumi:\$PATH" >> /etc/profile
    echo "export PATH=$DEVENV_ROOT/.local-cache/esc:\$PATH" >> /etc/profile

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

    fnm_env=$(fnm env --json)

    # Parse the JSON file contents
    PARSED_FNM_ENV=$(jq -r '.' <<< "$fnm_env")
    FNM_MULTISHELL_PATH=$(jq -r '.FNM_MULTISHELL_PATH' <<< "$PARSED_FNM_ENV")

    # add fnm to the path
    echo "export PATH=$FNM_MULTISHELL_PATH/bin:\$PATH" >> /etc/profile

    # add fnm environment variables
    for key in $(jq -r 'keys[]' <<< "$PARSED_FNM_ENV"); do
      value=$(jq -r ".$key" <<< "$PARSED_FNM_ENV")
      echo "export $key=$value" >> /etc/profile
    done
  '';
}
