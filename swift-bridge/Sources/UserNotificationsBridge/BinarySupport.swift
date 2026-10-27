import Foundation
import UserNotificationsABI

final class UNArena {
    private var releases: [() -> Void] = []

    func keep<T>(_ value: T) -> UnsafePointer<T> {
        let pointer = UnsafeMutablePointer<T>.allocate(capacity: 1)
        pointer.initialize(to: value)
        releases.append { pointer.deinitialize(count: 1); pointer.deallocate() }
        return UnsafePointer(pointer)
    }

    func array<T>(_ values: [T]) -> UNSliceRaw {
        guard !values.isEmpty else { return UNSliceRaw(data: nil, len: 0) }
        let pointer = UnsafeMutablePointer<T>.allocate(capacity: values.count)
        pointer.initialize(from: values, count: values.count)
        releases.append { pointer.deinitialize(count: values.count); pointer.deallocate() }
        return UNSliceRaw(data: UnsafeRawPointer(pointer), len: values.count)
    }

    deinit {
        for release in releases.reversed() { release() }
    }
}

protocol UNABIValue {
    associatedtype Raw
    func raw(_ arena: UNArena) -> Raw
    static func read(_ raw: Raw) throws -> Self
}

func un_binary_error() -> NSError {
    NSError(
        domain: "usernotifications-rs",
        code: Int(UNR_INVALID_ARGUMENT),
        userInfo: [NSLocalizedDescriptionKey: "invalid binary bridge data"]
    )
}

func un_read_array<T>(_ raw: UNSliceRaw, as type: T.Type) throws -> [T] {
    if raw.len == 0 { return [] }
    guard let data = raw.data,
          raw.len > 0,
          raw.len <= Int.max / MemoryLayout<T>.stride,
          Int(bitPattern: data) % MemoryLayout<T>.alignment == 0
    else { throw un_binary_error() }
    return Array(UnsafeBufferPointer(start: data.assumingMemoryBound(to: T.self), count: raw.len))
}

func un_encode_binary<T: UNABIValue>(_ value: T) -> UnsafeMutablePointer<UNBufferRaw> {
    let arena = UNArena()
    let data = arena.keep(value.raw(arena))
    let buffer = arena.keep(UNBufferRaw(
        data: UnsafeRawPointer(data),
        size: MemoryLayout<T.Raw>.size,
        owner: Unmanaged.passRetained(arena).toOpaque(),
        release: un_binary_buffer_free
    ))
    return UnsafeMutablePointer(mutating: buffer)
}

func un_decode_binary<T: UNABIValue>(
    _ buffer: UnsafePointer<UNBufferRaw>?,
    as type: T.Type
) throws -> T {
    guard let raw = buffer?.pointee,
          raw.size == MemoryLayout<T.Raw>.size,
          let data = raw.data,
          Int(bitPattern: data) % MemoryLayout<T.Raw>.alignment == 0
    else { throw un_binary_error() }
    return try T.read(data.assumingMemoryBound(to: T.Raw.self).pointee)
}

@_cdecl("un_binary_buffer_free")
public func un_binary_buffer_free(_ buffer: UnsafeMutablePointer<UNBufferRaw>?) {
    guard let owner = buffer?.pointee.owner else { return }
    Unmanaged<UNArena>.fromOpaque(owner).release()
}

func un_release_binary(_ buffer: UnsafeMutablePointer<UNBufferRaw>?) {
    guard let buffer else { return }
    buffer.pointee.release?(buffer)
}

final class UNBinaryPayload {
    private var buffer: UnsafeMutablePointer<UNBufferRaw>?

    init(_ buffer: UnsafeMutablePointer<UNBufferRaw>) {
        self.buffer = buffer
    }

    func take() -> UnsafeMutablePointer<UNBufferRaw>? {
        defer { buffer = nil }
        return buffer
    }

    deinit {
        un_release_binary(buffer)
    }
}

extension String: UNABIValue {
    func raw(_ arena: UNArena) -> UNStringRaw {
        let bytes = arena.array(Array(utf8))
        return UNStringRaw(data: bytes.data?.assumingMemoryBound(to: UInt8.self), len: bytes.len)
    }

    static func read(_ raw: UNStringRaw) throws -> String {
        let bytes = try un_read_array(UNSliceRaw(data: raw.data, len: raw.len), as: UInt8.self)
        guard let value = String(bytes: bytes, encoding: .utf8) else { throw un_binary_error() }
        return value
    }
}

