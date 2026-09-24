// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "UserNotificationsBridge",
    platforms: [
        .macOS(.v12)
    ],
    products: [
        .library(
            name: "UserNotificationsBridge",
            type: .static,
            targets: ["UserNotificationsBridge"]
        )
    ],
    targets: [
        .target(
            name: "UserNotificationsBridge",
            path: "Sources/UserNotificationsBridge",
            publicHeadersPath: "include"
        )
    ]
)
