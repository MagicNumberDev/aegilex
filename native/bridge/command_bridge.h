#pragma once

#include "../aegilex_types.h"

#include "../runtime_bridge.h"

#include <string>
#include <vector>

namespace endstone {
class CommandSender;
}

namespace aegilex::native {

class HostContext;

// Routes a first-class guest command to the owning Wasm plugin.
class CommandBridge {
  public:
    CommandBridge(HostContext &context, Runtime *runtime);
    ~CommandBridge() = default;

    CommandBridge(const CommandBridge &) = delete;
    CommandBridge &operator=(const CommandBridge &) = delete;

    bool handle_guest_command(endstone::CommandSender &sender, const std::string &plugin_id,
                              const std::string &subcommand, const std::vector<std::string> &args) noexcept;

  private:
    HostContext &context_;
    Runtime *runtime_;
};

} // namespace aegilex::native
