#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."
source scripts/env.sh

missing=()
os=$(uname -s)
case "$os" in
  Darwin)
    command -v brew >/dev/null || { echo 'Install Homebrew before running just setup.' >&2; exit 1; }
    command -v git >/dev/null || missing+=(git)
    command -v git-lfs >/dev/null || missing+=(git-lfs)
    command -v bsdtar >/dev/null || missing+=(libarchive)
    command -v curl >/dev/null || missing+=(curl)
    command -v jq >/dev/null || missing+=(jq)
    command -v unzip >/dev/null || missing+=(unzip)
    if [[ -n ${missing[*]-} ]]; then brew install "${missing[@]}"; fi
    command -v cc >/dev/null || { echo 'Install Xcode Command Line Tools: xcode-select --install' >&2; exit 1; }
    ;;
  Linux)
    if command -v apt-get >/dev/null; then
      manager=apt
      command -v bsdtar >/dev/null || missing+=(libarchive-tools)
      command -v cc >/dev/null || missing+=(build-essential)
    elif command -v dnf >/dev/null; then
      manager=dnf
      command -v bsdtar >/dev/null || missing+=(bsdtar)
      command -v cc >/dev/null || missing+=(gcc)
      command -v make >/dev/null || missing+=(make)
    elif command -v pacman >/dev/null; then
      manager=pacman
      command -v bsdtar >/dev/null || missing+=(libarchive)
      command -v cc >/dev/null || missing+=(base-devel)
    else
      echo 'Linux setup requires apt-get, dnf, or pacman; install the required tools manually on other distributions.' >&2
      exit 1
    fi
    command -v git >/dev/null || missing+=(git)
    command -v git-lfs >/dev/null || missing+=(git-lfs)
    command -v curl >/dev/null || missing+=(curl)
    command -v jq >/dev/null || missing+=(jq)
    command -v zip >/dev/null || missing+=(zip)
    command -v unzip >/dev/null || missing+=(unzip)
    if [[ -n ${missing[*]-} ]]; then
      run_package_manager() {
        if [[ $(id -u) == 0 ]]; then
          "$@"
        else
          command -v sudo >/dev/null || { echo 'Install sudo or run setup as root to install system packages.' >&2; exit 1; }
          sudo "$@"
        fi
      }
      case "$manager" in
        apt) run_package_manager apt-get update; run_package_manager apt-get install -y "${missing[@]}" ;;
        dnf) run_package_manager dnf install -y "${missing[@]}" ;;
        pacman) run_package_manager pacman -S --needed --noconfirm "${missing[@]}" ;;
      esac
    fi
    ;;
  MSYS*|MINGW*)
    command -v pacman >/dev/null || { echo 'Windows setup requires MSYS2 Bash with pacman, not Git Bash or WSL. Install MSYS2 and use its UCRT64 terminal.' >&2; exit 1; }
    [[ $(uname -m) == x86_64 ]] || { echo 'Windows setup supports x86_64 only.' >&2; exit 1; }
    command -v git >/dev/null || missing+=(git)
    command -v git-lfs >/dev/null || missing+=(mingw-w64-ucrt-x86_64-git-lfs)
    command -v bsdtar >/dev/null || missing+=(bsdtar)
    command -v curl >/dev/null || missing+=(curl)
    command -v jq >/dev/null || missing+=(jq)
    command -v zip >/dev/null || missing+=(zip)
    command -v unzip >/dev/null || missing+=(unzip)
    command -v sha256sum >/dev/null || missing+=(coreutils)
    if [[ -n ${missing[*]-} ]]; then pacman -S --needed --noconfirm "${missing[@]}"; fi
    command -v powershell.exe >/dev/null || { echo 'Windows setup requires Windows PowerShell.' >&2; exit 1; }
    # rustc locates MSVC through Visual Studio; cl.exe need not already be on PATH.
    # PowerShell, not Bash, expands these variables.
    # shellcheck disable=SC2016
    if ! powershell.exe -NoProfile -NonInteractive -Command '$v = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"; if (!(Test-Path $v)) { exit 1 }; $p = & $v -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath; if (!$p) { exit 1 }' >/dev/null; then
      echo 'Install Microsoft C++ Build Tools with the Desktop development with C++ workload and Windows SDK: https://visualstudio.microsoft.com/visual-cpp-build-tools/' >&2
      exit 1
    fi
    ;;
  *) echo 'Setup supports macOS, Linux (apt/dnf/pacman), and native Windows with MSYS2.' >&2; exit 1 ;;
