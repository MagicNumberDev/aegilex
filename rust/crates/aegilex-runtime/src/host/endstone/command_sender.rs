//! Core ABI implementation for `native/bindings/endstone/command_sender.h`.

use crate::host::endstone::support::*;

impl crate::core_host::imports::HostCommandSender for PluginStoreState {
    fn command_sender_get_name(
        &mut self,
        self_: u32,
    ) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "command-sender.command-sender.get-name")?;
            resolve_sender(
                self,
                sender_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|sender| sender.getName())
            .map_err(map_core_host_error)
        })())
    }

    fn command_sender_send_message(
        &mut self,
        self_: u32,
        message: MessageMessage,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "command-sender.command-sender.send-message")?;
            resolve_sender(
                self,
                sender_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|sender| match message {
                MessageMessage::PlainText(message) => {
                    let _: () = sender.sendMessage(&message);
                }
                MessageMessage::Translatable(message) => {
                    let _: () = sender.sendTranslatableMessage(&message.text, message.parameters);
                }
            })
            .map_err(map_core_host_error)
        })())
    }

    fn command_sender_send_error_message(
        &mut self,
        self_: u32,
        message: MessageMessage,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "command-sender.command-sender.send-error-message")?;
            resolve_sender(
                self,
                sender_handle(self, self_).map_err(map_core_host_error)?,
            )
            .map(|sender| match message {
                MessageMessage::PlainText(message) => {
                    let _: () = sender.sendErrorMessage(&message);
                }
                MessageMessage::Translatable(message) => {
                    let _: () =
                        sender.sendTranslatableErrorMessage(&message.text, message.parameters);
                }
            })
            .map_err(map_core_host_error)
        })())
    }
}

impl crate::core_host::imports::HostBlockCommandSender for PluginStoreState {
    fn get_block(&mut self, sender: u32) -> Result<Result<u32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "block-command-sender.get-block")?;
            let block = resolve_sender(
                self,
                sender_handle(self, sender).map_err(map_core_host_error)?,
            )
            .map(|sender| sender.getBlock())
            .map_err(map_core_host_error)?;
            self.insert_block_resource(block)
                .map_err(map_core_host_error)
        })())
    }
}
