#include "command_bridge.h"
#include "../aegilex_types.h"

#include "../bindings/endstone/command_sender.h"
#include "../host_context.h"

#include <endstone/command/command_sender.h>
#include <endstone/logger.h>
#include <endstone/message.h>
#include <endstone/player.h>
#include <endstone/server.h>

#include <algorithm>
#include <cstdint>
#include <string>
#include <vector>

namespace {

[[nodiscard]] std::uint32_t sender_kind(const endstone::CommandSender &sender) noexcept
{
    if (sender.asPlayer() != nullptr) {
        return aegilex::kSenderPlayer;
    }
    if (sender.asConsole() != nullptr) {
        return aegilex::kSenderConsole;
    }
    if (sender.asBlock() != nullptr) {
        return aegilex::kSenderBlock;
    }
    if (sender.asActor() != nullptr) {
        return aegilex::kSenderActor;
    }
    return aegilex::kSenderConsole;
}

} // namespace

namespace aegilex::native {

CommandBridge::CommandBridge(HostContext &context, Runtime *runtime) : context_(context), runtime_(runtime)
{
}

bool CommandBridge::handle_guest_command(endstone::CommandSender &sender, const std::string &plugin_id,
                                         const std::string &subcommand, const std::vector<std::string> &args) noexcept
{
    try {
        if (runtime_ == nullptr || context_.server.native() == nullptr || !context_.accepting_calls ||
            !context_.server.native()->isPrimaryThread()) {
            return false;
        }

        const auto &plugin_ids = context_.enabled_plugin_ids();
        if (std::find(plugin_ids.begin(), plugin_ids.end(), plugin_id) == plugin_ids.end()) {
            sender.sendErrorMessage(endstone::Message{"Unknown or disabled Aegilex plugin '" + plugin_id + "'."});
            return true;
        }

        aegilex::runtime::CommandData data{};
        data.sender_kind = sender_kind(sender);
        const auto *player = sender.asPlayer();
        data.has_player_id = player != nullptr;
        if (player != nullptr) {
            const auto &id = player->getUniqueId();
            data.player_id = rust::Vec<std::uint8_t>();
            for (auto it = id.begin(); it != id.end(); ++it) {
                data.player_id.push_back(*it);
            }
        }
        data.sender_name = rust::String(sender.getName());
        data.subcommand = rust::String(subcommand);
        data.args = rust::Vec<rust::String>();
        for (const auto &arg : args) {
            data.args.push_back(rust::String(arg));
        }

        const auto invocation_id = context_.next_invocation_id();
        auto sender_facade = std::make_unique<aegilex::native::host::CommandSender>(&sender, context_.server.native());
        const auto outcome = aegilex::runtime::dispatch_command(*runtime_->handle, plugin_id, invocation_id,
                                                                std::move(sender_facade), data);

        if (!outcome.handled) {
            return false;
        }
        if (!outcome.error.empty()) {
            sender.sendErrorMessage(endstone::Message{std::string(outcome.error)});
        }
        else if (!outcome.reply.empty()) {
            sender.sendMessage(endstone::Message{std::string(outcome.reply)});
        }
        return true;
    }
    catch (...) {
        return false;
    }
}

} // namespace aegilex::native
