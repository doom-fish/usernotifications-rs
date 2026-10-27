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
            name: "UserNotificationsABI",
            path: "Sources/UserNotificationsABI",
            publicHeadersPath: "include"
        ),
        .target(
            name: "UserNotificationsBridge",
            dependencies: ["UserNotificationsABI"],
            path: "Sources/UserNotificationsBridge",
            publicHeadersPath: "include"
        ),
        .testTarget(
            name: "UserNotificationsBridgeTests",
            dependencies: ["UserNotificationsBridge"],
            path: "Tests/UserNotificationsBridgeTests"
        )
    ]
)
