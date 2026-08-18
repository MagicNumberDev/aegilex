// Test-only typed Logger facade stubs. Never linked into the plugin.

#include "bindings/endstone/logger.h"

#include "aegilex_types.h"

#include <memory>

namespace aegilex::native {

class Logger::impl {};

Logger::Logger(endstone::Logger *) noexcept : impl_(std::make_shared<class Logger::impl>())
{
}

std::uint32_t Logger::log(const std::uint32_t level, const rust::Str) const noexcept
{
    return level >= aegilex::kLogOff ? aegilex::kInvalidArgument : aegilex::kOk;
}

rust::String Logger::getName() const
{
    return rust::String("Aegilex");
}

std::uint32_t Logger::getLevel() const noexcept
{
    return aegilex::kLogInfo;
}

std::uint32_t Logger::setLevel(const std::uint32_t level) const noexcept
{
    return level > aegilex::kLogOff ? aegilex::kInvalidArgument : aegilex::kOk;
}

bool Logger::isEnabledFor(const std::uint32_t level) const noexcept
{
    return level <= aegilex::kLogInfo;
}

LoggerResult::LoggerResult(const std::uint32_t status, std::unique_ptr<Logger> logger) noexcept
    : status_(status), logger_(std::move(logger))
{
}

std::uint32_t LoggerResult::getStatus() const noexcept
{
    return status_;
}

std::unique_ptr<Logger> LoggerResult::takeLogger() noexcept
{
    return std::move(logger_);
}

} // namespace aegilex::native
