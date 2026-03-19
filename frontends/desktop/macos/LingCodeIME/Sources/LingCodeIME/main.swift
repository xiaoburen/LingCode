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
    
    // MARK: - 生命周期
    
    override init!(server: IMKServer!, delegate: Any!, client inputClient: Any!) {
        super.init(server: server, delegate: delegate, client: inputClient)
        NSLog("[LingCode] 输入控制器初始化")
    }
    
    override func activateServer(_ sender: Any!) {
        inputBuffer = ""
        candidates = []
        selectedIndex = 0
        NSLog("[LingCode] 输入法激活")
    }
    
    override func deactivateServer(_ sender: Any!) {
        NSLog("[LingCode] 输入法停用")
    }
    
    // MARK: - 按键处理
    
    override func handle(_ event: NSEvent!, client sender: Any!) -> Bool {
        guard let event = event, event.type == .keyDown else { return false }
        
        let keyCode = event.keyCode
        
        // 处理特殊键
        switch keyCode {
        case kVK_Return:
            return handleReturnKey()
        case kVK_Escape:
            return handleEscapeKey()
        case kVK_Space:
            return handleSpaceKey()
        case kVK_Delete:
            return handleBackspaceKey()
        default:
            break
        }
        
        // 处理数字键选择候选词
        if keyCode >= kVK_ANSI_1 && keyCode <= kVK_ANSI_9 {
            let index = Int(keyCode - kVK_ANSI_1)
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
        updateCandidates()
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
        
        // 显示当前输入（Composing 状态）
        if let client = self.client() as? IMKTextInput {
            client.setMarkedText(inputBuffer, selectionRange: NSRange(location: inputBuffer.count, length: 0), replacementRange: NSRange(location: NSNotFound, length: NSNotFound))
        }
        
        NSLog("[LingCode] 输入: \(inputBuffer), 候选词: \(candidates)")
    }
    
    private func mockCandidates(for pinyin: String) -> [String] {
        let mockData: [String: [String]] = [
            "zhong": ["中", "种", "重", "众", "钟"],
            "wen": ["文", "问", "闻", "稳"],
            "zhongwen": ["中文", "种文", "重文"],
            "ni": ["你", "您", "尼"],
            "hao": ["好", "号", "毫"],
            "nihao": ["你好", "您好", "尼好"],
            "wo": ["我", "握"],
            "shi": ["是", "时", "事", "十"],
            "woshi": ["我是", "我时", "我事"],
        ]
        return mockData[pinyin] ?? []
    }
    
    private func selectCandidate(at index: Int) -> Bool {
        guard index < candidates.count else { return false }
        commitText(candidates[index])
        resetInput()
        return true
    }
    
    // MARK: - 文本提交
    
    private func commitText(_ text: String) {
        guard let client = self.client() as? IMKTextInput else { return }
        client.insertText(text, replacementRange: NSRange(location: NSNotFound, length: NSNotFound))
        NSLog("[LingCode] 提交文本: \(text)")
    }
    
    private func resetInput() {
        inputBuffer = ""
        candidates = []
        selectedIndex = 0
        
        // 清除标记文本
        if let client = self.client() as? IMKTextInput {
            client.setMarkedText("", selectionRange: NSRange(location: 0, length: 0), replacementRange: NSRange(location: NSNotFound, length: NSNotFound))
        }
    }
}