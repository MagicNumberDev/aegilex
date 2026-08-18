#pragma once

#include <cstdint>

// Internal C++ types and status codes for the native host. Replaces the
// legacy C ABI headers (aegilex/types.h, aegilex/abi.h) after the typed
// bridge migration.

namespace aegilex {

using status = std::uint32_t;

inline constexpr std::uint64_t kInvalidHandle = 0ull;

inline constexpr status kOk = 0u;
inline constexpr status kInvalidArgument = 1u;
inline constexpr status kNotFound = 3u;
inline constexpr status kDenied = 4u;
inline constexpr status kWrongThread = 5u;
inline constexpr status kLimitExceeded = 6u;
inline constexpr status kHostError = 8u;
inline constexpr status kInternalError = 9u;

inline constexpr std::uint32_t kSenderConsole = 1u;
inline constexpr std::uint32_t kSenderBlock = 2u;
inline constexpr std::uint32_t kSenderActor = 3u;
inline constexpr std::uint32_t kSenderPlayer = 4u;

inline constexpr std::uint32_t kLogTrace = 0u;
inline constexpr std::uint32_t kLogDebug = 1u;
inline constexpr std::uint32_t kLogInfo = 2u;
inline constexpr std::uint32_t kLogWarning = 3u;
inline constexpr std::uint32_t kLogError = 4u;
inline constexpr std::uint32_t kLogCritical = 5u;
inline constexpr std::uint32_t kLogOff = 6u;

inline constexpr std::uint32_t kPermissionDefault = 0u;
inline constexpr std::uint32_t kPermissionOperator = 1u;
inline constexpr std::uint32_t kPermissionConsole = 2u;
inline constexpr std::uint32_t kPermissionDefaultTrue = 0u;
inline constexpr std::uint32_t kPermissionDefaultFalse = 1u;
inline constexpr std::uint32_t kPermissionDefaultOperator = 2u;
inline constexpr std::uint32_t kPermissionDefaultNotOperator = 3u;
inline constexpr std::uint32_t kPermissionDefaultConsole = 4u;

inline constexpr std::uint32_t kPluginLoadStartup = 0u;

} // namespace aegilex
