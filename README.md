# Zed2NativeIDE

A Zed extension that allows you to jump from Zed to native IDEs (Xcode or Android Studio) at the current file and line number.

## Features

- **Auto Jump**: Automatically jumps to the appropriate IDE based on file type
  - iOS files (`.swift`, `.m`, `.mm`, `.h`, `.xib`, `.storyboard`, etc.) → Xcode
  - Android files (`.kt`, `.java`, `.xml`, `.gradle`, etc.) → Android Studio
- **Manual Jump**: Direct commands to jump to Xcode or Android Studio
- **Line Precision**: Opens the file at the exact line where your cursor is located

## Installation

1. Clone this repository to your Zed extensions directory:
   ```bash
   # macOS extensions path
   ~/Library/Application Support/Zed/extensions
   ```

2. Build the extension:
   ```bash
   cd Zed2NativeIDE
   cargo build --release
   ```

3. Reload Zed

## Usage

### Keyboard Shortcut

Press `Ctrl+J` to automatically jump to the appropriate IDE based on the current file type.

### Command Palette

Open the command palette (`Cmd+Shift+P` on macOS) and run:
- `Jump to Xcode` - Open current file in Xcode
- `Jump to Android Studio` - Open current file in Android Studio
- `Auto Jump to Native IDE` - Automatically choose based on file type

### Context Menu

Right-click in the editor to access jump commands.

## Requirements

- **macOS**: This extension requires macOS as it uses the `xed` and `studio` command-line tools
- **Xcode**: Must have Xcode installed with command-line tools
- **Android Studio**: Must have Android Studio installed with `studio` command in PATH

## Supported File Types

### iOS/Xcode
- Swift: `.swift`
- Objective-C: `.m`, `.mm`, `.h`
- Interface Builder: `.xib`, `.storyboard`
- Config files: `.plist`, `.xcconfig`, `.entitlements`, `.pbxproj`

### Android/Android Studio
- Kotlin: `.kt`, `.kts`
- Java: `.java`
- XML: `.xml`
- Gradle: `.gradle`, `.groovy`
- Other: `.properties`, `.aidl`, `.pro`

## Development

```bash
# Build
cargo build

# Watch for changes
cargo watch -x build

# Test
cargo test
```

## Credits

Based on the VS Code extension [jump-to-native-ide](../jump-to-native-ide).

## License

MIT