esac

if ! command -v rustup >/dev/null; then
  tmp=$(mktemp -d)
  trap 'rm -rf "$tmp"' EXIT
  case "$os" in
    MSYS*|MINGW*)
      url=https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe
      curl --proto '=https' --tlsv1.2 -fsSL "$url" -o "$tmp/rustup-init.exe"
      curl --proto '=https' --tlsv1.2 -fsSL "$url.sha256" -o "$tmp/rustup-init.sha256"
      read -r expected _ < "$tmp/rustup-init.sha256"
      actual=$(sha256sum "$tmp/rustup-init.exe")
      if [[ ${actual%% *} != "$expected" ]]; then echo 'rustup-init checksum mismatch.' >&2; exit 1; fi
      "$tmp/rustup-init.exe" -y --no-modify-path --default-host x86_64-pc-windows-msvc --default-toolchain none
      ;;
    *)
      curl --proto '=https' --tlsv1.2 -fsSL https://sh.rustup.rs -o "$tmp/rustup-init.sh"
      sh "$tmp/rustup-init.sh" -y --no-modify-path --default-toolchain none
      ;;
  esac
fi
# Install the toolchain and components selected by the root rust-toolchain.toml.
rustup show active-toolchain || rustup toolchain install
case "$os" in
  MSYS*|MINGW*)
    if ! rustc -vV | grep -q '^host: x86_64-pc-windows-msvc$'; then
      echo 'The active Rust toolchain must use x86_64-pc-windows-msvc. Select an MSVC toolchain for this project; setup will not change your default host.' >&2
      exit 1
    fi
    ;;
esac
if ! command -v bun >/dev/null; then
  case "$os" in
    Darwin) brew install oven-sh/bun/bun ;;
    MSYS*|MINGW*)
      tmp=${tmp:-$(mktemp -d)}
      trap 'rm -rf "$tmp"' EXIT
      curl --proto '=https' --tlsv1.2 -fsSL https://bun.com/install.ps1 -o "$tmp/bun-install.ps1"
      if [[ -n ${BUN_INSTALL:-} ]]; then
        BUN_INSTALL=$(cygpath -w "$BUN_INSTALL") powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "$(cygpath -w "$tmp/bun-install.ps1")" -NoPathUpdate -NoRegisterInstallation -NoCompletions
      else
        powershell.exe -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "$(cygpath -w "$tmp/bun-install.ps1")" -NoPathUpdate -NoRegisterInstallation -NoCompletions
      fi
      command -v bun >/dev/null || { echo 'Bun installation failed; see the installer output.' >&2; exit 1; }
      ;;
    *)
      tmp=${tmp:-$(mktemp -d)}
      trap 'rm -rf "$tmp"' EXIT
      curl --proto '=https' --tlsv1.2 -fsSL https://bun.com/install -o "$tmp/bun-install.sh"
      bash "$tmp/bun-install.sh"
      ;;
  esac
fi
for pair in 'prek:prek' 'taplo:taplo-cli' 'cargo-deny:cargo-deny' 'cargo-shear:cargo-shear'; do
  program=${pair%%:*}
  package=${pair#*:}
  command -v "$program" >/dev/null || cargo install --locked "$package"
done

# LFS must install its pre-push hook before prek migrates it to pre-push.legacy.
hook=$(git rev-parse --git-path hooks/pre-push)
if [[ -e "$hook.legacy" ]] && grep -q 'git-lfs' "$hook.legacy"; then
  git lfs install --local --skip-repo
else
  if [[ -f "$hook" && ! -e "$hook.legacy" ]] && grep -q 'File generated by prek' "$hook"; then
    prek uninstall --hook-type pre-push
  fi
  git lfs install --local
fi
prek install
printf '%s\n' 'Setup complete. Run just build.'
