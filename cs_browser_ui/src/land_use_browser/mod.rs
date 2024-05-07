use kay::{World, ActorSystem, TypedID};
use compact::{CVec, CHashMap, COption};
use stdweb::serde::Serde;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use stdweb::js_export;
use browser_utils::{to_js_mesh, flatten_instances};
use SYSTEM;
use cb_util::config_manager::{Name, ConfigUser, ConfigUserID};
use ::std::collections::HashMap;
use ::land_use::buildings::{BuildingID, BuildingStyle};
use ::land_use::buildings::architecture::{build_building};
use ::land_use::buildings::architecture::language::ArchitectureRule;
use ::land_use::buildings::architecture::materials_and_props::{ALL_MATERIALS, ALL_PROP_TYPES};
use ::land_use::zone_planning::Lot;
use ::economy::households::HouseholdID;

#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), js_export)]
pub fn get_building_info(building_id: Serde<BuildingID>) {
    let system = unsafe { &mut *SYSTEM };
    let world = &mut system.world();
    building_id
        .0
        .get_ui_info(BrowserLandUseUIID::local_first(world).into(), world);
}

#[derive(Compact, Clone)]
pub struct BrowserLandUseUI {
    id: BrowserLandUseUIID,
    architecture_rules: CHashMap<Name, ArchitectureRule>,
}

impl BrowserLandUseUI {
    pub fn spawn(id: BrowserLandUseUIID, world: &mut World) -> BrowserLandUseUI {
        {
            BuildingID::global_broadcast(world).get_render_info(id.into(), world);
        }

        let ui = BrowserLandUseUI {
            id,
            architecture_rules: CHashMap::new(),
        };
        ui.get_initial_config(world);
        ui
    }
}

impl ConfigUser<ArchitectureRule> for BrowserLandUseUI {
    fn local_cache(&mut self) -> &mut CHashMap<Name, ArchitectureRule> {
        &mut self.architecture_rules
    }

    fn on_config_change(
        &mut self,
        name: Name,
        maybe_value: &COption<ArchitectureRule>,
        world: &mut World,
    ) {
        // TODO: make this more specific
        BuildingID::global_broadcast(world).get_render_info(self.id.into(), world);

        self.apply_config_change(name, maybe_value, world);
    }
}

use land_use::ui::{LandUseUI, LandUseUIID};

impl LandUseUI for BrowserLandUseUI {
    fn on_building_constructed(
        &mut self,
        id: BuildingID,
        lot: &Lot,
        households: &CVec<HouseholdID>,
        style: BuildingStyle,
        world: &mut World,
    ) {
        let result = build_building(lot, style, &self.architecture_rules, households, world);
        match result {
            Ok(building_mesh) => {
                let material_updates: ::stdweb::Object = building_mesh
                    .meshes
                    .into_iter()
                    .map(|(material, mesh)| {
                        let update_op: ::stdweb::Object = Some(("$set", to_js_mesh(&mesh)))
                            .into_iter()
                            .collect::<HashMap<_, _>>()
                            .into();
                        let material_update: ::stdweb::Object =
                            Some((id.as_raw_string(), update_op))
                                .into_iter()
                                .collect::<HashMap<_, _>>()
                                .into();
                        (material.to_string(), material_update)
                    })
                    .collect::<HashMap<_, _>>()
                    .into();

                let prop_updates: ::stdweb::Object = building_mesh
                    .props
                    .into_iter()
                    .map(|(prop_type, instances)| {
                        let update_op: ::stdweb::Object =
                            Some(("$set", flatten_instances(&instances)))
                                .into_iter()
                                .collect::<HashMap<_, _>>()
                                .into();
                        let material_update: ::stdweb::Object =
                            Some((id.as_raw_string(), update_op))
                                .into_iter()
                                .collect::<HashMap<_, _>>()
                                .into();
                        (prop_type.to_string(), material_update)
                    })
                    .collect::<HashMap<_, _>>()
                    .into();;

                js! {
                    window.cbReactApp.boundSetState(oldState => update(oldState, {
                        landUse: {rendering: {
                            buildingMeshes: @{material_updates},
                            buildingProps: @{prop_updates}
                        }},
                        households: {
                            buildingPositions: {[@{Serde(id)}]: {
                                "$set": @{Serde(lot.center_point())}
                            }},
                            buildingShapes: {[@{Serde(id)}]: {
                                "$set": @{Serde(lot.area.clone())}
                            }}
                        }
                    }));
                };
            }
            Err(reason) => {
                js! {console.error("Building geometry generation error", @{reason})};
            }
        };
    }

    fn on_building_destructed(&mut self, id: BuildingID, _world: &mut World) {
        let unset_op: ::stdweb::Object = Some(("$unset", vec![id.as_raw_string()]))
            .into_iter()
            .collect::<HashMap<_, _>>()
            .into();
        let material_unsets: ::stdweb::Object = ALL_MATERIALS
            .iter()
            .map(|material| (material.to_string(), unset_op.clone()))
            .collect::<HashMap<_, _>>()
            .into();
        let prop_unsets: ::stdweb::Object = ALL_PROP_TYPES
            .iter()
            .map(|prop_type| (prop_type.to_string(), unset_op.clone()))
            .collect::<HashMap<_, _>>()
            .into();
        js! {
            window.cbReactApp.boundSetState(oldState => update(oldState, {
                landUse: {rendering: {
                    buildingMeshes: @{material_unsets},
                    buildingProps: @{prop_unsets}
                }},
                households: {buildingPositions: {"$unset": [@{Serde(id)}]}}
            }));
        }
    }

    fn on_building_ui_info(
        &mut self,
        _id: BuildingID,
        style: BuildingStyle,
        households: &CVec<HouseholdID>,
        _world: &mut World,
    ) {
        js! {
            window.cbReactApp.boundSetState(oldState => update(oldState, {
                households: {
                    inspectedBuildingState: {"$set": {
                        households: @{Serde(households)},
                        style: @{Serde(style)},
                    }}
                }
            }));
        }
    }
}

mod kay_auto;
pub use self::kay_auto::*;

pub fn setup(system: &mut ActorSystem) {
    system.register::<BrowserLandUseUI>();
    auto_setup(system);
}

pub fn spawn(world: &mut World) {
    BrowserLandUseUIID::spawn(world);
}
