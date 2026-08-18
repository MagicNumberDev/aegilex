#include "chunk_event_facade.h"

#include <endstone/event/chunk/chunk_event.h>

namespace aegilex::native::endstone_binding::events {

ChunkEventFacade::ChunkEventFacade(const endstone::ChunkEvent &event) noexcept
    : chunk_x_(event.getChunk().getX()), chunk_z_(event.getChunk().getZ()),
      dimension_(event.getChunk().getDimension().getName())
{
}

ChunkEventFacade::ChunkEventFacade(const std::int32_t chunk_x, const std::int32_t chunk_z,
                                   std::string dimension) noexcept
    : chunk_x_(chunk_x), chunk_z_(chunk_z), dimension_(std::move(dimension))
{
}

std::int32_t ChunkEventFacade::getChunkX() const noexcept
{
    return chunk_x_;
}

std::int32_t ChunkEventFacade::getChunkZ() const noexcept
{
    return chunk_z_;
}

rust::String ChunkEventFacade::getDimensionForRust() const noexcept
{
    return rust::String(dimension_);
}

} // namespace aegilex::native::endstone_binding::events
