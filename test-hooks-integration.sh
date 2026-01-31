#!/bin/bash

# Integration test for fnm hooks functionality
# This script demonstrates the install hooks in action

set -e

echo "🧪 Testing fnm install hooks functionality"
echo "============================================="

# Set up test environment
TEST_DIR="/tmp/fnm-hooks-integration-test"
FNM_DIR="$TEST_DIR/.fnm"
HOOKS_DIR="$FNM_DIR/hooks"

# Clean up any previous test
rm -rf "$TEST_DIR"
mkdir -p "$HOOKS_DIR"

echo "📁 Test directory: $TEST_DIR"
echo "📁 FNM directory: $FNM_DIR"
echo "📁 Hooks directory: $HOOKS_DIR"

# Create test hooks
cat > "$HOOKS_DIR/pre-install" << 'EOF'
#!/bin/bash
echo "🪝 PRE-INSTALL HOOK EXECUTED!"
echo "  Version: $FNM_VERSION"
echo "  Arch: $FNM_ARCH"
echo "  FNM Dir: $FNM_DIR"
echo "  Installation Dir: $FNM_INSTALLATION_DIR"
echo "  Working Dir: $(pwd)"
echo ""
EOF

cat > "$HOOKS_DIR/post-install" << 'EOF'
#!/bin/bash
echo "🎉 POST-INSTALL HOOK EXECUTED!"
echo "  Successfully installed: $FNM_VERSION"
echo "  Arch: $FNM_ARCH"
echo "  Installation completed in: $FNM_INSTALLATION_DIR"
echo ""
EOF

cat > "$HOOKS_DIR/install-failed" << 'EOF'
#!/bin/bash
echo "❌ INSTALL-FAILED HOOK EXECUTED!"
echo "  Failed to install: $FNM_VERSION"
echo "  Error occurred for arch: $FNM_ARCH"
echo ""
EOF

# Make hooks executable
chmod +x "$HOOKS_DIR"/*

echo "✅ Created test hooks:"
ls -la "$HOOKS_DIR"
echo ""

echo "🚀 Testing hooks with fnm install command..."
echo "Note: This will attempt to install Node.js 18.0.0 to demonstrate hooks"
echo ""

# Build fnm if not already built
FNM_BINARY="./target/release/fnm"
if [ ! -f "$FNM_BINARY" ]; then
    echo "Building fnm..."
    cargo build --release
fi

# Test the hooks by attempting an install
export FNM_DIR="$FNM_DIR"
echo "Using fnm from: $FNM_BINARY"
echo "Using FNM_DIR: $FNM_DIR"
echo ""

# Try to install a version (this will trigger hooks)
echo "Attempting to install Node.js v18.0.0..."
echo "This should trigger the pre-install hook, and then either post-install or install-failed hook."
echo ""

"$FNM_BINARY" install 18.0.0 || echo "Installation may have failed, but hooks should have executed"

echo ""
echo "🧹 Cleaning up test directory..."
rm -rf "$TEST_DIR"

echo "✅ Integration test completed!"
echo ""
echo "If you saw the hook messages above, the hooks system is working correctly!"
