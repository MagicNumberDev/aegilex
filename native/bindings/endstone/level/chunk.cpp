#include "chunk.h"

#include <aegilex-runtime/src/cxx_host_level.rs.h>

#include <endstone/level/chunk.h>
#include <endstone/level/dimension.h>
#include <endstone/level/level.h>

#include <memory>

namespace aegilex::native::level {

class Chunk::impl {
  public:
    explicit impl(endstone::Chunk *chunk) noexcept : chunk(chunk)
    {
    }

    endstone::Chunk *chunk;
};

Chunk::Chunk(endstone::Chunk *chunk) noexcept : impl(std::make_shared<class Chunk::impl>(chunk))
{
}

endstone::Chunk *Chunk::native() const noexcept
{
    return impl == nullptr ? nullptr : impl->chunk;
}

std::int32_t Chunk::getX() const
{
    try {
        return impl->chunk->getX();
    }
    catch (...) {
        return 0;
    }
}

std::int32_t Chunk::getZ() const
{
    try {
        return impl->chunk->getZ();
    }
    catch (...) {
        return 0;
    }
}

rust::String Chunk::getLevelName() const
{
    try {
        return rust::String(impl->chunk->getLevel().getName());
    }
    catch (...) {
        return rust::String();
    }
}

rust::String Chunk::getDimensionName() const
{
    try {
        return rust::String(impl->chunk->getDimension().getName());
    }
    catch (...) {
        return rust::String();
    }
}

} // namespace aegilex::native::level
