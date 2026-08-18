#include "command_sender.h"

#include "level/block.h"

#include <endstone/command/block_command_sender.h>
#include <endstone/command/command_sender.h>
#include <endstone/level/dimension.h>
#include <endstone/message.h>
#include <endstone/server.h>

#include <memory>
#include <string>
#include <string_view>
#include <utility>

namespace aegilex::native::host {

class CommandSender::impl {
  public:
    explicit impl(endstone::CommandSender *sender, endstone::Server *server) noexcept : sender(sender), server(server)
    {
    }

    endstone::CommandSender *sender;
    endstone::Server *server;
};

namespace {

[[nodiscard]] endstone::Message translatable_message(const rust::Str text, const rust::Vec<rust::String> &parameters)
{
    std::vector<std::string> native_parameters;
    native_parameters.reserve(parameters.size());
    for (const auto &parameter : parameters) {
        native_parameters.emplace_back(parameter.data(), parameter.size());
    }
    return endstone::Translatable{std::string(text.data(), text.size()), std::move(native_parameters)};
}

} // namespace

CommandSender::CommandSender(endstone::CommandSender *sender, endstone::Server *server) noexcept
    : impl(std::make_shared<class CommandSender::impl>(sender, server))
{
}

endstone::CommandSender *CommandSender::native() const noexcept
{
    return impl == nullptr ? nullptr : impl->sender;
}

rust::String CommandSender::getName() const
{
    try {
        return rust::String(impl->sender->getName());
    }
    catch (...) {
        return rust::String();
    }
}

void CommandSender::sendMessage(const rust::Str message) const
{
    try {
        impl->sender->sendMessage(endstone::Message{std::string(message.data(), message.size())});
    }
    catch (...) {
    }
}

void CommandSender::sendTranslatableMessage(const rust::Str text, rust::Vec<rust::String> parameters) const
{
    try {
        impl->sender->sendMessage(translatable_message(text, parameters));
    }
    catch (...) {
    }
}

void CommandSender::sendErrorMessage(const rust::Str message) const
{
    try {
        impl->sender->sendErrorMessage(endstone::Message{std::string(message.data(), message.size())});
    }
    catch (...) {
    }
}

void CommandSender::sendTranslatableErrorMessage(const rust::Str text, rust::Vec<rust::String> parameters) const
{
    try {
        impl->sender->sendErrorMessage(translatable_message(text, parameters));
    }
    catch (...) {
    }
}

std::unique_ptr<::aegilex::native::level::Block> CommandSender::getBlock() const
{
    try {
        auto *sender = impl == nullptr ? nullptr : impl->sender;
        auto *block_sender = sender == nullptr ? nullptr : sender->asBlock();
        auto block = block_sender == nullptr ? nullptr : block_sender->getBlock();
        return block == nullptr ? std::unique_ptr<::aegilex::native::level::Block>()
                                : std::make_unique<::aegilex::native::level::Block>(std::move(block), impl->server);
    }
    catch (...) {
        return std::unique_ptr<::aegilex::native::level::Block>();
    }
}

} // namespace aegilex::native::host
