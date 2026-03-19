// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "LingCodeIME",
    platforms: [.macOS(.v13)],
    products: [
        .executable(
            name: "LingCodeIME",
            targets: ["LingCodeIME"]
        ),
    ],
    dependencies: [],
    targets: [
        .executableTarget(
            name: "LingCodeIME",
            dependencies: [],
            swiftSettings: [
                .unsafeFlags(["-I/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/InputMethodKit.framework/Headers"])
            ],
            linkerSettings: [
                .unsafeFlags([
                    "-L../../../../../../target/debug",
                    "-llingcode_ffi",
                    "-framework", "InputMethodKit",
                    "-framework", "Carbon"
                ])
            ]
        ),
    ]
)