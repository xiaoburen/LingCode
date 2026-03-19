#!/bin/bash
# 灵码输入法 macOS 安装脚本

set -e

echo "📝 安装灵码输入法..."

# 构建项目
echo "🔨 构建项目..."
cd "$(dirname "$0")"
swift build -c release

# 安装路径
INSTALL_DIR="$HOME/Library/Input Methods"
APP_NAME="LingCodeIME.app"
BUNDLE_ID="com.lingcode.inputmethod"

# 创建 .app 包结构
echo "📦 创建应用包..."
APP_PATH="$INSTALL_DIR/$APP_NAME"
mkdir -p "$APP_PATH/Contents/MacOS"
mkdir -p "$APP_PATH/Contents/Resources"

# 复制可执行文件
cp .build/release/LingCodeIME "$APP_PATH/Contents/MacOS/"

# 创建 Info.plist
cat > "$APP_PATH/Contents/Info.plist" << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>en</string>
    <key>CFBundleExecutable</key>
    <string>LingCodeIME</string>
    <key>CFBundleIdentifier</key>
    <string>com.lingcode.inputmethod</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>LingCodeIME</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>0.1.0</string>
    <key>CFBundleVersion</key>
    <string>1</string>
    <key>LSBackgroundOnly</key>
    <true/>
    <key>NSPrincipalClass</key>
    <string>LingCodeInputController</string>
    <key>InputMethodConnectionName</key>
    <string>LingCodeInputMethod_Connection</string>
    <key>InputMethodServerControllerClass</key>
    <string>LingCodeInputController</string>
    <key>TSInputMethodIconFileKey</key>
    <string>icon</string>
</dict>
</plist>
EOF

# 注册输入法
echo "🔧 注册输入法..."
register_input_source() {
    local app_path="$1"
    # 移除旧的输入法（如果存在）
    defaults read com.apple.HIToolbox AppleEnabledInputSources 2>/dev/null | \
        grep -o '"InputSourceKind" = "Non Keyboard Input Method";' -A 3 | \
        grep -B 1 "$BUNDLE_ID" | \
        grep -o '"Bundle ID" = "[^"]*";' | \
        sed 's/.*"\([^"]*\)".*/\1/' | \
        while read -r bundle_id; do
            if [ -n "$bundle_id" ]; then
                echo "移除旧的输入法: $bundle_id"
                defaults delete com.apple.HIToolbox AppleEnabledInputSources -array-add "{Bundle ID = \"$bundle_id\";}" 2>/dev/null || true
            fi
        done
    
    # 注册新的输入法
    /System/Library/CoreServices/Input\ Method\ Switcher.app/Contents/MacOS/Input\ Method\ Switcher --register "$app_path" 2>/dev/null || true
}

register_input_source "$APP_PATH"

echo ""
echo "✅ 安装完成！"
echo ""
echo "⚠️  请手动完成以下步骤："
echo "1. 打开 系统设置 → 键盘 → 输入法"
echo "2. 点击 '+' 添加输入法"
echo "3. 选择 '简体中文' 或 '其他' → '灵码输入法'"
echo "4. 点击 '添加'"
echo ""
echo "📝 或者重启登录后，输入法将自动可用"
echo ""
echo "📍 安装路径: $APP_PATH"