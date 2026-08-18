#pragma once

#include "../../aegilex_types.h"

#include "../../host_context.h"

#include <string_view>

namespace aegilex::native::endstone_binding {

[[nodiscard]] aegilex::status validate_context(HostContext *context, std::string_view plugin_id,
                                               HostContext **out_context) noexcept;

} // namespace aegilex::native::endstone_binding
