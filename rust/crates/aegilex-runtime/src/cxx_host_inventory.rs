#[allow(clippy::too_many_arguments)]
#[cxx::bridge(namespace = "aegilex::native::inventory")]
pub(crate) mod ffi {
    struct Enchantment {
        type_id: String,
        level: i32,
    }

    struct ItemMetaBase {
        has_display_name: bool,
        display_name: String,
        has_lore: bool,
        lore: Vec<String>,
        enchants: Vec<Enchantment>,
        unbreakable: bool,
        has_damage: bool,
        damage: i32,
        has_repair_cost: bool,
        repair_cost: i32,
    }

    // Flattened endstone::ItemMeta variant (kind: 0=Item, 1=Book, 2=CrossBow,
    // 3=Map, 4=WritableBook). Optional fields use has_* companion flags.
    struct ItemMeta {
        map_id: i64,
        pages: Vec<String>,
        title: String,
        author: String,
        base: ItemMetaBase,
        kind: u8,
        has_title: bool,
        has_author: bool,
        has_generation: bool,
        generation: u8,
        has_map_id: bool,
        has_map_view: bool,
    }

    struct NbtEntry {
        key: String,
        value_index: u32,
    }

    struct NbtNode {
        kind: u8,
        byte_value: u8,
        short_value: i16,
        int_value: i32,
        long_value: i64,
        float_value: f32,
        double_value: f64,
        string_value: String,
        byte_array: Vec<u8>,
        int_array: Vec<i32>,
        child_indices: Vec<u32>,
        entries: Vec<NbtEntry>,
    }

    struct Nbt {
        root_index: u32,
        nodes: Vec<NbtNode>,
    }

