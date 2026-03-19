import Foundation
import InputMethodKit
import Carbon

// MARK: - 输入法入口点

let kConnectionName = "LingCodeInputMethod_Connection"

@main
class LingCodeIMEApp {
    static func main() {
        NSLog("[LingCode] 输入法启动")
        
        // 创建输入法服务器
        guard let server = IMKServer(
            name: kConnectionName,
            bundleIdentifier: "com.lingcode.inputmethod"
        ) else {
            NSLog("[LingCode] 无法创建输入法服务器")
            return
        }
        
        NSLog("[LingCode] 输入法服务器运行中...")
        
        // 运行 RunLoop
        RunLoop.main.run()
    }
}

// MARK: - 输入法控制器

@objc(LingCodeInputController)
class LingCodeInputController: IMKInputController {
    
    // MARK: - 状态
    private var inputBuffer: String = ""
    private var candidates: [String] = []
    private var selectedIndex: Int = 0
    private var candidatesWindow: CandidatesWindowController?
    
    // MARK: - 生命周期
    
    override init!(server: IMKServer!, delegate: Any!, client inputClient: Any!) {
        super.init(server: server, delegate: delegate, client: inputClient)
        candidatesWindow = CandidatesWindowController()
        NSLog("[LingCode] 输入控制器初始化")
    }
    
    override func activateServer(_ sender: Any!) {
        inputBuffer = ""
        candidates = []
        selectedIndex = 0
        NSLog("[LingCode] 输入法激活")
    }
    
    override func deactivateServer(_ sender: Any!) {
        hideCandidatesWindow()
        NSLog("[LingCode] 输入法停用")
    }
    
    // MARK: - 按键处理
    
    override func handle(_ event: NSEvent!, client sender: Any!) -> Bool {
        guard let event = event, event.type == .keyDown else { return false }
        
        let keyCode = event.keyCode
        
        // 处理特殊键
        switch Int(keyCode) {
        case kVK_Return:
            return handleReturnKey()
        case kVK_Escape:
            return handleEscapeKey()
        case kVK_Space:
            return handleSpaceKey()
        case kVK_Delete:
            return handleBackspaceKey()
        case kVK_UpArrow:
            return moveSelection(up: true)
        case kVK_DownArrow:
            return moveSelection(up: false)
        default:
            break
        }
        
        // 处理数字键选择候选词
        if keyCode >= kVK_ANSI_1 && keyCode <= kVK_ANSI_9 {
            let index = Int(keyCode) - Int(kVK_ANSI_1)
            return selectCandidate(at: index)
        }
        
        // 处理字母输入
        if let chars = event.characters,
           chars.count == 1,
           let char = chars.first,
           char.isASCII,
           char.isLetter {
            return handleLetterInput(char.lowercased())
        }
        
        return false
    }
    
    // MARK: - 输入处理
    
    private func handleLetterInput(_ letter: String) -> Bool {
        inputBuffer.append(letter)
        updateCandidates()
        return true
    }
    
    private func handleBackspaceKey() -> Bool {
        if inputBuffer.isEmpty { return false }
        inputBuffer.removeLast()
        
        if inputBuffer.isEmpty {
            hideCandidatesWindow()
            clearMarkedText()
        } else {
            updateCandidates()
        }
        return true
    }
    
    private func handleReturnKey() -> Bool {
        if inputBuffer.isEmpty { return false }
        commitText(inputBuffer)
        resetInput()
        return true
    }
    
    private func handleSpaceKey() -> Bool {
        if candidates.isEmpty { return false }
        commitText(candidates[selectedIndex])
        resetInput()
        return true
    }
    
    private func handleEscapeKey() -> Bool {
        if inputBuffer.isEmpty { return false }
        resetInput()
        return true
    }
    
    // MARK: - 候选词管理
    