extension Array: UNABIValue where Element: UNABIValue {
    func raw(_ arena: UNArena) -> UNSliceRaw {
        arena.array(map { $0.raw(arena) })
    }

    static func read(_ raw: UNSliceRaw) throws -> Self {
        try un_read_array(raw, as: Element.Raw.self).map { try Element.read($0) }
    }
}

enum UNPropertyValue {
    case null
    case bool(Bool)
    case integer(Int64)
    case unsigned(UInt64)
    case real(Double)
    case string(String)
    case data(Data)
    case date(Date)
    case array([UNPropertyValue])
    case dictionary([String: UNPropertyValue])

    static func fromFoundationObject(_ value: Any) -> Self {
        switch value {
        case let number as NSNumber where CFGetTypeID(number) == CFBooleanGetTypeID():
            return .bool(number.boolValue)
        case let number as NSNumber:
            let type = String(cString: number.objCType)
            if type == "f" || type == "d" { return .real(number.doubleValue) }
            if type == "Q" || type == "L" || type == "I" || type == "S" || type == "C" {
                return .unsigned(number.uint64Value)
            }
            return .integer(number.int64Value)
        case let string as String:
            return .string(string)
        case let data as Data:
            return .data(data)
        case let date as Date:
            return .date(date)
        case let dictionary as NSDictionary:
            var result: [String: Self] = [:]
            for (key, value) in dictionary {
                result[String(describing: key)] = fromFoundationObject(value)
            }
            return .dictionary(result)
        case let array as NSArray:
            return .array(array.map(fromFoundationObject))
        case _ as NSNull:
            return .null
        default:
            return .string(String(describing: value))
        }
    }

    var foundationObject: Any {
        switch self {
        case .null: return NSNull()
        case .bool(let value): return NSNumber(value: value)
        case .integer(let value): return NSNumber(value: value)
        case .unsigned(let value): return NSNumber(value: value)
        case .real(let value): return NSNumber(value: value)
        case .string(let value): return value
        case .data(let value): return value
        case .date(let value): return value
        case .array(let value): return value.map(\.foundationObject)
        case .dictionary(let value): return value.mapValues(\.foundationObject)
        }
    }
}

extension Dictionary: UNABIValue where Key == String, Value == UNPropertyValue {
    func raw(_ arena: UNArena) -> UNUserInfoRaw {
        let entries = keys.sorted().map { key in
            UNDictionaryEntryRaw(key: key.raw(arena), value: self[key]!.raw(arena))
        }
        return UNUserInfoRaw(entries: arena.array(entries))
    }

    static func read(_ raw: UNUserInfoRaw) throws -> Self {
        var result: Self = [:]
        for entry in try un_read_array(raw.entries, as: UNDictionaryEntryRaw.self) {
            result[try String.read(entry.key)] = try UNPropertyValue.read(entry.value)
        }
        return result
    }
}

extension UNPropertyValue: UNABIValue {
    func raw(_ arena: UNArena) -> UNValueRaw {
        var raw = UNValueRaw()
        switch self {
        case .null: raw.tag = 0
        case .bool(let value): raw.tag = 1; raw.unsigned_value = value ? 1 : 0
        case .integer(let value): raw.tag = 2; raw.signed_value = value
        case .unsigned(let value): raw.tag = 3; raw.unsigned_value = value
        case .real(let value): raw.tag = 4; raw.real_value = value
        case .string(let value): raw.tag = 5; raw.string = value.raw(arena)
        case .data(let value): raw.tag = 6; raw.values = arena.array(Array(value))
        case .date(let value): raw.tag = 7; raw.real_value = value.timeIntervalSince1970
        case .array(let value): raw.tag = 8; raw.values = value.raw(arena)
        case .dictionary(let value): raw.tag = 9; raw.entries = value.raw(arena).entries
        }
        return raw
    }

    static func read(_ raw: UNValueRaw) throws -> Self {
        switch raw.tag {
        case 0: return .null
        case 1: return .bool(raw.unsigned_value != 0)
        case 2: return .integer(raw.signed_value)
        case 3: return .unsigned(raw.unsigned_value)
        case 4: return .real(raw.real_value)
        case 5: return .string(try String.read(raw.string))
        case 6: return .data(Data(try un_read_array(raw.values, as: UInt8.self)))
        case 7: return .date(Date(timeIntervalSince1970: raw.real_value))
        case 8: return .array(try [Self].read(raw.values))
        case 9: return .dictionary(try [String: Self].read(UNUserInfoRaw(entries: raw.entries)))
        default: throw un_binary_error()
        }
    }
}
