#!/bin/bash

# Define operating systems and their dependencies
declare -A deps=(
    ["debian"]="build-essential cmake curl nasm pkg-config libclang-dev libopenslide-dev libssl-dev"
    ["fedora"]="cmake curl nasm pkg-config clang-devel openslide-devel openssl-devel"
    ["ubuntu"]="cmake curl nasm pkg-config libclang-dev libopenslide-dev libssl-dev"
)

select os in "${!deps[@]}"; do
    echo ""

    # TODO: Separate per decoder/generator.
    echo "Do you want to proceed with installing the following dependencies?"
    for dep in ${deps["$os"]}; do
        echo "  $dep"
    done

    echo "  rustup"
    echo "  bun"
    echo ""

    # Check if the response is 'y' or 'n'
    while true; do
        read -p "(y/n): " response
        if [[ "$response" =~ ^[Yy]$ ]]; then
            echo "Installing..."
            break
        elif [[ "$response" =~ ^[Nn]$ ]]; then
            exit 0
        else
            echo "Invalid input. Please enter 'y' or 'n'."
        fi
    done

    case "$os" in
        debian|ubuntu)
            eval "
                sudo apt-get install -y ${deps["$os"]} && \
                curl -fsSL https://bun.sh/install | bash && \
                curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
                "
            ;;
        fedora)
            eval "
                sudo dnf install -y ${deps["$os"]} && \
                curl -fsSL https://bun.sh/install | bash && \
                curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
                "
            ;;
    esac
done