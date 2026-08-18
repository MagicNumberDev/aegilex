#pragma once

#include <endstone/endstone.hpp>

#include <memory>
#include <string>
#include <vector>

#include "host_context.h"

namespace aegilex::native {
struct Runtime;
class WasmPluginLoader;
} // namespace aegilex::native

class AegilexPlugin : public endstone::Plugin {
  public:
    ~AegilexPlugin() override;

    void onLoad() override;
    void onEnable() override;
    void onDisable() override;
    bool onCommand(endstone::CommandSender &sender, const endstone::Command &command,
                   const std::vector<std::string> &args) override;

  private:
    [[nodiscard]] bool start_runtime();

    std::shared_ptr<aegilex::native::HostContext> host_context_;
    std::unique_ptr<aegilex::native::Runtime> runtime_;
    aegilex::native::WasmPluginLoader *wasm_loader_{};
};