    unsafe extern "C++" {
        include!("bindings/endstone/inventory/item_stack.h");
        include!("bindings/endstone/inventory/item_type.h");
        include!("bindings/endstone/inventory/inventory.h");
        include!("bindings/endstone/inventory/player_inventory.h");
        include!("bindings/endstone/inventory/item_meta.h");
        include!("bindings/endstone/server.h");

        type ItemStack;
        type ItemStackRef;
        type ItemStackCollection;
        type ItemType;
        type Inventory;
        type PlayerInventory;
        #[namespace = "aegilex::native::server"]
        type Server = crate::cxx_host_server::ffi::Server;
        fn getItemType(self: &Server, type_id: &str) -> UniquePtr<ItemType>;
        fn createItemMetaForType(
            self: &Server,
            type_id: &str,
            out: &mut ItemMeta,
            projectiles: Pin<&mut ItemStackCollection>,
        ) -> bool;
        fn isItemMetaApplicable(
            self: &Server,
            type_id: &str,
            meta: &ItemMeta,
            projectiles: &ItemStackCollection,
            out: &mut bool,
        ) -> bool;
        fn areItemMetasEqual(
            self: &Server,
            has_a: bool,
            a: &ItemMeta,
            a_projectiles: &ItemStackCollection,
            has_b: bool,
            b: &ItemMeta,
            b_projectiles: &ItemStackCollection,
            out: &mut bool,
        ) -> bool;
        fn convertItemMetaForType(
            self: &Server,
            type_id: &str,
            meta: &ItemMeta,
            projectiles: &ItemStackCollection,
            out: &mut ItemMeta,
            converted_projectiles: Pin<&mut ItemStackCollection>,
        ) -> bool;

        // Raw-pointer factories are used only when native code introduces an
        // Endstone object into Rust's guest-handle table.

        // ItemStack facade — mirrors endstone/inventory/item_stack.h.
        #[allow(dead_code)]
        fn getType(self: &ItemStack) -> String;
        fn setType(self: &ItemStack, type_id: &str) -> bool;
        #[allow(dead_code)]
        fn getAmount(self: &ItemStack) -> i32;
        #[allow(dead_code)]
        fn setAmount(self: &ItemStack, amount: i32);
        #[allow(dead_code)]
        fn getData(self: &ItemStack) -> i32;
        #[allow(dead_code)]
        fn setData(self: &ItemStack, data: i32);
        fn getTranslationKey(self: &ItemStack) -> String;
        fn getMaxStackSize(self: &ItemStack) -> i32;
        fn cloneItemStack(self: &ItemStack) -> UniquePtr<ItemStack>;
        fn hasItemMeta(self: &ItemStack) -> bool;
        fn equals(self: &ItemStack, other: &ItemStack) -> bool;
        fn isSimilar(self: &ItemStack, other: &ItemStack) -> bool;
        fn getMeta(self: &ItemStack, out: &mut ItemMeta) -> bool;
        fn getChargedProjectiles(self: &ItemStack) -> UniquePtr<ItemStackCollection>;
        fn setMeta(
            self: &ItemStack,
            server: &Server,
            meta: &ItemMeta,
            projectiles: &ItemStackCollection,
            out: &mut bool,
        ) -> bool;
        fn getNbt(self: &ItemStack, out: &mut Nbt) -> bool;
        fn setNbt(self: &ItemStack, value: &Nbt) -> bool;

        // ItemStackRef facade — read-only callback-scoped ItemStack view.
        fn getType(self: &ItemStackRef) -> String;
        fn getAmount(self: &ItemStackRef) -> i32;
        fn getData(self: &ItemStackRef) -> i32;
        fn getTranslationKey(self: &ItemStackRef) -> String;
        fn getMaxStackSize(self: &ItemStackRef) -> i32;
        fn cloneItemStack(self: &ItemStackRef) -> UniquePtr<ItemStack>;
        fn hasItemMeta(self: &ItemStackRef) -> bool;
        fn isSimilar(self: &ItemStackRef, other: &ItemStack) -> bool;
        fn getMeta(self: &ItemStackRef, out: &mut ItemMeta) -> bool;
        fn getChargedProjectiles(self: &ItemStackRef) -> UniquePtr<ItemStackCollection>;
        fn getNbt(self: &ItemStackRef, out: &mut Nbt) -> bool;
        #[allow(dead_code)]
        fn borrow_item_stack(item: &ItemStack) -> UniquePtr<ItemStackRef>;

        // ItemType facade — mirrors endstone/inventory/item_type.h.
        fn getTypeId(self: &ItemType) -> String;
        fn getTranslationKey(self: &ItemType) -> String;
        fn getMaxStackSize(self: &ItemType) -> i32;
        fn getMaxDurability(self: &ItemType) -> i32;
        fn createItemStack(self: &ItemType, amount: i32) -> UniquePtr<ItemStack>;

        // Inventory facade — all item crossings use typed ItemStack wrappers.
        fn getSize(self: &Inventory) -> i32;
        fn getMaxStackSize(self: &Inventory) -> i32;
        fn getItem(self: &Inventory, index: i32) -> UniquePtr<ItemStack>;
        fn setItem(self: &Inventory, index: i32, item: &ItemStack);
        fn clear(self: &Inventory);
        fn clearIndex(self: &Inventory, index: i32);
        fn addItem(self: &Inventory, item: &ItemStack) -> UniquePtr<ItemStack>;
        fn removeItem(self: &Inventory, item: &ItemStack) -> UniquePtr<ItemStack>;
        fn containsType(self: &Inventory, type_id: &str) -> bool;
        fn containsStack(self: &Inventory, item: &ItemStack) -> bool;
        fn containsAtLeastType(self: &Inventory, type_id: &str, amount: i32) -> bool;
        fn containsAtLeastStack(self: &Inventory, item: &ItemStack, amount: i32) -> bool;
        fn firstEmpty(self: &Inventory) -> i32;
        fn isEmpty(self: &Inventory) -> bool;
        fn removeType(self: &Inventory, type_id: &str);
        fn removeStack(self: &Inventory, item: &ItemStack);
        // PlayerInventory facade — mirrors endstone/inventory/player_inventory.h.
        fn asInventory(self: &PlayerInventory) -> UniquePtr<Inventory>;
        fn getHeldItemSlot(self: &PlayerInventory) -> i32;
        fn setHeldItemSlot(self: &PlayerInventory, slot: i32);
        fn getHelmet(self: &PlayerInventory) -> UniquePtr<ItemStack>;
        fn setHelmet(self: &PlayerInventory, item: &ItemStack);
        fn clearHelmet(self: &PlayerInventory);
        fn getChestplate(self: &PlayerInventory) -> UniquePtr<ItemStack>;
        fn setChestplate(self: &PlayerInventory, item: &ItemStack);
        fn clearChestplate(self: &PlayerInventory);
        fn getLeggings(self: &PlayerInventory) -> UniquePtr<ItemStack>;
        fn setLeggings(self: &PlayerInventory, item: &ItemStack);
        fn clearLeggings(self: &PlayerInventory);
        fn getBoots(self: &PlayerInventory) -> UniquePtr<ItemStack>;
        fn setBoots(self: &PlayerInventory, item: &ItemStack);
        fn clearBoots(self: &PlayerInventory);
        fn getItemInMainHand(self: &PlayerInventory) -> UniquePtr<ItemStack>;
        fn setItemInMainHand(self: &PlayerInventory, item: &ItemStack);
        fn clearItemInMainHand(self: &PlayerInventory);
        fn getItemInOffHand(self: &PlayerInventory) -> UniquePtr<ItemStack>;
        fn setItemInOffHand(self: &PlayerInventory, item: &ItemStack);
        fn clearItemInOffHand(self: &PlayerInventory);
        // The collection keeps crossbow projectiles as typed facade copies;
        // Rust maps them to guest-visible ids only at the WIT boundary.
        #[Self = "ItemStackCollection"]
        #[rust_name = "create_item_stack_collection"]
        fn create() -> UniquePtr<ItemStackCollection>;
        fn push(self: Pin<&mut ItemStackCollection>, item: &ItemStack);
        fn len(self: &ItemStackCollection) -> usize;
        fn get(self: &ItemStackCollection, index: usize) -> UniquePtr<ItemStack>;

    }

    #[namespace = "aegilex::native::player"]
    unsafe extern "C++" {
        include!("bindings/endstone/actor/player.h");

        type Player = crate::cxx_host_player::ffi::Player;

        fn getInventory(self: &Player) -> UniquePtr<PlayerInventory>;
        fn getEnderChest(self: &Player) -> UniquePtr<Inventory>;
    }
}
