#!/bin/bash
set -euo pipefail

# Downloads a local ENSRainbow dataset and extracts TSV for offline label healing.
# This script is an operator helper only; the indexer runtime never calls ENSRainbow APIs.

OUT_DIR="${ENSRAINBOW_OUT_DIR:-./healed-names}"
BASE_URL="https://bucket.ensrainbow.io"

if [ "${DATA_VERSION:-}" = "v2" ]; then
    echo "Downloading v2 rainbow tables (DATA_VERSION=v2)..."
    DATA_FILE="ensrainbow_v2.sql.gz"
    CHECKSUM_FILE="ensrainbow_v2.sql.gz.sha256sum"
elif [ "${DATA_VERSION:-}" = "test" ]; then
    echo "Downloading test environment data (DATA_VERSION=test)..."
    DATA_FILE="ens_test_env_names.sql.gz"
    CHECKSUM_FILE="ens_test_env_names.sql.gz.sha256sum"
elif [ -z "${DATA_VERSION:-}" ] || [ "${DATA_VERSION:-}" = "v1" ]; then
    echo "Downloading v1 rainbow tables (DATA_VERSION not set or empty)..."
    DATA_FILE="ens_names.sql.gz"
    CHECKSUM_FILE="ens_names.sql.gz.sha256sum"
else
    echo "Error: Invalid DATA_VERSION value '$DATA_VERSION'."
    echo "Allowed values are 'v2', 'test', 'v1' (default v1)."
    exit 1
fi

LICENSE_FILE="THE_GRAPH_LICENSE.txt"
TARGET_FILE="$OUT_DIR/ens_names.sql.gz"
TARGET_CHECKSUM_FILE="$OUT_DIR/ens_names.sql.gz.sha256sum"
TARGET_TSV_FILE="$OUT_DIR/ens_names.tsv"

download_with_progress() {
    local url="$1"
    local output="$2"
    local description="$3"

    echo "Downloading $description..."
    if command -v wget > /dev/null 2>&1; then
        wget -nv -O "$output" "$url"
    else
        curl -fL --progress-bar -o "$output" "$url"
    fi
}

verify_checksum() {
    local checksum_file="$1"

    if command -v sha256sum > /dev/null 2>&1; then
        sha256sum -c "$checksum_file"
        return
    fi

    local expected
    local file_name
    local actual
    expected="$(awk '{print $1}' "$checksum_file")"
    file_name="$(awk '{print $2}' "$checksum_file")"
    actual="$(shasum -a 256 "$(dirname "$checksum_file")/$file_name" | awk '{print $1}')"
    test "$expected" = "$actual"
}

extract_tsv() {
    echo "Extracting local label TSV..."
    gzip -cd "$TARGET_FILE" \
        | awk '
            /^COPY public\.ens_names \(hash, name\) FROM stdin;$/ { in_copy = 1; next }
            /^\\\.$/ { in_copy = 0; next }
            in_copy == 1 { print }
          ' > "$TARGET_TSV_FILE"
    echo "Wrote $TARGET_TSV_FILE"
}

mkdir -p "$OUT_DIR"

download_with_progress "$BASE_URL/$CHECKSUM_FILE" "$TARGET_CHECKSUM_FILE" "checksum file"

if [ -f "$TARGET_FILE" ] && [ -f "$TARGET_CHECKSUM_FILE" ]; then
    echo "Found existing files, verifying checksum..."
    if verify_checksum "$TARGET_CHECKSUM_FILE" > /dev/null 2>&1; then
        echo "Existing files are valid."
        extract_tsv
        exit 0
    fi
    echo "Checksum verification failed, downloading fresh files."
fi

download_with_progress "$BASE_URL/$LICENSE_FILE" "$OUT_DIR/$LICENSE_FILE" "license file"
download_with_progress "$BASE_URL/$DATA_FILE" "$TARGET_FILE" "ENS names database"

echo "Verifying downloaded files..."
verify_checksum "$TARGET_CHECKSUM_FILE"
extract_tsv
echo "Download successful and verified."
