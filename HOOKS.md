# fnm Hooks

fnm now supports Git-style hooks that allow you to execute custom scripts during various operations. This feature enables you to automate tasks such as environment setup, package installation, notifications, and cleanup.

## Hook Types

The following hooks are currently supported:

### Install Hooks

- **`pre-install`**: Executed before downloading and installing a Node.js version
- **`post-install`**: Executed after successful installation of a Node.js version  
- **`install-failed`**: Executed when Node.js installation fails

## Hook Location

Hooks should be placed in the `hooks/` subdirectory of your fnm directory:

- **Default location**: `~/.fnm/hooks/`
- **Custom location**: `$FNM_DIR/hooks/` (if you've set a custom FNM_DIR)

## Hook Script Requirements

1. **Executable**: Hook scripts must be executable (`chmod +x hook-name`)
2. **Any format**: Scripts can be written in any language (shell, Python, Node.js, etc.)
3. **Return code**: Hooks should return exit code 0 for success, non-zero for failure
4. **Error handling**: If a hook fails, fnm will log the error but continue with the operation

## Environment Variables

The following environment variables are available to all hooks:

- `FNM_VERSION`: The Node.js version being operated on (e.g., "v18.17.0")
- `FNM_ARCH`: The architecture being used (e.g., "x64", "arm64")
- `FNM_DIR`: The fnm installation directory
- `FNM_INSTALLATION_DIR`: The specific installation directory for this version

## Examples

### Pre-install Hook

```bash
#!/bin/bash
# ~/.fnm/hooks/pre-install

echo "🚀 About to install Node.js $FNM_VERSION"

# Example: Send notification
osascript -e "display notification \"Installing Node.js $FNM_VERSION\" with title \"fnm\""

# Example: Clean up old caches
npm cache clean --force 2>/dev/null || true
```

### Post-install Hook

```bash
#!/bin/bash
# ~/.fnm/hooks/post-install

echo "✅ Node.js $FNM_VERSION installed successfully"

# Example: Auto-install global packages
npm install -g pnpm yarn typescript

# Example: Set up development environment
if [ -f "./package.json" ]; then
    echo "Installing project dependencies..."
    npm install
fi

# Example: Send notification
osascript -e "display notification \"Node.js $FNM_VERSION ready!\" with title \"fnm\""
```

### Install-failed Hook

```bash
#!/bin/bash
# ~/.fnm/hooks/install-failed

echo "❌ Failed to install Node.js $FNM_VERSION"

# Example: Log failure
echo "$(date): Failed to install $FNM_VERSION" >> ~/.fnm/install-failures.log

# Example: Send error notification
osascript -e "display notification \"Failed to install Node.js $FNM_VERSION\" with title \"fnm Error\""
```

### Python Hook Example

```python
#!/usr/bin/env python3
# ~/.fnm/hooks/post-install

import os
import subprocess
import json

version = os.environ.get('FNM_VERSION')
install_dir = os.environ.get('FNM_INSTALLATION_DIR')

print(f"🐍 Python post-install hook for Node.js {version}")

# Example: Update IDE settings
ide_config = {
    "nodejs": {
        "version": version,
        "path": install_dir
    }
}

with open(os.path.expanduser("~/.vscode/settings.json"), "w") as f:
    json.dump(ide_config, f, indent=2)

print("Updated IDE configuration")
```

### Node.js Hook Example

```javascript
#!/usr/bin/env node
// ~/.fnm/hooks/pre-install

const version = process.env.FNM_VERSION;
const arch = process.env.FNM_ARCH;

console.log(`📦 Installing Node.js ${version} (${arch})`);

// Example: Check system requirements
const fs = require('fs');
const os = require('os');

if (os.freemem() < 1024 * 1024 * 1024) { // Less than 1GB free
    console.warn('⚠️  Warning: Low memory detected');
}

// Example: Log installation attempt
const logEntry = {
    timestamp: new Date().toISOString(),
    version,
    arch,
    platform: os.platform()
};

fs.appendFileSync(
    require('path').join(os.homedir(), '.fnm', 'install-log.json'),
    JSON.stringify(logEntry) + '\n'
);
```

## Common Use Cases

### Automatic Package Manager Setup

```bash
#!/bin/bash
# Auto-install preferred package managers after Node.js installation

npm install -g pnpm@latest yarn@latest
corepack enable
```

### Development Environment Setup

```bash
#!/bin/bash
# Set up development tools and configurations

# Install global development tools
npm install -g typescript eslint prettier nodemon

# Copy development configurations
cp ~/.dotfiles/tsconfig.json ./
cp ~/.dotfiles/.eslintrc.js ./
```

### Project Dependency Management

```bash
#!/bin/bash
# Auto-install project dependencies if package.json exists

if [ -f "./package.json" ]; then
    echo "📦 Installing project dependencies..."
    
    # Use the best available package manager
    if [ -f "./pnpm-lock.yaml" ]; then
        pnpm install
    elif [ -f "./yarn.lock" ]; then
        yarn install
    else
        npm install
    fi
fi
```

### System Integration

```bash
#!/bin/bash
# Update system PATH and configurations

# Update shell configuration
echo "export NODE_VERSION=$FNM_VERSION" >> ~/.bashrc

# Update system service configurations
sudo systemctl reload nginx 2>/dev/null || true
```

## Troubleshooting

### Hook Not Executing

1. **Check executable permissions**: `ls -la ~/.fnm/hooks/`
2. **Verify shebang line**: Ensure scripts start with proper shebang (e.g., `#!/bin/bash`)
3. **Test manually**: Run the hook script directly to check for errors
4. **Check fnm logs**: Look for hook-related error messages

### Hook Execution Fails

1. **Check return codes**: Ensure hooks exit with code 0 on success
2. **Handle missing dependencies**: Check that required tools are installed
3. **Use absolute paths**: Avoid relying on PATH environment in hooks
4. **Add error handling**: Use `set -e` in bash scripts for better error handling

### Debugging Hooks

```bash
# Add debugging to your hooks
#!/bin/bash
set -x  # Enable debug output
set -e  # Exit on error

echo "Hook started with PID: $$"
echo "Environment variables:"
env | grep FNM_
echo "Working directory: $(pwd)"

# Your hook logic here...

echo "Hook completed successfully"
```

## Future Enhancements

The hooks system is designed to be extensible. Future versions may include:

- Additional hook types (pre-use, post-use, pre-uninstall, post-uninstall)
- Hook configuration files
- Hook templates and generators
- Async hook execution
- Hook dependency management
