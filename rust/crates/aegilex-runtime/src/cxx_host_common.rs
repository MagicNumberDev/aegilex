#[allow(clippy::too_many_arguments)]
#[cxx::bridge(namespace = "aegilex::native::host")]
pub(crate) mod ffi {
    enum PermissionLevel {
        Default = 0,
        Operator = 1,
        Console = 2,
    }

    struct EffectivePermission {
        has: bool,
        value: bool,
    }

    unsafe extern "C++" {
        include!("bindings/endstone/command_sender.h");
        include!("bindings/endstone/permissions/permissible.h");
        include!("bindings/endstone/server.h");

        type CommandSender;
        type Permissible;
        #[namespace = "aegilex::native::server"]
        type Server = crate::cxx_host_server::ffi::Server;

        fn getName(self: &CommandSender) -> String;
        fn sendMessage(self: &CommandSender, message: &str);
        fn sendTranslatableMessage(self: &CommandSender, text: &str, parameters: Vec<String>);
        fn sendErrorMessage(self: &CommandSender, message: &str);
        fn sendTranslatableErrorMessage(self: &CommandSender, text: &str, parameters: Vec<String>);

        fn getBlock(self: &CommandSender) -> UniquePtr<Block>;

        fn getPermissionLevel(self: &Permissible) -> PermissionLevel;
        fn isPermissionSet(self: &Permissible, name: &str) -> bool;
        fn hasPermission(self: &Permissible, name: &str) -> bool;
        fn getEffectivePermission(self: &Permissible, name: &str) -> EffectivePermission;
        fn listEffectivePermissionNames(self: &Permissible) -> Vec<String>;
        fn recalculatePermissions(self: &Permissible);
        fn permissible_from_command_sender(sender: &CommandSender) -> UniquePtr<Permissible>;
        fn asCommandSender(permissible: &Permissible, server: &Server) -> UniquePtr<CommandSender>;

    }

    #[namespace = "aegilex::native::actor"]
    unsafe extern "C++" {
        include!("bindings/endstone/actor/actor.h");

        type Actor = crate::cxx_host_actor::ffi::Actor;
        type Mob = crate::cxx_host_actor::ffi::Mob;
    }

    #[namespace = "aegilex::native::player"]
    unsafe extern "C++" {
        include!("bindings/endstone/actor/player.h");

        type Player = crate::cxx_host_player::ffi::Player;
    }

    #[namespace = "aegilex::native::level"]
    unsafe extern "C++" {
        include!("bindings/endstone/level/block.h");

        type Block = crate::cxx_host_level::ffi::Block;
    }
}
