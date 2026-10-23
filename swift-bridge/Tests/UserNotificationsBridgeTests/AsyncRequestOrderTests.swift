import Foundation
import UserNotifications
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

        func requestJson(_ body: String) -> String {
            """
            {"identifier":"tag","content":{"title":"","subtitle":"","body":"\(body)","category_identifier":"","thread_identifier":"","attachments":[],"summary_argument":"","summary_argument_count":1},"trigger":null}
            """
        }

        requestJson("first").withCString { request in
            un_add_notification_request(request, using: add) { error in
                XCTAssertNil(error)
                calls.append("done:first")
            }
        }
        calls.append("remove:tag")
        requestJson("second").withCString { request in
            un_add_notification_request(request, using: add) { error in
                XCTAssertNil(error)
                calls.append("done:second")
            }
        }

        XCTAssertEqual(calls, ["add:tag:first", "remove:tag", "add:tag:second"])
        completions.forEach { $0(nil) }
        XCTAssertEqual(
            calls,
            ["add:tag:first", "remove:tag", "add:tag:second", "done:first", "done:second"]
        )
    }
}
