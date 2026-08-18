#include "dimension.h"

#include <aegilex-runtime/src/cxx_host_level.rs.h>

#include <endstone/level/dimension.h>
#include <endstone/level/level.h>

#include <memory>

namespace aegilex::native::level {

class Dimension::impl {
  public:
    explicit impl(endstone::Dimension *dimension) noexcept : dimension(dimension)
    {
    }

    endstone::Dimension *dimension;
};

Dimension::Dimension(endstone::Dimension *dimension) noexcept : impl(std::make_shared<class Dimension::impl>(dimension))
{
}

endstone::Dimension *Dimension::native() const noexcept
{
    return impl == nullptr ? nullptr : impl->dimension;
}

rust::String Dimension::getName() const
{
    try {
        return rust::String(impl->dimension->getName());
    }
    catch (...) {
        return rust::String();
    }
}

std::uint32_t Dimension::getType() const
{
    try {
        return static_cast<std::uint32_t>(impl->dimension->getType());
    }
    catch (...) {
        return 0;
    }
}

rust::String Dimension::getLevelName() const
{
    try {
        return rust::String(impl->dimension->getLevel().getName());
    }
    catch (...) {
        return rust::String();
    }
}

} // namespace aegilex::native::level
