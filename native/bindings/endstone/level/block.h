#pragma once

#include "rust/cxx.h"

#include <cstddef>
#include <cstdint>
#include <memory>
#include <string>

namespace endstone {
class Block;
class Server;
} // namespace endstone

namespace aegilex::native::level {

struct Location;
struct BlockData;
struct BlockSnapshot;

// OOP/Pimpl facade over endstone::Block. Each facade owns an independent
// Endstone clone for the guest invocation.
class Block {
  public:
    explicit Block(std::unique_ptr<endstone::Block> block, endstone::Server *server = nullptr) noexcept;
    explicit Block(const endstone::Block &block, endstone::Server *server = nullptr) noexcept;
    ~Block() noexcept;

    Block(const Block &) = delete;
    Block &operator=(const Block &) = delete;

    rust::String getType() const;
    void setType(rust::Str type_id, bool apply_physics) const;
    std::int32_t getX() const;
    std::int32_t getY() const;
    std::int32_t getZ() const;
    Location getLocation() const;
    BlockData getData() const;
    BlockSnapshot captureState() const;
    [[nodiscard]] std::unique_ptr<Block> getRelative(std::int32_t dx, std::int32_t dy, std::int32_t dz) const;
    [[nodiscard]] std::unique_ptr<Block> clone() const;
    [[nodiscard]] std::uint32_t setData(const BlockData &data, bool apply_physics) const;
    [[nodiscard]] endstone::Block *native() const noexcept;

  private:
    class impl;
    std::shared_ptr<impl> impl;
};

} // namespace aegilex::native::level
