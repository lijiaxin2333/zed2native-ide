use std::process::Command;
use zed_extension_api as zed;

// macOS file extensions for iOS/Xcode
const IOS_EXTENSIONS: &[&str] = &[
    ".swift", ".m", ".mm", ".h", ".xib", ".storyboard", ".plist", ".xcconfig",
    ".entitlements", ".pbxproj",
];

// macOS file extensions for Android/Android Studio
const ANDROID_EXTENSIONS: &[&str] = &[
    ".kt", ".kts", ".java", ".xml", ".gradle", ".groovy", ".properties",
    ".aidl", ".pro",
];

pub struct Zed2NativeIDE {
    state: Option<zed::State>,
}

impl zed::Extension for Zed2NativeIDE {
    fn new() -> Self {
        Self { state: None }
    }

    fn activate(&mut self, state: zed::State) {
        self.state = Some(state);
    }

    fn deactivate(&mut self) {
        self.state = None;
    }

    fn run_command(
        &mut self,
        command: &str,
        args: Vec<zed::CommandArg>,
    ) -> Result<(), String> {
        match command {
            "zed2native-ide:jump-to-xcode" => {
                self.jump_to_xcode(args)?;
            }
            "zed2native-ide:jump-to-android-studio" => {
                self.jump_to_android_studio(args)?;
            }
            "zed2native-ide:auto-jump" => {
                self.auto_jump(args)?;
            }
            _ => return Err(format!("Unknown command: {}", command)),
        }
        Ok(())
    }
}

impl Zed2NativeIDE {
    /// Jump to Xcode at the current file and line
    fn jump_to_xcode(&self, _args: Vec<zed::CommandArg>) -> Result<(), String> {
        let state = self.state.as_ref().ok_or("Extension not activated")?;

        // Get current workspace and active buffer
        let workspace = state.workspace().ok_or("No active workspace")?;
        let buffer = workspace
            .active_buffer()
            .ok_or("No active buffer")?;

        // Get file path
        let file_path = buffer
            .file()
            .ok_or("Buffer is not a file")?
            .path()
            .to_string();

        // Get current cursor position
        let cursors = buffer.cursors();
        let cursor = cursors
            .first()
            .ok_or("No cursor position")?;
        let line = cursor.start.row;

        // Execute xed command
        let output = Command::new("xed")
            .arg("-l")
            .arg(line.to_string())
            .arg(&file_path)
            .output()
            .map_err(|e| format!("Failed to execute xed: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("xed failed: {}", stderr));
        }

        Ok(())
    }

    /// Jump to Android Studio at the current file and line
    fn jump_to_android_studio(&self, _args: Vec<zed::CommandArg>) -> Result<(), String> {
        let state = self.state.as_ref().ok_or("Extension not activated")?;

        // Get current workspace and active buffer
        let workspace = state.workspace().ok_or("No active workspace")?;
        let buffer = workspace
            .active_buffer()
            .ok_or("No active buffer")?;

        // Get file path
        let file_path = buffer
            .file()
            .ok_or("Buffer is not a file")?
            .path()
            .to_string();

        // Get current cursor position
        let cursors = buffer.cursors();
        let cursor = cursors
            .first()
            .ok_or("No cursor position")?;
        let line = cursor.start.row;

        // Execute studio command
        let output = Command::new("studio")
            .arg("--line")
            .arg(line.to_string())
            .arg(&file_path)
            .output()
            .map_err(|e| format!("Failed to execute studio: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("studio failed: {}", stderr));
        }

        Ok(())
    }

    /// Auto jump to the appropriate IDE based on file type
    fn auto_jump(&self, args: Vec<zed::CommandArg>) -> Result<(), String> {
        let state = self.state.as_ref().ok_or("Extension not activated")?;

        // Get current workspace and active buffer
        let workspace = state.workspace().ok_or("No active workspace")?;
        let buffer = workspace
            .active_buffer()
            .ok_or("No active buffer")?;

        // Get file path
        let file_path = buffer
            .file()
            .ok_or("Buffer is not a file")?
            .path()
            .to_string();

        // Determine IDE based on file extension
        let extension = file_path
            .rsplit('.')
            .next()
            .map(|ext| format!(".{}", ext.to_lowercase()))
            .unwrap_or_default();

        if IOS_EXTENSIONS.contains(&extension.as_str()) {
            self.jump_to_xcode(args)?;
        } else if ANDROID_EXTENSIONS.contains(&extension.as_str()) {
            self.jump_to_android_studio(args)?;
        } else {
            return Err(format!(
                "Unsupported file type: {}. Supported: {} (iOS), {} (Android)",
                extension,
                IOS_EXTENSIONS.join(", "),
                ANDROID_EXTENSIONS.join(", ")
            ));
        }

        Ok(())
    }
}

zed::register_extension!(Zed2NativeIDE);
