import Cocoa

/// 候选词窗口控制器
class CandidatesWindowController: NSWindowController {
    
    private var candidatesView: CandidatesView!
    
    init() {
        let window = NSWindow(
            contentRect: NSRect(x: 0, y: 0, width: 300, height: 200),
            styleMask: [.borderless, .nonactivatingPanel],
            backing: .buffered,
            defer: false
        )
        
        super.init(window: window)
        
        window.level = .floating
        window.isOpaque = false
        window.backgroundColor = NSColor.clear
        window.hasShadow = true
        
        setupUI()
    }
    
    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }
    
    private func setupUI() {
        guard let contentView = window?.contentView else { return }
        
        candidatesView = CandidatesView(frame: contentView.bounds)
        candidatesView.autoresizingMask = [.width, .height]
        contentView.addSubview(candidatesView)
    }
    
    func show(at point: NSPoint, candidates: [String], selectedIndex: Int) {
        candidatesView.updateCandidates(candidates, selectedIndex: selectedIndex)
        
        // 调整窗口大小
        let size = candidatesView.intrinsicContentSize
        window?.setContentSize(size)
        
        // 定位窗口（在光标下方）
        var windowPoint = point
        windowPoint.y -= size.height + 5
        window?.setFrameOrigin(windowPoint)
        
        window?.orderFront(nil)
    }
    
    func hide() {
        window?.orderOut(nil)
    }
    
    func updateCandidates(_ candidates: [String], selectedIndex: Int) {
        candidatesView.updateCandidates(candidates, selectedIndex: selectedIndex)
        let size = candidatesView.intrinsicContentSize
        window?.setContentSize(size)
    }
    
    func updateSelection(_ index: Int) {
        candidatesView.updateSelection(index)
    }
}

/// 候选词视图
class CandidatesView: NSView {
    
    private var candidates: [String] = []
    private var selectedIndex: Int = 0
    
    private let itemHeight: CGFloat = 32
    private let padding: CGFloat = 8
    private let maxVisibleItems = 9
    
    override init(frame frameRect: NSRect) {
        super.init(frame: frameRect)
        setupAppearance()
    }
    
    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }
    
    private func setupAppearance() {
        self.wantsLayer = true
        self.layer?.backgroundColor = NSColor.windowBackgroundColor.withAlphaComponent(0.95).cgColor
        self.layer?.cornerRadius = 6
        self.layer?.borderWidth = 0.5
        self.layer?.borderColor = NSColor.separatorColor.cgColor
        self.layer?.shadowColor = NSColor.black.cgColor
        self.layer?.shadowOffset = CGSize(width: 0, height: 2)
        self.layer?.shadowRadius = 4
        self.layer?.shadowOpacity = 0.15
    }
    
    func updateCandidates(_ candidates: [String], selectedIndex: Int) {
        self.candidates = Array(candidates.prefix(maxVisibleItems))
        self.selectedIndex = min(selectedIndex, self.candidates.count - 1)
        self.needsDisplay = true
    }
    
    func updateSelection(_ index: Int) {
        self.selectedIndex = min(index, candidates.count - 1)
        self.needsDisplay = true
    }
    
    override var intrinsicContentSize: NSSize {
        let visibleCount = min(candidates.count, maxVisibleItems)
        let height = max(CGFloat(visibleCount) * itemHeight + padding * 2, 40)
        return NSSize(width: 220, height: height)
    }
    
    override func draw(_ dirtyRect: NSRect) {
        super.draw(dirtyRect)
        
        // 绘制背景
        NSColor.windowBackgroundColor.withAlphaComponent(0.95).setFill()
        bounds.fill()
        
        // 绘制候选词
        for (index, candidate) in candidates.enumerated() {
            let y = bounds.height - padding - CGFloat(index + 1) * itemHeight
            let rect = NSRect(x: padding, y: y, width: bounds.width - padding * 2, height: itemHeight)
            
            // 绘制选中背景
            if index == selectedIndex {
                let selectionRect = NSRect(x: 4, y: y + 2, width: bounds.width - 8, height: itemHeight - 4)
                let path = NSBezierPath(roundedRect: selectionRect, xRadius: 4, yRadius: 4)
                NSColor.selectedContentBackgroundColor.setFill()
                path.fill()
            }
            
            // 绘制序号
            let indexString = "\(index + 1)."
            let indexAttributes: [NSAttributedString.Key: Any] = [
                .font: NSFont.systemFont(ofSize: 11),
                .foregroundColor: NSColor.secondaryLabelColor
            ]
            let indexSize = indexString.size(withAttributes: indexAttributes)
            let indexPoint = NSPoint(x: rect.minX + 4, y: y + (itemHeight - indexSize.height) / 2)
            indexString.draw(at: indexPoint, withAttributes: indexAttributes)
            
            // 绘制候选词
            let textColor = (index == selectedIndex) ? NSColor.selectedTextColor : NSColor.labelColor
            let candidateAttributes: [NSAttributedString.Key: Any] = [
                .font: NSFont.systemFont(ofSize: 18),
                .foregroundColor: textColor
            ]
            let candidateX = rect.minX + indexSize.width + 12
            let candidatePoint = NSPoint(x: candidateX, y: y + (itemHeight - 18) / 2)
            candidate.draw(at: candidatePoint, withAttributes: candidateAttributes)
        }
        
        // 绘制边框
        NSColor.separatorColor.setStroke()
        let borderPath = NSBezierPath(roundedRect: bounds.insetBy(dx: 0.5, dy: 0.5), xRadius: 6, yRadius: 6)
        borderPath.lineWidth = 0.5
        borderPath.stroke()
    }
}