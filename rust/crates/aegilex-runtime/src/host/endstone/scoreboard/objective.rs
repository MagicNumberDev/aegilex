//! Core ABI implementation for `native/bindings/endstone/scoreboard/objective.h`.

use crate::host::endstone::support::*;

impl crate::core_host::imports::HostObjective for PluginStoreState {
    fn objective_get_name(&mut self, self_: u32) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "objective.objective.get-name")?;
            objective_value(self, self_)
                .map(|value| value.name.clone())
                .map_err(map_core_host_error)
        })())
    }

    fn objective_get_display_name(
        &mut self,
        self_: u32,
    ) -> Result<Result<String, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "objective.objective.get-display-name")?;
            let value = objective_value(self, self_).map_err(map_core_host_error)?;
            Ok(resolve_objective_value(self, value)
                .map_err(map_core_host_error)?
                .getDisplayName())
        })())
    }

    fn objective_set_display_name(
        &mut self,
        self_: u32,
        display_name: String,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "objective.objective.set-display-name")?;
            let value = objective_value(self, self_).map_err(map_core_host_error)?;
            resolve_objective_value(self, value)
                .map_err(map_core_host_error)?
                .setDisplayName(&display_name);
            Ok(())
        })())
    }

    fn objective_get_criteria(
        &mut self,
        self_: u32,
    ) -> Result<Result<CriteriaCriteria, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "objective.objective.get-criteria")?;
            let value = objective_value(self, self_).map_err(map_core_host_error)?;
            let criteria = resolve_objective_value(self, value)
                .map_err(map_core_host_error)?
                .getCriteria();
            match criteria {
                0 => Ok(CriteriaCriteria {
                    name: "dummy".to_owned(),
                    read_only: false,
                    default_render_type: RenderTypeRenderType::Integer,
                }),
                _ => Err(map_core_host_error(invalid_input())),
            }
        })())
    }

    fn objective_is_modifiable(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "objective.objective.is-modifiable")?;
            let value = objective_value(self, self_).map_err(map_core_host_error)?;
            Ok(resolve_objective_value(self, value)
                .map_err(map_core_host_error)?
                .isModifiable())
        })())
    }

    fn objective_is_displayed(
        &mut self,
        self_: u32,
    ) -> Result<Result<bool, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "objective.objective.is-displayed")?;
            let value = objective_value(self, self_).map_err(map_core_host_error)?;
            Ok(resolve_objective_value(self, value)
                .map_err(map_core_host_error)?
                .isDisplayed())
        })())
    }

    fn objective_get_display_slot(
        &mut self,
        self_: u32,
    ) -> Result<Result<Option<DisplaySlotDisplaySlot>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "objective.objective.get-display-slot")?;
            let value = objective_value(self, self_).map_err(map_core_host_error)?;
            let objective = resolve_objective_value(self, value).map_err(map_core_host_error)?;
            let mut slot = 0;
            objective
                .getDisplaySlot(&mut slot)
                .then(|| from_slot(slot).map_err(map_core_host_error))
                .transpose()
        })())
    }

    fn objective_set_display_slot(
        &mut self,
        self_: u32,
        slot: Option<DisplaySlotDisplaySlot>,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "objective.objective.set-display-slot")?;
            let value = objective_value(self, self_).map_err(map_core_host_error)?;
            resolve_objective_value(self, value)
                .map_err(map_core_host_error)?
                .setDisplaySlot(slot.is_some(), slot.map_or(0, |slot| slot as u32));
            Ok(())
        })())
    }

    fn objective_get_sort_order(
        &mut self,
        self_: u32,
    ) -> Result<Result<Option<ObjectiveSortOrderObjectiveSortOrder>, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "objective.objective.get-sort-order")?;
            let value = objective_value(self, self_).map_err(map_core_host_error)?;
            let objective = resolve_objective_value(self, value).map_err(map_core_host_error)?;
            let mut order = 0;
            objective
                .getSortOrder(&mut order)
                .then(|| from_sort_order(order).map_err(map_core_host_error))
                .transpose()
        })())
    }

    fn objective_set_sort_order(
        &mut self,
        self_: u32,
        order: ObjectiveSortOrderObjectiveSortOrder,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "objective.objective.set-sort-order")?;
            let value = objective_value(self, self_).map_err(map_core_host_error)?;
            resolve_objective_value(self, value)
                .map_err(map_core_host_error)?
                .setSortOrder(order as u32);
            Ok(())
        })())
    }

    fn objective_set_display(
        &mut self,
        self_: u32,
        slot: Option<DisplaySlotDisplaySlot>,
        order: ObjectiveSortOrderObjectiveSortOrder,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "objective.objective.set-display")?;
            let value = objective_value(self, self_).map_err(map_core_host_error)?;
            resolve_objective_value(self, value)
                .map_err(map_core_host_error)?
                .setDisplay(
                    slot.is_some(),
                    slot.map_or(0, |slot| slot as u32),
                    order as u32,
                );
            Ok(())
        })())
    }

    fn objective_get_render_type(
        &mut self,
        self_: u32,
    ) -> Result<Result<RenderTypeRenderType, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "objective.objective.get-render-type")?;
            let value = objective_value(self, self_).map_err(map_core_host_error)?;
            from_render_type(
                resolve_objective_value(self, value)
                    .map_err(map_core_host_error)?
                    .getRenderType(),
            )
            .map_err(map_core_host_error)
        })())
    }

    fn objective_get_score_value(
        &mut self,
        self_: u32,
        entry: u32,
    ) -> Result<Result<i32, TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "objective.objective.get-score-value")?;
            let value = objective_value(self, self_).map_err(map_core_host_error)?;
            let objective = resolve_objective_value(self, value).map_err(map_core_host_error)?;
            let entry = resolve_score_entry(
                self,
                score_entry_handle(self, entry).map_err(map_core_host_error)?,
            )
            .map_err(map_core_host_error)?;
            get_objective_score(&objective, entry).map_err(map_core_host_error)
        })())
    }

    fn objective_set_score_value(
        &mut self,
        self_: u32,
        entry: u32,
        value: i32,
    ) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "objective.objective.set-score-value")?;
            let stored = objective_value(self, self_).map_err(map_core_host_error)?;
            let objective = resolve_objective_value(self, stored).map_err(map_core_host_error)?;
            let entry = resolve_score_entry(
                self,
                score_entry_handle(self, entry).map_err(map_core_host_error)?,
            )
            .map_err(map_core_host_error)?;
            set_objective_score(&objective, entry, value).map_err(map_core_host_error)
        })())
    }

    fn objective_unregister(&mut self, self_: u32) -> Result<Result<(), TypesHostError>, String> {
        Ok((|| {
            check_capability(self, "objective.objective.unregister")?;
            let stored = objective_value(self, self_).map_err(map_core_host_error)?;
            resolve_objective_value(self, stored)
                .map_err(map_core_host_error)?
                .unregister();
            Ok(())
        })())
    }
}
