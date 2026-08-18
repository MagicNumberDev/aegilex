//! Core ABI implementation for `native/bindings/endstone/events/weather_change_event_facade.h`.

use super::support::*;



fn resolve_weather_change_event(
    state: &PluginStoreState,
    event: u32,
) -> Result<&cxx_event::WeatherChangeEventFacade, HostError> {
    let handle = event_handle(state, event, ResourceKind::WeatherChangeEvent)?;
    state
        .handles
        .weather_change_event(state.invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

fn resolve_weather_change_event_mut(
    state: &mut PluginStoreState,
    event: u32,
) -> Result<std::pin::Pin<&mut cxx_event::WeatherChangeEventFacade>, HostError> {
    let handle = event_handle(state, event, ResourceKind::WeatherChangeEvent)?;
    let invocation_id = state.invocation_id;
    state
        .handles
        .weather_change_event_mut(invocation_id, handle)
        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
}

impl crate::core_host::imports::HostWeatherChangeEvent for PluginStoreState {
    fn weather_change_event_get_to_weather(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "weather-change-event.weather-change-event.get-to-weather",
            )?;
            resolve_weather_change_event(self, self_)
                .map(|event| event.getToWeather())
                .map_err(map_core_host_error)
        })())
    }

    fn weather_change_event_is_cancelled(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "weather-change-event.weather-change-event.is-cancelled",
            )?;
            resolve_weather_change_event(self, self_)
                .map(|event| event.isCancelled())
                .map_err(map_core_host_error)
        })())
    }

    fn weather_change_event_set_cancelled(
        &mut self,
        self_: u32,
        cancelled: bool,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(
                self,
                "weather-change-event.weather-change-event.set-cancelled",
            )?;
            resolve_weather_change_event_mut(self, self_)
                .and_then(|event| {
                    event
                        .setCancelled(cancelled)
                        .then_some(())
                        .ok_or_else(|| HostError::from_status(AEGILEX_NOT_FOUND))
                })
                .map_err(map_core_host_error)?;
            Ok(())
        })())
    }
}
