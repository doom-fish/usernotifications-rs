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

@inline(__always)
func un_string(_ value: String) -> UnsafeMutablePointer<CChar>? {
    value.withCString { strdup($0) }
}

@inline(__always)
func un_write_error(
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ message: String
) {
    errorOut?.pointee = un_string(message)
}

func un_json_safe(_ value: Any) -> Any {
    switch value {
    case let dict as [String: Any]:
        return dict.mapValues(un_json_safe)
    case let dict as NSDictionary:
        var object: [String: Any] = [:]
        for (key, value) in dict {
            object[String(describing: key)] = un_json_safe(value)
        }
        return object
    case let array as [Any]:
        return array.map(un_json_safe)
    case let array as NSArray:
        return array.map(un_json_safe)
    case let date as Date:
        return date.timeIntervalSince1970
    case let number as NSNumber:
        return number
    case let string as String:
        return string
    case _ as NSNull:
        return NSNull()
    default:
        return String(describing: value)
    }
}

func un_json_string(_ value: Any) -> String {
    let safe = un_json_safe(value)

    func encode(_ object: Any) -> String? {
        guard JSONSerialization.isValidJSONObject(object) else {
            return nil
        }
        do {
            let data = try JSONSerialization.data(withJSONObject: object, options: [.sortedKeys])
            return String(data: data, encoding: .utf8)
        } catch {
            return nil
        }
    }

    if let encoded = encode(safe) {
        return encoded
    }
    if let encodedScalar = encode([safe]) {
        return String(encodedScalar.dropFirst().dropLast())
    }
    return "null"
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
            throw DecodingError.dataCorruptedError(in: container, debugDescription: "unsupported JSON value")
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

func un_decode_json<T: Decodable>(_ cString: UnsafePointer<CChar>?, as type: T.Type) throws -> T {
    guard let cString else {
        throw NSError(domain: "usernotifications-rs", code: Int(UNR_INVALID_ARGUMENT), userInfo: [
            NSLocalizedDescriptionKey: "missing JSON payload",
        ])
    }

    let data = Data(String(cString: cString).utf8)
    return try JSONDecoder().decode(T.self, from: data)
}
