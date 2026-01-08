use zed_extension_api as zed;

// macOS file extensions for iOS/Xcode
const IOS_EXTENSIONS: &[&str] = &[
    "swift", "m", "mm", "h", "xib", "storyboard", "plist", "xcconfig",
    "entitlements", "pbxproj",
];

// macOS file extensions for Android/Android Studio
const ANDROID_EXTENSIONS: &[&str] = &[
    "kt", "kts", "java", "xml", "gradle", "groovy", "properties",
    "aidl", "pro",
];

struct JumpToNativeIde;

impl zed::Extension for JumpToNativeIde {
    fn new() -> Self {
        JumpToNativeIde
    }

    /// Run slash command - this is how Zed extensions handle custom commands
    fn run_slash_command(
        &self,
        command: zed::SlashCommand,
        args: Vec<String>,
        _worktree: Option<&zed::Worktree>,
    ) -> Result<zed::SlashCommandOutput, String> {
        let command_name = command.name.as_str();

        match command_name {
            "jump_to_xcode" => {
                let file_path = args.first().ok_or("No file path provided")?;
                let line = args
                    .get(1)
                    .and_then(|l| l.parse::<u32>().ok())
                    .unwrap_or(1);
                self.jump_to_xcode(file_path, line)?;
                Ok(zed::SlashCommandOutput {
                    text: "Opened in Xcode".to_string(),
                    sections: vec![],
                })
            }
            "jump_to_android_studio" => {
                let file_path = args.first().ok_or("No file path provided")?;
                let line = args
                    .get(1)
                    .and_then(|l| l.parse::<u32>().ok())
                    .unwrap_or(1);
                self.jump_to_android_studio(file_path, line)?;
                Ok(zed::SlashCommandOutput {
                    text: "Opened in Android Studio".to_string(),
                    sections: vec![],
                })
            }
            "auto_jump" => {
                let file_path = args.first().ok_or("No file path provided")?;
                let line = args
                    .get(1)
                    .and_then(|l| l.parse::<u32>().ok())
                    .unwrap_or(1);
                self.auto_jump(file_path, line)?;
                Ok(zed::SlashCommandOutput {
                    text: "Opened in native IDE".to_string(),
                    sections: vec![],
                })
            }
            _ => Err(format!("Unknown command: {}", command_name)),
        }
    }

    /// Provide completions for slash command arguments
    fn complete_slash_command_argument(
        &self,
        _command: zed::SlashCommand,
        _args: Vec<String>,
    ) -> Result<Vec<zed::SlashCommandArgumentCompletion>, String> {
        Ok(vec![])
    }
}

impl JumpToNativeIde {
    /// Get file extension from path
    fn get_extension(&self, file_path: &str) -> Option<String> {
        file_path
            .rsplit('.')
            .next()
            .map(|ext| ext.to_lowercase())
    }

    /// Determine if file is an iOS file
    fn is_ios_file(&self, file_path: &str) -> bool {
        if let Some(ext) = self.get_extension(file_path) {
            IOS_EXTENSIONS.contains(&ext.as_str())
        } else {
            false
        }
    }

    /// Determine if file is an Android file
    fn is_android_file(&self, file_path: &str) -> bool {
        if let Some(ext) = self.get_extension(file_path) {
            ANDROID_EXTENSIONS.contains(&ext.as_str())
        } else {
            false
        }
    }

    /// Auto jump to the appropriate IDE based on file type
    fn auto_jump(&self, file_path: &str, line: u32) -> Result<(), String> {
        if self.is_ios_file(file_path) {
            self.jump_to_xcode(file_path, line)?;
        } else if self.is_android_file(file_path) {
            self.jump_to_android_studio(file_path, line)?;
        } else {
            return Err(format!(
                "Unsupported file type: {}. Supported: {} (iOS), {} (Android)",
                self.get_extension(file_path).unwrap_or_default(),
                IOS_EXTENSIONS.join(", "),
                ANDROID_EXTENSIONS.join(", ")
            ));
        }
        Ok(())
    }

    /// Jump to Xcode at the specified file and line
    fn jump_to_xcode(&self, file_path: &str, line: u32) -> Result<(), String> {
        let mut cmd = zed::process::Command::new("xed")
            .arg("-l")
            .arg(line.to_string())
            .arg(file_path);

        let _output = cmd.output().map_err(|e| format!("Failed to execute xed: {}", e))?;
        Ok(())
    }

    /// Jump to Android Studio at the specified file and line
    fn jump_to_android_studio(&self, file_path: &str, line: u32) -> Result<(), String> {
        let mut cmd = zed::process::Command::new("studio")
            .arg("--line")
            .arg(line.to_string())
            .arg(file_path);

        let _output = cmd.output().map_err(|e| format!("Failed to execute studio: {}", e))?;
        Ok(())
    }
}

zed::register_extension!(JumpToNativeIde);
