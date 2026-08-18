#include "block_explode_event_facade.h"

#include <aegilex-runtime/src/cxx_runtime.rs.h>

#include <endstone/event/block/block_explode_event.h>
#include <endstone/level/dimension.h>

namespace aegilex::native::endstone_binding::events {

BlockExplodeEventFacade::BlockExplodeEventFacade(endstone::BlockExplodeEvent *event) noexcept : event_(event)
{
    if (event_ == nullptr) {
        return;
    }
    try {
        block_ = &event_->getBlock();
        for (const auto &block : event_->getBlockList()) {
            if (block != nullptr) {
                affected_blocks_.push_back(std::make_unique<::aegilex::native::level::Block>(*block));
            }
        }
    }
    catch (...) {
        block_ = nullptr;
        affected_blocks_.clear();
    }
}

std::unique_ptr<::aegilex::native::level::Block> BlockExplodeEventFacade::getBlock() const noexcept
{
    if (block_ == nullptr) {
        return nullptr;
    }
    try {
        return std::make_unique<::aegilex::native::level::Block>(*block_);
    }
    catch (...) {
        return nullptr;
    }
}

std::uint64_t BlockExplodeEventFacade::getBlockCount() const noexcept
{
    return affected_blocks_.size();
}

std::unique_ptr<::aegilex::native::level::Block> BlockExplodeEventFacade::getAffectedBlock(const std::uint64_t index) const noexcept
{
    if (index >= affected_blocks_.size() || affected_blocks_[index] == nullptr) {
        return nullptr;
    }
    return affected_blocks_[index]->clone();
}

bool BlockExplodeEventFacade::isCancelled() const noexcept
{
    if (event_ == nullptr) {
        return false;
    }
    try {
        return event_->isCancelled();
    }
    catch (...) {
        return false;
    }
}

bool BlockExplodeEventFacade::setCancelled(const bool cancelled) noexcept
{
    if (event_ == nullptr) {
        return false;
    }
    try {
        event_->setCancelled(cancelled);
        return true;
    }
    catch (...) {
        return false;
    }
}

} // namespace aegilex::native::endstone_binding::events
