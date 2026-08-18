//! Core ABI implementation for `native/bindings/endstone/ban/ban_list.h`.

use crate::host::endstone::support::*;

impl crate::core_host::imports::HostBanList for PluginStoreState {
    fn ban_list_is_banned(
        &mut self,
        self_: u32,
        target: String,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "ban-list.ban-list.is-banned")?;
            resolve_ban_list(
                self,
                ban_list_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|list| list.isBanned(&target))
            .map_err(map_core_host_error)
        })())
    }

    fn ban_list_is_player_banned(
        &mut self,
        self_: u32,
        target: String,
        uuid: Option<Vec<u8>>,
        xuid: Option<String>,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "ban-list.ban-list.is-player-banned")?;
            let (has_uuid, uuid) = wit_uuid(uuid)?;
            let (has_xuid, xuid) = xuid.map_or((false, String::new()), |value| (true, value));
            resolve_ban_list(
                self,
                ban_list_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|list| list.isBannedByIdentity(&target, has_uuid, &uuid, has_xuid, &xuid))
            .map_err(map_core_host_error)
        })())
    }

    fn ban_list_add_ban(
        &mut self,
        self_: u32,
        target: String,
        reason: Option<String>,
        expires_ms: Option<i64>,
        source: Option<String>,
    ) -> Result<Result<BanListBanEntry, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "ban-list.ban-list.add-ban")?;
            let (has_reason, reason) = reason.map_or((false, String::new()), |value| (true, value));
            let (has_expires, expires) = expires_ms.map_or((false, 0), |value| (true, value));
            let (has_source, source) = source.map_or((false, String::new()), |value| (true, value));
            let mut entry = empty_ban_entry();
            let found = resolve_ban_list(
                self,
                ban_list_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|list| {
                list.addBan(
                    &target,
                    has_reason,
                    &reason,
                    has_expires,
                    expires,
                    has_source,
                    &source,
                    &mut entry,
                )
            })
            .map_err(map_core_host_error)?;
            found
                .then(|| ban_entry(entry))
                .ok_or(TypesHostError::NotFound)
        })())
    }

    fn ban_list_add_player_ban(
        &mut self,
        self_: u32,
        target: String,
        uuid: Option<Vec<u8>>,
        xuid: Option<String>,
        reason: Option<String>,
        expires_ms: Option<i64>,
        source: Option<String>,
    ) -> Result<Result<BanListBanEntry, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "ban-list.ban-list.add-player-ban")?;
            let (has_uuid, uuid) = wit_uuid(uuid)?;
            let (has_xuid, xuid) = xuid.map_or((false, String::new()), |value| (true, value));
            let (has_reason, reason) = reason.map_or((false, String::new()), |value| (true, value));
            let (has_expires, expires) = expires_ms.map_or((false, 0), |value| (true, value));
            let (has_source, source) = source.map_or((false, String::new()), |value| (true, value));
            let mut entry = empty_ban_entry();
            let found = resolve_ban_list(
                self,
                ban_list_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|list| {
                list.addBanByIdentity(
                    &target,
                    has_uuid,
                    &uuid,
                    has_xuid,
                    &xuid,
                    has_reason,
                    &reason,
                    has_expires,
                    expires,
                    has_source,
                    &source,
                    &mut entry,
                )
            })
            .map_err(map_core_host_error)?;
            found
                .then(|| ban_entry(entry))
                .ok_or(TypesHostError::NotFound)
        })())
    }

    fn ban_list_remove_ban(
        &mut self,
        self_: u32,
        target: String,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "ban-list.ban-list.remove-ban")?;
            resolve_ban_list(
                self,
                ban_list_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|list| list.removeBan(&target))
            .map_err(map_core_host_error)
        })())
    }

    fn ban_list_remove_player_ban(
        &mut self,
        self_: u32,
        target: String,
        uuid: Option<Vec<u8>>,
        xuid: Option<String>,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "ban-list.ban-list.remove-player-ban")?;
            let (has_uuid, uuid) = wit_uuid(uuid)?;
            let (has_xuid, xuid) = xuid.map_or((false, String::new()), |value| (true, value));
            resolve_ban_list(
                self,
                ban_list_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|list| list.removeBanByIdentity(&target, has_uuid, &uuid, has_xuid, &xuid))
            .map_err(map_core_host_error)
        })())
    }

    fn ban_list_list_entries(
        &mut self,
        self_: u32,
    ) -> Result<Result<Vec<BanListBanEntry>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "ban-list.ban-list.list-entries")?;
            let list = resolve_ban_list(
                self,
                ban_list_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map_err(map_core_host_error)?;
            Ok(list
                .getTargets()
                .into_iter()
                .filter_map(|target| {
                    let mut entry = empty_ban_entry();
                    list.getBanEntry(&target, &mut entry)
                        .then(|| ban_entry(entry))
                })
                .collect())
        })())
    }

    fn ban_list_get_entry(
        &mut self,
        self_: u32,
        target: String,
    ) -> Result<Result<Option<BanListBanEntry>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "ban-list.ban-list.get-entry")?;
            let mut entry = empty_ban_entry();
            let found = resolve_ban_list(
                self,
                ban_list_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|list| list.getBanEntry(&target, &mut entry))
            .map_err(map_core_host_error)?;
            Ok(found.then(|| ban_entry(entry)))
        })())
    }

    fn get(
        &mut self,
        list_kind: BanListBanListKind,
    ) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "ban-list.get")?;
            let list = resolve_server(self)
                .map(|server| match list_kind {
                    BanListBanListKind::Player => server.getPlayerBanList(),
                    BanListBanListKind::Ip => server.getIpBanList(),
                })
                .map_err(map_core_host_error)?;
            self.insert_ban_list_resource(list)
                .map_err(map_core_host_error)
        })())
    }
}
