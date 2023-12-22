#!/bin/sh
# shellcheck enable=add-default-case
# shellcheck enable=avoid-nullary-conditions
# shellcheck enable=check-unassigned-uppercase
# shellcheck enable=deprecate-which
# shellcheck enable=quote-safe-variables
# shellcheck enable=require-variable-braces
set -eu

rm -rf /tmp/rivet_cli_install
mkdir /tmp/rivet_cli_install
cd /tmp/rivet_cli_install

UNAME="$(uname -s)"
ARCH="$(uname -m)"

# Find asset suffix
if [ "$(printf '%s' "$UNAME" | cut -c 1-6)" = "Darwin" ]; then
	echo
	echo "> Detected macOS"

	echo
	echo "> Installing jq"
	if [ "$ARCH" = "x86_64" ]; then
		url="https://github.com/jqlang/jq/releases/download/jq-1.7/jq-macos-amd64"
	elif [ "$ARCH" = "arm64" ]; then
		url="https://github.com/jqlang/jq/releases/download/jq-1.7/jq-macos-arm64"
	else
		echo "Unknown arch $ARCH" 1>&2
		exit 1
	fi
	curl -fsSL "$url" -o ./jq
	chmod +x ./jq

	if [ "$ARCH" = "x86_64" ]; then
		ASSET_NAME="rivet-cli-x86_64-apple-darwin"
	elif [ "$ARCH" = "arm64" ]; then
		ASSET_NAME="rivet-cli-aarch64-apple-darwin"
	else
		echo "Unknown arch $ARCH" 1>&2
		exit 1
	fi
elif [ "$(printf '%s' "$UNAME" | cut -c 1-5)" = "Linux" ]; then
	echo
	echo "> Detected Linux ($(getconf LONG_BIT) bit)"

	echo
	echo "> Installing jq"
	curl -fsSL "https://github.com/stedolan/jq/releases/download/jq-1.7/jq-linux$(getconf LONG_BIT)" -o ./jq
	chmod +x ./jq

	ASSET_NAME="rivet-cli-x86_64-unknown-linux-gnu"
else
	echo "Unable to determine platform" 1>&2
	exit 1
fi

FILE_NAME="${ASSET_NAME}.tar.xz"

# Determine install location
set +u
if [ -z "$BIN_DIR" ]; then
	BIN_DIR="/usr/local/bin"
fi
set -u
INSTALL_PATH="$BIN_DIR/rivet"

if [ ! -d "$BIN_DIR" ]; then
    # Find the base parent directory. We're using mkdir -p, which recursively creates directories, so we can't rely on `dirname`.
    CHECK_DIR="$BIN_DIR"
    while [ ! -d "$CHECK_DIR" ] && [ "$CHECK_DIR" != "/" ]; do
        CHECK_DIR=$(dirname "$CHECK_DIR")
    done

    # Check if the directory is writable
    if [ ! -w "$CHECK_DIR" ]; then
        echo
        echo "> Creating directory $BIN_DIR (requires sudo)"
        sudo mkdir -p "$BIN_DIR"
    else
        echo
        echo "> Creating directory $BIN_DIR"
        mkdir -p "$BIN_DIR"
    fi

fi

# Find latest CLI version to download
set +u
if [ -z "$RIVET_CLI_VERSION" ]; then
	echo
	echo "> Fetching latest release version for $ASSET_NAME"
	RIVET_CLI_VERSION="$( \
		curl -fsSL https://api.github.com/repos/rivet-gg/cli/releases \
		| ./jq -re \
			--arg file_name "$FILE_NAME" \
			'[.[] | select(.prerelease == false and (.assets[] | select(.name | endswith($file_name))))] | first | .tag_name' \
	)"
fi
set -u

# Download CLI
URL="https://github.com/rivet-gg/cli/releases/download/${RIVET_CLI_VERSION}/${FILE_NAME}"
echo
echo "> Downloading $URL"
curl -fsSL "$URL" -o rivet_cli.tar.xz

# Extract
echo
echo "> Extracting rivet.tar.xz"
tar xJf rivet_cli.tar.xz

# Move binary
if [ ! -w "$BIN_DIR" ]; then
    echo
    echo "> Installing rivet to $INSTALL_PATH (requires sudo)"
    sudo mv "./${ASSET_NAME}/rivet-cli" "$INSTALL_PATH"
else
    echo
    echo "> Installing rivet to $INSTALL_PATH"
    mv "./${ASSET_NAME}/rivet-cli" "$INSTALL_PATH"
fi

# Check if path may be incorrect
case ":$PATH:" in
	*:$BIN_DIR:*) ;;
	*) echo "WARNING: $BIN_DIR is not in \$PATH" ;;
esac

echo
echo "> Checking installation"
"$BIN_DIR/rivet" --version

echo
echo "Rivet was installed successfully."
echo "Run 'rivet --help' to get started."
