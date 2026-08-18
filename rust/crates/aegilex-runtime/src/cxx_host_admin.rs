#[allow(clippy::too_many_arguments)]
#[cxx::bridge(namespace = "aegilex::native::admin")]
pub(crate) mod ffi {
    // Ban entry value data. Ban entries are snapshots, not objects: the guest
    // reads per-property getters over a single bridge fetch.
    struct PlayerBanEntry {
        name: String,
        has_uuid: bool,
        uuid: [u8; 16],
        has_xuid: bool,
        xuid: String,
        has_reason: bool,
        reason: String,
        source: String,
        created: i64,
        has_expires: bool,
        expires: i64,
    }

    // Permission definition child: name -> bool value pair.
    struct PermissionChild {
        name: String,
        value: bool,
    }

    // Attachment permission lookup result.
    struct PermissionValue {
        has: bool,
        value: bool,
    }

    unsafe extern "C++" {
        include!("bindings/endstone/ban/ban_list.h");
        include!("bindings/endstone/permissions/permission.h");
        include!("bindings/endstone/permissions/permission_attachment.h");
        include!("bindings/endstone/permissions/permission_definition.h");
        include!("bindings/endstone/server.h");
        include!("bindings/endstone/permissions/permissible.h");

        type BanList;
        type PermissionAttachment;
        type PermissionDefinition;
        type PermissionDefinitionCollection;
        #[namespace = "aegilex::native::server"]
        type Server = crate::cxx_host_server::ffi::Server;
        #[namespace = "aegilex::native::host"]
        type Permissible = crate::cxx_host_common::ffi::Permissible;

        fn getPlayerBanList(self: &Server) -> UniquePtr<BanList>;
        fn getIpBanList(self: &Server) -> UniquePtr<BanList>;
        fn isBanned(self: &BanList, target: &str) -> bool;
        fn isBannedByIdentity(
            self: &BanList,
            target: &str,
            has_uuid: bool,
            uuid: &[u8],
            has_xuid: bool,
            xuid: &str,
        ) -> bool;
        fn getBanEntry(self: &BanList, target: &str, out: &mut PlayerBanEntry) -> bool;
        fn addBan(
            self: &BanList,
            target: &str,
            has_reason: bool,
            reason: &str,
            has_expires: bool,
            expires: i64,
            has_source: bool,
            source: &str,
            out: &mut PlayerBanEntry,
        ) -> bool;
        fn addBanByIdentity(
            self: &BanList,
            target: &str,
            has_uuid: bool,
            uuid: &[u8],
            has_xuid: bool,
            xuid: &str,
            has_reason: bool,
            reason: &str,
            has_expires: bool,
            expires: i64,
            has_source: bool,
            source: &str,
            out: &mut PlayerBanEntry,
        ) -> bool;
        fn removeBan(self: &BanList, target: &str);
        fn removeBanByIdentity(
            self: &BanList,
            target: &str,
            has_uuid: bool,
            uuid: &[u8],
            has_xuid: bool,
            xuid: &str,
        );
        fn getTargets(self: &BanList) -> Vec<String>;

        fn attach(
            self: &Permissible,
            server: &Server,
            name: &str,
            value: bool,
        ) -> UniquePtr<PermissionAttachment>;
        fn attachEmpty(self: &Permissible, server: &Server) -> UniquePtr<PermissionAttachment>;
        fn setPermission(self: &PermissionAttachment, permission: &str, value: bool);
        fn unsetPermission(self: &PermissionAttachment, permission: &str);
        fn getPermissions(self: &PermissionAttachment) -> Vec<String>;
        fn getPermissionValue(self: &PermissionAttachment, permission: &str) -> PermissionValue;
        fn remove(self: &PermissionAttachment) -> bool;
        fn isSame(self: &PermissionAttachment, other: &PermissionAttachment) -> bool;
        fn getEffectiveAttachment(
            self: &Permissible,
            permission: &str,
        ) -> UniquePtr<PermissionAttachment>;

        // Permission definitions mirror endstone/permissions/permission.h.
        fn getPermissionDefinition(self: &Server, name: &str) -> UniquePtr<PermissionDefinition>;
        fn addPermissionDefinition(
            self: &Server,
            name: &str,
            has_description: bool,
            description: &str,
            has_default: bool,
            default_value: u8,
            children: &Vec<PermissionChild>,
        ) -> UniquePtr<PermissionDefinition>;
        fn removePermissionDefinitionByName(self: &Server, name: &str) -> bool;
        fn listDefaultPermissionDefinitions(
            self: &Server,
            level: u8,
        ) -> UniquePtr<PermissionDefinitionCollection>;
        fn len(self: &PermissionDefinitionCollection) -> usize;
        fn takePermissionDefinition(
            self: Pin<&mut PermissionDefinitionCollection>,
            index: usize,
        ) -> UniquePtr<PermissionDefinition>;
        fn getName(self: &PermissionDefinition) -> String;
        fn getDescription(self: &PermissionDefinition) -> String;
        fn setDescription(self: &PermissionDefinition, description: &str);
        fn getDefault(self: &PermissionDefinition) -> u8;
        fn setDefault(self: &PermissionDefinition, default_value: u8);
        fn getChildren(self: &PermissionDefinition) -> Vec<PermissionChild>;
        fn addChild(self: &PermissionDefinition, name: &str, value: bool);
        fn removeChild(self: &PermissionDefinition, name: &str);
        fn recalculatePermissibles(self: &PermissionDefinition);
        fn recalculatePermissionDefaults(self: &Server, definition: &PermissionDefinition);
        fn addParentByName(
            self: &PermissionDefinition,
            name: &str,
            value: bool,
        ) -> UniquePtr<PermissionDefinition>;

        fn dispatchConsoleCommand(self: &Server, command_line: &str) -> bool;
    }
}
