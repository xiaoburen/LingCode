import Foundation
import InputMethodKit
import Carbon

// MARK: - FFI Bridge

/// FFI 函数声明
@_silgen_name("lingcode_engine_new")
func lingcode_engine_new() -> UnsafeMutableRawPointer?

@_silgen_name("lingcode_engine_with_dicts")
func lingcode_engine_with_dicts(_ dictDir: UnsafePointer<CChar>) -> UnsafeMutableRawPointer?

@_silgen_name("lingcode_engine_free")
func lingcode_engine_free(_ engine: UnsafeMutableRawPointer?)

@_silgen_name("lingcode_process_key")
func lingcode_process_key(_ engine: UnsafeMutableRawPointer?, _ key: CChar) -> Int32

@_silgen_name("lingcode_get_buffer")
func lingcode_get_buffer(_ engine: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>?

@_silgen_name("lingcode_get_candidate_count")
func lingcode_get_candidate_count(_ engine: UnsafeMutableRawPointer?) -> Int32

@_silgen_name("lingcode_get_candidate")
func lingcode_get_candidate(_ engine: UnsafeMutableRawPointer?, _ index: Int32) -> UnsafeMutablePointer<CChar>?

@_silgen_name("lingcode_select_candidate")
func lingcode_select_candidate(_ engine: UnsafeMutableRawPointer?, _ index: Int32) -> UnsafeMutablePointer<CChar>?

@_silgen_name("lingcode_backspace")
func lingcode_backspace(_ engine: UnsafeMutableRawPointer?) -> Int32

@_silgen_name("lingcode_clear")
func lingcode_clear(_ engine: UnsafeMutableRawPointer?)

@_silgen_name("lingcode_string_free")
func lingcode_string_free(_ s: UnsafeMutablePointer<CChar>?)

// MARK: - 字符串扩展

extension String {
    init?(cString: UnsafeMutablePointer<CChar>?) {
        guard let cString = cString else { return nil }
        self.init(cString: cString)
    }
}

// MARK: - 输入法入口点

let kConnectionName = "LingCodeInputMethod_Connection"

@main
class LingCodeIMEApp {
    static func main() {
        NSLog("[LingCode] 输入法启动")
        
        guard let server = IMKServer(
            name: kConnectionName,
            bundleIdentifier: "com.lingcode.inputmethod"
        ) else {
            NSLog("[LingCode] 无法创建输入法服务器")
            return
        }
        
        NSLog("[LingCode] 输入法服务器运行中...")
        
        RunLoop.main.run()
    }
}

// MARK: - 输入法控制器

@objc(LingCodeInputController)
class LingCodeInputController: IMKInputController {
    
    // MARK: - 状态
    private var engine: UnsafeMutableRawPointer?
    private var candidatesWindow: CandidatesWindowController?
    private var selectedIndex: Int = 0
    
    // MARK: - 生命周期
    
    override init!(server: IMKServer!, delegate: Any!, client inputClient: Any!) {
        super.init(server: server, delegate: delegate, client: inputClient)
        
        // 初始化 Rust 引擎
        self.engine = lingcode_engine_new()
        if self.engine == nil {
            NSLog("[LingCode] 警告: 无法创建 Rust 引擎，使用模拟数据")
        } else {
            NSLog("[LingCode] Rust 引擎初始化成功")
        }
        
        self.candidatesWindow = CandidatesWindowController()
        NSLog("[LingCode] 输入控制器初始化")
    }
    
    override func activateServer(_ sender: Any!) {
        selectedIndex = 0
        NSLog("[LingCode] 输入法激活")
    }
    
    override func deactivateServer(_ sender: Any!) {
        hideCandidatesWindow()
        NSLog("[LingCode] 输入法停用")
    }
    
    deinit {
        if let engine = engine {
            lingcode_engine_free(engine)
        }
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
        guard let engine = engine else {
            // 回退到模拟数据
            return handleLetterInputMock(letter)
        }
        
        let key = CChar(letter.utf8.first!)
        let result = lingcode_process_key(engine, key)
        
        if result == 1 {
            updateCandidates()
            return true
        }
        return false
    }
    
    private func handleBackspaceKey() -> Bool {
        guard let engine = engine else {
            return handleBackspaceKeyMock()
        }
        
        let result = lingcode_backspace(engine)
        
        if result == 1 {
            updateCandidates()
            return true
        } else {
            hideCandidatesWindow()
            clearMarkedText()
            return true
        }
    }
    
    private func handleReturnKey() -> Bool {
        guard let engine = engine else {
            return handleReturnKeyMock()
        }
        
        guard let bufferPtr = lingcode_get_buffer(engine) else { return false }
        let buffer = String(cString: bufferPtr) ?? ""
        lingcode_string_free(bufferPtr)
        
        if buffer.isEmpty { return false }
        commitText(buffer)
        lingcode_clear(engine)
        updateCandidates()
        return true
    }
    
    private func handleSpaceKey() -> Bool {
        guard let engine = engine else {
            return handleSpaceKeyMock()
        }
        
        return selectCandidate(at: 0)
    }
    
    private func handleEscapeKey() -> Bool {
        guard let engine = engine else {
            return handleEscapeKeyMock()
        }
        
        lingcode_clear(engine)
        hideCandidatesWindow()
        clearMarkedText()
        return true
    }
    
    // MARK: - 候选词管理
    
    private func updateCandidates() {
        guard let engine = engine else {
            updateCandidatesMock()
            return
        }
        
        // 获取输入缓冲区
        guard let bufferPtr = lingcode_get_buffer(engine) else {
            hideCandidatesWindow()
            return
        }
        let buffer = String(cString: bufferPtr) ?? ""
        lingcode_string_free(bufferPtr)
        
        // 更新标记文本
        updateMarkedText(buffer)
        
        // 获取候选词数量
        let count = lingcode_get_candidate_count(engine)
        
        if count > 0 {
            // 获取候选词列表
            var candidates: [String] = []
            for i in 0..<count {
                if let candidatePtr = lingcode_get_candidate(engine, Int32(i)) {
                    if let candidate = String(cString: candidatePtr) {
                        candidates.append(candidate)
                    }
                    lingcode_string_free(candidatePtr)
                }
            }
            
            selectedIndex = 0
            showCandidatesWindow(candidates: candidates)
            
            NSLog("[LingCode] 输入: \(buffer), 候选词: \(candidates)")
        } else {
            hideCandidatesWindow()
        }
    }
    
    private func selectCandidate(at index: Int) -> Bool {
        guard let engine = engine else {
            return selectCandidateMock(at: index)
        }
        
        guard let committedPtr = lingcode_select_candidate(engine, Int32(index)) else {
            return false
        }
        
        let committed = String(cString: committedPtr) ?? ""
        lingcode_string_free(committedPtr)
        
        if !committed.isEmpty {
            commitText(committed)
            hideCandidatesWindow()
            clearMarkedText()
        }
        
        return true
    }
    
    private func moveSelection(up: Bool) -> Bool {
        guard let engine = engine else {
            return moveSelectionMock(up: up)
        }
        
        let count = lingcode_get_candidate_count(engine)
        guard count > 0 else { return false }
        
        if up {
            selectedIndex = (selectedIndex - 1 + Int(count)) % Int(count)
        } else {
            selectedIndex = (selectedIndex + 1) % Int(count)
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
    
    private func updateMarkedText(_ text: String = "") {
        guard let client = self.client() else { return }
        let attrString = NSAttributedString(string: text)
        client.setMarkedText(attrString, selectionRange: NSRange(location: text.count, length: 0), replacementRange: NSRange(location: NSNotFound, length: NSNotFound))
    }
    
    private func clearMarkedText() {
        guard let client = self.client() else { return }
        client.setMarkedText("", selectionRange: NSRange(location: 0, length: 0), replacementRange: NSRange(location: NSNotFound, length: NSNotFound))
    }
    
    // MARK: - 候选词窗口
    
    private func showCandidatesWindow(candidates: [String]) {
        guard let client = self.client() else { return }
        
        var cursorRect = NSRect()
        let _ = client.attributes(forCharacterIndex: 0, lineHeightRectangle: &cursorRect)
        
        let screenPoint = cursorRect.origin
        
        candidatesWindow?.show(at: screenPoint, candidates: candidates, selectedIndex: selectedIndex)
    }
    
    private func hideCandidatesWindow() {
        candidatesWindow?.hide()
    }
    
    // MARK: - Mock 实现（备用）
    
    private var mockInputBuffer: String = ""
    private var mockCandidates: [String] = []
    
    private func handleLetterInputMock(_ letter: String) -> Bool {
        mockInputBuffer.append(letter)
        updateCandidatesMock()
        return true
    }
    
    private func handleBackspaceKeyMock() -> Bool {
        if mockInputBuffer.isEmpty { return false }
        mockInputBuffer.removeLast()
        
        if mockInputBuffer.isEmpty {
            hideCandidatesWindow()
            clearMarkedText()
        } else {
            updateCandidatesMock()
        }
        return true
    }
    
    private func handleReturnKeyMock() -> Bool {
        if mockInputBuffer.isEmpty { return false }
        commitText(mockInputBuffer)
        resetMockInput()
        return true
    }
    
    private func handleSpaceKeyMock() -> Bool {
        if mockCandidates.isEmpty { return false }
        commitText(mockCandidates[selectedIndex])
        resetMockInput()
        return true
    }
    
    private func handleEscapeKeyMock() -> Bool {
        if mockInputBuffer.isEmpty { return false }
        resetMockInput()
        return true
    }
    
    private func updateCandidatesMock() {
        mockCandidates = mockCandidatesData(for: mockInputBuffer)
        selectedIndex = 0
        
        updateMarkedText(mockInputBuffer)
        
        if !mockCandidates.isEmpty {
            showCandidatesWindow(candidates: mockCandidates)
        } else {
            hideCandidatesWindow()
        }
        
        NSLog("[LingCode] 输入: \(mockInputBuffer), 候选词: \(mockCandidates)")
    }
    
    private func selectCandidateMock(at index: Int) -> Bool {
        guard index < mockCandidates.count else { return false }
        commitText(mockCandidates[index])
        resetMockInput()
        return true
    }
    
    private func moveSelectionMock(up: Bool) -> Bool {
        guard !mockCandidates.isEmpty else { return false }
        
        if up {
            selectedIndex = (selectedIndex - 1 + mockCandidates.count) % mockCandidates.count
        } else {
            selectedIndex = (selectedIndex + 1) % mockCandidates.count
        }
        
        candidatesWindow?.updateSelection(selectedIndex)
        return true
    }
    
    private func resetMockInput() {
        mockInputBuffer = ""
        mockCandidates = []
        selectedIndex = 0
        hideCandidatesWindow()
        clearMarkedText()
    }
    
    private func mockCandidatesData(for pinyin: String) -> [String] {
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
        
        if let exact = mockData[pinyin] {
            return exact
        }
        
        for (key, values) in mockData {
            if key.hasPrefix(pinyin) && key != pinyin {
                return values
            }
        }
        
        return []
    }
}