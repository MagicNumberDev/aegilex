cmake_minimum_required(VERSION 3.29)

project(aegilex VERSION 0.1.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

option(AEGILEX_BUILD_PLUGIN "Build the Endstone plugin" ON)
set(ENDSTONE_API_VERSION "0.11" CACHE STRING "Endstone API version")
set(AEGILEX_RUNTIME_LIBRARY "" CACHE FILEPATH "Path to the Rust aegilex-runtime static library")
set(AEGILEX_RUNTIME_CXX_INCLUDE "" CACHE PATH "Path to the Rust cxxbridge include directory")

# This target is deliberately excluded from the default build. Cargo links it only
# for unit tests; the Endstone plugin never depends on it.
add_library(aegilex_test_stub STATIC EXCLUDE_FROM_ALL
    test_stub/test_stub_host.cpp
    test_stub/test_stub_actor.cpp
    test_stub/test_stub_item_actor.cpp
    test_stub/test_stub_player.cpp
    test_stub/test_stub_inventory.cpp
    test_stub/test_stub_level.cpp
    test_stub/test_stub_server.cpp
    test_stub/test_stub_ui.cpp
    test_stub/test_stub_admin.cpp
    test_stub/test_stub_common.cpp
    test_stub/test_stub_logger.cpp
    test_stub/test_stub_event.cpp
)
target_include_directories(aegilex_test_stub PRIVATE "${CMAKE_CURRENT_SOURCE_DIR}")
if (AEGILEX_RUNTIME_CXX_INCLUDE)
    target_include_directories(aegilex_test_stub PRIVATE "${AEGILEX_RUNTIME_CXX_INCLUDE}")
endif ()
set_target_properties(aegilex_test_stub PROPERTIES ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib")
foreach (config Debug Release RelWithDebInfo MinSizeRel)
    set_target_properties(aegilex_test_stub PROPERTIES ARCHIVE_OUTPUT_DIRECTORY_${config} "${CMAKE_BINARY_DIR}/lib")
endforeach ()

if (AEGILEX_BUILD_PLUGIN)
    if (WIN32 AND NOT (CMAKE_CXX_COMPILER_ID MATCHES "Clang" AND CMAKE_CXX_COMPILER_FRONTEND_VARIANT STREQUAL "MSVC"))
        message(FATAL_ERROR "Aegilex requires clang-cl on Windows. Run cargo xtask build after installing LLVM clang-cl.")
    endif ()

    if (NOT AEGILEX_RUNTIME_LIBRARY)
        message(FATAL_ERROR "AEGILEX_RUNTIME_LIBRARY must point to the Cargo-built Rust static library.")
    endif ()

    if (NOT EXISTS "${AEGILEX_RUNTIME_LIBRARY}")
        message(FATAL_ERROR "Rust static library does not exist: ${AEGILEX_RUNTIME_LIBRARY}")
    endif ()

    if (NOT AEGILEX_RUNTIME_CXX_INCLUDE)
        message(FATAL_ERROR "AEGILEX_RUNTIME_CXX_INCLUDE must point to Cargo's cxxbridge include directory.")
    endif ()

    if (NOT EXISTS "${AEGILEX_RUNTIME_CXX_INCLUDE}")
        message(FATAL_ERROR "Rust cxxbridge include directory does not exist: ${AEGILEX_RUNTIME_CXX_INCLUDE}")
    endif ()

    include(FetchContent)
    set(FETCHCONTENT_UPDATES_DISCONNECTED ON)
    FetchContent_Declare(
        endstone
        GIT_REPOSITORY https://github.com/EndstoneMC/endstone.git
        GIT_TAG v${ENDSTONE_API_VERSION}
    )
    FetchContent_MakeAvailable(endstone)

    string(TOUPPER ${PROJECT_NAME} PROJECT_NAME_UPPER)
    configure_file(version.h.in generated/version.h @ONLY)

    endstone_add_plugin(aegilex
        plugin.cpp
        runtime_bridge.cpp
        host_bridge.cpp
        wasm_plugin_loader.cpp
        bindings/endstone/actor/actor.cpp
        bindings/endstone/actor/item_actor.cpp
        bindings/endstone/actor/mob.cpp
        bindings/endstone/actor/player.cpp
        bindings/endstone/ban/ban_list.cpp
        bindings/endstone/boss/boss_bar.cpp
        bindings/endstone/command_sender.cpp
        bindings/endstone/inventory.cpp
        bindings/endstone/logger.cpp
        bindings/endstone/level/level.cpp
        bindings/endstone/level/dimension.cpp
        bindings/endstone/level/chunk.cpp
        bindings/endstone/level/block.cpp
        bindings/endstone/map/map.cpp
        bindings/endstone/permissions/permissible.cpp
        bindings/endstone/permissions/permission.cpp
        bindings/endstone/permissions/permission_definition.cpp
        bindings/endstone/permissions/permission_attachment.cpp
        bindings/endstone/plugin.cpp
        bindings/endstone/scoreboard/scoreboard.cpp
        bindings/endstone/scoreboard/objective.cpp
        bindings/endstone/server.cpp
        bindings/endstone/validation.cpp
        bindings/endstone/events/actor_damage_event_facade.cpp
        bindings/endstone/events/actor_death_event_facade.cpp
        bindings/endstone/events/block_explode_event_facade.cpp
        bindings/endstone/events/actor_explode_event_facade.cpp
        bindings/endstone/events/actor_knockback_event_facade.cpp
        bindings/endstone/events/actor_remove_event_facade.cpp
        bindings/endstone/events/plugin_lifecycle_event_facade.cpp
        bindings/endstone/events/server_load_event_facade.cpp
        bindings/endstone/events/chunk_event_facade.cpp
        bindings/endstone/events/actor_spawn_event_facade.cpp
        bindings/endstone/events/actor_teleport_event_facade.cpp
        bindings/endstone/events/broadcast_message_event_facade.cpp
        bindings/endstone/events/packet_send_event_facade.cpp
        bindings/endstone/events/packet_receive_event_facade.cpp
        bindings/endstone/events/map_initialize_event_facade.cpp
        bindings/endstone/events/script_message_event_facade.cpp
        bindings/endstone/events/player_chat_event_facade.cpp
        bindings/endstone/events/player_command_event_facade.cpp
        bindings/endstone/events/player_kick_event_facade.cpp
        bindings/endstone/events/player_login_event_facade.cpp
        bindings/endstone/events/player_game_mode_change_event_facade.cpp
        bindings/endstone/events/player_emote_event_facade.cpp
        bindings/endstone/events/player_skin_change_event_facade.cpp
        bindings/endstone/events/player_death_event_facade.cpp
        bindings/endstone/events/player_join_event_facade.cpp
        bindings/endstone/events/player_quit_event_facade.cpp
        bindings/endstone/events/player_dimension_change_event_facade.cpp
        bindings/endstone/events/player_bed_enter_event_facade.cpp
        bindings/endstone/events/player_bed_leave_event_facade.cpp
        bindings/endstone/events/player_respawn_event_facade.cpp
        bindings/endstone/events/player_item_held_event_facade.cpp
        bindings/endstone/events/player_move_event_facade.cpp
        bindings/endstone/events/player_drop_item_event_facade.cpp
        bindings/endstone/events/block_break_event_facade.cpp
        bindings/endstone/events/block_cook_event_facade.cpp
        bindings/endstone/events/block_from_to_event_facade.cpp
        bindings/endstone/events/block_grow_event_facade.cpp
        bindings/endstone/events/block_piston_event_facade.cpp
        bindings/endstone/events/block_place_event_facade.cpp
        bindings/endstone/events/leaves_decay_event_facade.cpp
        bindings/endstone/events/player_interact_event_facade.cpp
        bindings/endstone/events/player_interact_actor_event_facade.cpp
        bindings/endstone/events/player_item_consume_event_facade.cpp
        bindings/endstone/events/player_pickup_item_event_facade.cpp
        bindings/endstone/events/server_command_event_facade.cpp
        bindings/endstone/events/server_list_ping_event_facade.cpp
        bindings/endstone/events/weather_change_event_facade.cpp
        bindings/endstone/events/thunder_change_event_facade.cpp
        bridge/command_bridge.cpp
        bridge/event_bridge.cpp
        bridge/task_bridge.cpp
        bridge/form_bridge.cpp
        bridge/map_renderer_bridge.cpp
    )

    target_include_directories(aegilex PRIVATE
        "${CMAKE_CURRENT_SOURCE_DIR}"
        "${CMAKE_CURRENT_SOURCE_DIR}/../include"
        "${AEGILEX_RUNTIME_CXX_INCLUDE}"
        "${CMAKE_CURRENT_BINARY_DIR}/generated"
    )
    target_link_libraries(aegilex PRIVATE "${AEGILEX_RUNTIME_LIBRARY}")

    # rustc's staticlib requires these Windows system libraries when linked by CMake.
    if (WIN32)
        target_link_libraries(aegilex PRIVATE advapi32 bcrypt ntdll ole32 shell32 userenv uuid ws2_32)
    elseif (UNIX)
        target_link_libraries(aegilex PRIVATE dl pthread m)
    endif ()
endif ()
