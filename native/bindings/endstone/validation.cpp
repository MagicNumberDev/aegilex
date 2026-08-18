#include "validation.h"
#include "../../aegilex_types.h"

#include <endstone/server.h>

#include <algorithm>
#include <cstdint>
#include <string_view>

namespace {

constexpr std::size_t kMaxPluginIdBytes = 64;

[[nodiscard]] bool valid_plugin_id(const std::string_view plugin_id) noexcept
{
    if (plugin_id.empty() || plugin_id.size() > kMaxPluginIdBytes) {
        return false;
    }

    return std::all_of(plugin_id.begin(), plugin_id.end(), [](const char value) {
        return (value >= 'a' && value <= 'z') || (value >= '0' && value <= '9') || value == '_';
    });
}

} // namespace

namespace aegilex::native::endstone_binding {

aegilex::status validate_context(HostContext *context, const std::string_view plugin_id,
                                 HostContext **out_context) noexcept
{
    if (context == nullptr || !context->accepting_calls) {
        return aegilex::kDenied;
    }
    if (out_context == nullptr || context->logger == nullptr || context->server.native() == nullptr ||
        !valid_plugin_id(plugin_id)) {
        return aegilex::kInvalidArgument;
    }
    if (!context->server.native()->isPrimaryThread()) {
        return aegilex::kWrongThread;
    }

    *out_context = context;
    return aegilex::kOk;
}

} // namespace aegilex::native::endstone_binding
