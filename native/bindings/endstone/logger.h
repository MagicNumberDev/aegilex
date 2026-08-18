#pragma once

#include "rust/cxx.h"

#include <cstdint>
#include <memory>

namespace endstone {
class Logger;
}

namespace aegilex::native {

// OOP/Pimpl facade over endstone::Logger. The impl holds only a non-owning
// logger pointer owned by Endstone or its plugin loader.
class Logger {
  public:
    explicit Logger(endstone::Logger *logger) noexcept;
    ~Logger() noexcept = default;

    Logger(const Logger &) = delete;
    Logger &operator=(const Logger &) = delete;

    [[nodiscard]] std::uint32_t log(std::uint32_t level, rust::Str message) const noexcept;
    [[nodiscard]] rust::String getName() const;
    [[nodiscard]] std::uint32_t getLevel() const noexcept;
    [[nodiscard]] std::uint32_t setLevel(std::uint32_t level) const noexcept;
    [[nodiscard]] bool isEnabledFor(std::uint32_t level) const noexcept;

  private:
    class impl;
    std::shared_ptr<impl> impl_;
};

// Carries factory status separately from the ownership of the typed facade so
// Rust can preserve HostContext validation failures before registering a handle.
class LoggerResult {
  public:
    LoggerResult(std::uint32_t status, std::unique_ptr<Logger> logger) noexcept;
    ~LoggerResult() noexcept = default;

    LoggerResult(const LoggerResult &) = delete;
    LoggerResult &operator=(const LoggerResult &) = delete;

    [[nodiscard]] std::uint32_t getStatus() const noexcept;
    [[nodiscard]] std::unique_ptr<Logger> takeLogger() noexcept;

  private:
    std::uint32_t status_;
    std::unique_ptr<Logger> logger_;
};

} // namespace aegilex::native
