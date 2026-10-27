import Foundation
import UserNotifications
import UserNotificationsABI
import XCTest
@testable import UserNotificationsBridge

final class AsyncRequestOrderTests: XCTestCase {
    func testRequestSubmissionKeepsCallOrder() {
        var calls: [String] = []
        var completions: [(Error?) -> Void] = []
        let add: (UNNotificationRequest, @escaping (Error?) -> Void) -> Void = { request, completion in
            calls.append("add:\(request.identifier):\(request.content.body)")
            completions.append(completion)
        }

        func requestBinary(_ body: String) -> UnsafeMutablePointer<UNBufferRaw> {
            let content = UNNotificationContentPayload(
                title: "", subtitle: "", body: body, badge: nil,
                category_identifier: "", thread_identifier: "", user_info: nil,
                sound: nil, attachments: [], summary_argument: "", summary_argument_count: 1,
                interruption_level: nil, relevance_score: nil, filter_criteria: nil,
                localized_title: nil, localized_subtitle: nil, localized_body: nil,
                localized_summary_argument: nil
            )
            let request = UNNotificationRequestPayload(identifier: "tag", content: content, trigger: nil)
            return un_encode_binary(request)
        }

        let first = requestBinary("first")
        defer { un_release_binary(first) }
        un_add_notification_request(UnsafePointer(first), using: add) { error in
            XCTAssertNil(error)
            calls.append("done:first")
        }
        calls.append("remove:tag")
        let second = requestBinary("second")
        defer { un_release_binary(second) }
        un_add_notification_request(UnsafePointer(second), using: add) { error in
            XCTAssertNil(error)
            calls.append("done:second")
        }

        XCTAssertEqual(calls, ["add:tag:first", "remove:tag", "add:tag:second"])
        completions.forEach { $0(nil) }
        XCTAssertEqual(
            calls,
            ["add:tag:first", "remove:tag", "add:tag:second", "done:first", "done:second"]
        )
    }
}
