import Foundation
import UserNotifications

public let UNR_OK: Int32 = 0
public let UNR_INVALID_ARGUMENT: Int32 = -1
public let UNR_FRAMEWORK_ERROR: Int32 = -2

@inline(__always)
public func un_retain<T: AnyObject>(_ object: T) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(object).toOpaque()
}

@inline(__always)
public func un_borrow<T: AnyObject>(_ ptr: UnsafeMutableRawPointer) -> T {
    Unmanaged<T>.fromOpaque(ptr).takeUnretainedValue()
}

@_cdecl("un_object_release")
public func un_object_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    Unmanaged<AnyObject>.fromOpaque(ptr).release()
}

@_cdecl("un_error_domain")
public func un_error_domain() -> UnsafeMutablePointer<CChar>? {
    un_string(UNErrorDomain)
}

@inline(__always)
func un_string(_ value: String) -> UnsafeMutablePointer<CChar>? {
    value.withCString { strdup($0) }
}

func un_error_message(_ error: Error) -> String {
    let nsError = error as NSError
    return "\(nsError.domain):\(nsError.code):\(nsError.localizedDescription)"
}

@inline(__always)
func un_write_error(
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ message: String
) {
    errorOut?.pointee = un_string(message)
}

@inline(__always)
func un_write_error(
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    error: Error
) {
    un_write_error(errorOut, un_error_message(error))
}

func un_encode_json<T: Encodable>(_ value: T) -> String {
    let encoder = JSONEncoder()
    encoder.outputFormatting = [.sortedKeys]
    guard let data = try? encoder.encode(value),
          let string = String(data: data, encoding: .utf8)
    else {
        return "null"
    }
    return string
}

enum UNJSONValue: Codable {
    case string(String)
    case number(Double)
    case bool(Bool)
    case object([String: UNJSONValue])
    case array([UNJSONValue])
    case null

    init(from decoder: Decoder) throws {
        let container = try decoder.singleValueContainer()
        if container.decodeNil() {
            self = .null
        } else if let value = try? container.decode(Bool.self) {
            self = .bool(value)
        } else if let value = try? container.decode(Double.self) {
            self = .number(value)
        } else if let value = try? container.decode(String.self) {
            self = .string(value)
        } else if let value = try? container.decode([String: UNJSONValue].self) {
            self = .object(value)
        } else if let value = try? container.decode([UNJSONValue].self) {
            self = .array(value)
        } else {
            throw DecodingError.dataCorruptedError(
                in: container,
                debugDescription: "unsupported JSON value"
            )
        }
    }

    func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()
        switch self {
        case .string(let value):
            try container.encode(value)
        case .number(let value):
            try container.encode(value)
        case .bool(let value):
            try container.encode(value)
        case .object(let value):
            try container.encode(value)
        case .array(let value):
            try container.encode(value)
        case .null:
            try container.encodeNil()
        }
    }

    static func fromFoundationObject(_ value: Any) -> UNJSONValue {
        switch value {
        case let string as String:
            return .string(string)
        case let number as NSNumber where CFGetTypeID(number) == CFBooleanGetTypeID():
            return .bool(number.boolValue)
        case let number as NSNumber:
            return .number(number.doubleValue)
        case let dict as [String: Any]:
            return .object(dict.mapValues(Self.fromFoundationObject))
        case let dict as NSDictionary:
            var object: [String: UNJSONValue] = [:]
            for (key, value) in dict {
                object[String(describing: key)] = Self.fromFoundationObject(value)
            }
            return .object(object)
        case let array as [Any]:
            return .array(array.map(Self.fromFoundationObject))
        case let array as NSArray:
            return .array(array.map(Self.fromFoundationObject))
        case let date as Date:
            return .number(date.timeIntervalSince1970)
        case _ as NSNull:
            return .null
        default:
            return .string(String(describing: value))
        }
    }

    var foundationObject: Any {
        switch self {
        case .string(let value):
            return value
        case .number(let value):
            return value
        case .bool(let value):
            return value
        case .object(let value):
            return value.mapValues(\.foundationObject)
        case .array(let value):
            return value.map(\.foundationObject)
        case .null:
            return NSNull()
        }
    }
}

struct UNLocalizedStringPayload: Codable {
    var key: String
    var arguments: [UNJSONValue]
}

func un_make_localized_string(_ localized: UNLocalizedStringPayload?, fallback: String) -> String {
    guard let localized else {
        return fallback
    }
    return NSString.localizedUserNotificationString(
        forKey: localized.key,
        arguments: localized.arguments.map(\.foundationObject)
    ) as String
}

func un_decode_json<T: Decodable>(_ cString: UnsafePointer<CChar>?, as type: T.Type) throws -> T {
    guard let cString else {
        throw NSError(domain: "usernotifications-rs", code: Int(UNR_INVALID_ARGUMENT), userInfo: [
            NSLocalizedDescriptionKey: "missing JSON payload",
        ])
    }

    let data = Data(String(cString: cString).utf8)
    return try JSONDecoder().decode(T.self, from: data)
}
