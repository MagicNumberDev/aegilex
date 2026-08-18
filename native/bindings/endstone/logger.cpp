#include "logger.h"

#include "aegilex_types.h"

#include <endstone/logger.h>

#include <memory>
#include <string>

namespace {

[[nodiscard]] endstone::Logger::Level to_endstone_level(const std::uint32_t level) noexcept
{
    switch (level) {
    case aegilex::kLogTrace:
        return endstone::Logger::Trace;
    case aegilex::kLogDebug:
        return endstone::Logger::Debug;
    case aegilex::kLogInfo:
        return endstone::Logger::Info;
    case aegilex::kLogWarning:
        return endstone::Logger::Warning;
    case aegilex::kLogError:
        return endstone::Logger::Error;
    case aegilex::kLogCritical:
        return endstone::Logger::Critical;
    case aegilex::kLogOff:
    default:
        return endstone::Logger::Off;
    }
}

// Endstone exposes the enabled threshold but not a direct level getter.
[[nodiscard]] std::uint32_t current_logger_level(const endstone::Logger &logger) noexcept
{
    struct Probe {
        endstone::Logger::Level endstone_level;
        std::uint32_t aegilex_level;
    };
    static constexpr Probe kProbes[] = {
        {endstone::Logger::Trace, aegilex::kLogTrace}, {endstone::Logger::Debug, aegilex::kLogDebug},
        {endstone::Logger::Info, aegilex::kLogInfo},   {endstone::Logger::Warning, aegilex::kLogWarning},
        {endstone::Logger::Error, aegilex::kLogError}, {endstone::Logger::Critical, aegilex::kLogCritical},
    };
    for (const auto &probe : kProbes) {
        if (logger.isEnabledFor(probe.endstone_level)) {
            return probe.aegilex_level;
        }
    }
    return aegilex::kLogOff;
}

} // namespace

namespace aegilex::native {

class Logger::impl {
  public:
    explicit impl(endstone::Logger *logger) noexcept : logger(logger)
    {
    }

    endstone::Logger *logger;
};

Logger::Logger(endstone::Logger *logger) noexcept : impl_(std::make_shared<class Logger::impl>(logger))
{
}

std::uint32_t Logger::log(const std::uint32_t level, const rust::Str message) const noexcept
{
    try {
        if (impl_ == nullptr || impl_->logger == nullptr) {
            return aegilex::kNotFound;
        }
        if (level >= aegilex::kLogOff) {
            return aegilex::kInvalidArgument;
        }
        impl_->logger->log(to_endstone_level(level), std::string(message.data(), message.size()));
        return aegilex::kOk;
    }
    catch (...) {
        return aegilex::kHostError;
    }
}

rust::String Logger::getName() const
{
    try {
        return impl_ == nullptr || impl_->logger == nullptr ? rust::String()
                                                            : rust::String(std::string(impl_->logger->getName()));
    }
    catch (...) {
        return rust::String();
    }
}

std::uint32_t Logger::getLevel() const noexcept
{
    try {
        return impl_ == nullptr || impl_->logger == nullptr ? aegilex::kLogOff : current_logger_level(*impl_->logger);
    }
    catch (...) {
        return aegilex::kLogOff;
    }
}

std::uint32_t Logger::setLevel(const std::uint32_t level) const noexcept
{
    try {
        if (impl_ == nullptr || impl_->logger == nullptr) {
            return aegilex::kNotFound;
        }
        if (level > aegilex::kLogOff) {
            return aegilex::kInvalidArgument;
        }
        impl_->logger->setLevel(to_endstone_level(level));
        return aegilex::kOk;
    }
    catch (...) {
        return aegilex::kHostError;
    }
}

bool Logger::isEnabledFor(const std::uint32_t level) const noexcept
{
    try {
        if (impl_ == nullptr || impl_->logger == nullptr || level > aegilex::kLogOff) {
            return false;
        }
        return impl_->logger->isEnabledFor(to_endstone_level(level));
    }
    catch (...) {
        return false;
    }
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