    private func updateCandidates() {
        candidates = mockCandidates(for: inputBuffer)
        selectedIndex = 0
        
        // 更新标记文本
        updateMarkedText()
        
        // 显示候选词窗口
        if !candidates.isEmpty {
            showCandidatesWindow()
        } else {
            hideCandidatesWindow()
        }
        
        NSLog("[LingCode] 输入: \(inputBuffer), 候选词: \(candidates)")
    }
    
    private func mockCandidates(for pinyin: String) -> [String] {
        let mockData: [String: [String]] = [
            "zhong": ["中", "种", "重", "众", "钟", "终", "忠", "仲", "肿"],
            "wen": ["文", "问", "闻", "稳", "吻", "纹", "蚊"],
            "zhongwen": ["中文", "种文", "重文"],
            "ni": ["你", "您", "尼", "泥", "拟", "逆", "妮"],
            "hao": ["好", "号", "毫", "豪", "耗", "浩", "郝"],
            "nihao": ["你好", "您好", "尼好"],
            "wo": ["我", "握", "窝", "卧", "沃", "涡"],
            "shi": ["是", "时", "事", "十", "市", "师", "使", "史"],
            "woshi": ["我是", "我时", "我事"],
            "de": ["的", "得", "地", "德"],
            "yi": ["一", "以", "已", "意", "义", "亿", "宜"],
            "ge": ["个", "各", "歌", "格", "哥", "隔"],
            "ren": ["人", "任", "认", "仁", "忍", "刃"],
        ]
        
        // 优先返回完整拼音匹配，然后是前缀匹配
        if let exact = mockData[pinyin] {
            return exact
        }
        
        // 前缀匹配
        for (key, values) in mockData {
            if key.hasPrefix(pinyin) && key != pinyin {
                return values
            }
        }
        
        return []
    }
    
    private func selectCandidate(at index: Int) -> Bool {
        guard index < candidates.count else { return false }
        commitText(candidates[index])
        resetInput()
        return true
    }
    
    private func moveSelection(up: Bool) -> Bool {
        guard !candidates.isEmpty else { return false }
        
        if up {
            selectedIndex = (selectedIndex - 1 + candidates.count) % candidates.count
        } else {
            selectedIndex = (selectedIndex + 1) % candidates.count
        }
        
        candidatesWindow?.updateSelection(selectedIndex)
        return true
    }
    
    // MARK: - 文本操作
    
    private func commitText(_ text: String) {
        guard let client = self.client() else { return }
        client.insertText(text, replacementRange: NSRange(location: NSNotFound, length: NSNotFound))
        NSLog("[LingCode] 提交文本: \(text)")
    }
    
    private func updateMarkedText() {
        guard let client = self.client() else { return }
        let attrString = NSAttributedString(string: inputBuffer)
        client.setMarkedText(attrString, selectionRange: NSRange(location: inputBuffer.count, length: 0), replacementRange: NSRange(location: NSNotFound, length: NSNotFound))
    }
    
    private func clearMarkedText() {
        guard let client = self.client() else { return }
        client.setMarkedText("", selectionRange: NSRange(location: 0, length: 0), replacementRange: NSRange(location: NSNotFound, length: NSNotFound))
    }
    
    private func resetInput() {
        inputBuffer = ""
        candidates = []
        selectedIndex = 0
        hideCandidatesWindow()
        clearMarkedText()
    }
    
    // MARK: - 候选词窗口
    
    private func showCandidatesWindow() {
        guard let client = self.client() else { return }
        
        // 获取光标位置
        var cursorRect = NSRect()
        let _ = client.attributes(forCharacterIndex: 0, lineHeightRectangle: &cursorRect)
        
        // 使用光标位置（已经是屏幕坐标）
        let screenPoint = cursorRect.origin
        
        candidatesWindow?.show(at: screenPoint, candidates: candidates, selectedIndex: selectedIndex)
    }
    
    private func hideCandidatesWindow() {
        candidatesWindow?.hide()
    }
}