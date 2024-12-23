#!/bin/bash

# Get the absolute path to the script's directory
script_dir=$(cd "$(dirname "$0")" && pwd)

# Set the path to Cargo.toml
cargo_toml="${script_dir}/../Cargo.toml"

# Set the directories to search in
search_dirs=(
    "${script_dir}/../src/"
    "${script_dir}/../src/modules/"
    "${script_dir}/../benches/"
    "${script_dir}/../examples/"
    # "${script_dir}/../tests/"
)

# Verify directories
for dir in "${search_dirs[@]}"; do
    if [[ ! -d "${dir}" ]]; then
        echo "Directory does not exist: ${dir}"
    fi
done

# Extract dependency names from the `[dependencies]` section
dependencies=$(awk '/\[dependencies\]/ {flag=1; next} /^\[/{flag=0} flag {print}' "${cargo_toml}" | grep -oE '^[a-zA-Z0-9_-]+' || true)

# Iterate over each dependency
while read -r dep; do
    # Skip empty lines
    [[ -z "${dep}" ]] && continue

    # Prepare a pattern to match Rust module imports
    dep_pattern=$(echo "${dep}" | tr '-' '_')

    # Check if the dependency is used in any of the specified directories
    found=false
    for dir in "${search_dirs[@]}"; do
        if grep -qir "${dep_pattern}" "${dir}"; then
            found=true
            break
        fi
    done

        # Special case for xml-rs: explicitly check for `xml` usage
    if [[ "${found}" = false ]]; then
        if [[ "${dep}" == "xml-rs" ]]; then
            # Check specifically for `xml` usage in the codebase
            xml_found=false
            for dir in "${search_dirs[@]}"; do
                if grep -qir "\bxml\b" "${dir}"; then
                    xml_found=true
                    break
                fi
            done

            if [[ "${xml_found}" = true ]]; then
                continue
            fi
        fi

        # Mark as unused if not found and not a special case
        printf "🗑️ The \033[1m%s\033[0m crate is not required!\n" "${dep}"
    fi


done <<< "${dependencies}"
