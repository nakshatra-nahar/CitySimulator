use kay::TypedID;
use stdweb::serde::Serde;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use stdweb::js_export;
use SYSTEM;

use cb_planning::GestureID;
use cb_planning::plan_manager::ProjectID;
use planning::{CBPlanManagerID, CBGestureIntent};
use transport::transport_planning::RoadLaneConfig;
use descartes::{Corner};

#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), js_export)]
pub fn plan_grid(project_id: Serde<ProjectID>, n: Serde<isize>, n_lanes: Serde<u8>, spacing: Serde<f32>) {
    let system = unsafe { &mut *SYSTEM };
    let world = &mut system.world();

    let plan_manager = CBPlanManagerID::global_first(world);

    use ::transport::transport_planning::RoadIntent;
    use ::descartes::P2;

    for x in -n.0 / 2..n.0 / 2 {
        let id = GestureID::new();
        let p1 = P2::new(x as f32 * spacing.0, (-n.0 / 2) as f32 * spacing.0);
        let p2 = P2::new(x as f32 * spacing.0, (n.0 / 2) as f32 * spacing.0);
        let d = (p2 - p1).normalize();
        plan_manager.start_new_gesture(
            project_id.0,
            id,
            CBGestureIntent::Road(RoadIntent::new(
                vec![Corner::new(p1, Some(d), Some(d)), Corner::new(p2, Some(d), Some(d))],
                RoadLaneConfig {
                    n_lanes_forward: n_lanes.0,
                    n_lanes_backward: n_lanes.0
                }
            )),
            world,
        );
    }

    for y in -n.0 / 2..n.0 / 2 {
        let id = GestureID::new();
        let p1 = P2::new((-n.0 / 2) as f32 * spacing.0, y as f32 * spacing.0);
        let p2 = P2::new((n.0 / 2) as f32 * spacing.0, y as f32 * spacing.0);
        let d = (p2 - p1).normalize();
        plan_manager.start_new_gesture(
            project_id.0,
            id,
            CBGestureIntent::Road(RoadIntent::new(
                vec![Corner::new(p1, Some(d), Some(d)), Corner::new(p2, Some(d), Some(d))],
                RoadLaneConfig {
                    n_lanes_forward: n_lanes.0,
                    n_lanes_backward: n_lanes.0
                }
            )),
            world,
        );
    }
}

#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), js_export)]
pub fn spawn_cars(tries_per_lane: usize) {
    let system = unsafe { &mut *SYSTEM };
    let world = &mut system.world();
    for _ in 0..tries_per_lane {
        ::transport::lane::LaneID::global_broadcast(world).manually_spawn_car_add_lane(world);
    }
}

use kay::{World, ActorSystem};
use compact::{CVec, CString};
use cb_util::log::{LogID, LogRecipient, LogRecipientID, Entry};

#[derive(Compact, Clone)]
pub struct LogUI {
    id: LogUIID,
}

impl LogUI {
    pub fn spawn(id: LogUIID, _: &mut World) -> LogUI {
        LogUI { id }
    }
}

impl LogRecipient for LogUI {
    fn receive_newest_logs(
        &mut self,
        entries: &CVec<Entry>,
        text: &CString,
        effective_last: u32,
        effective_text_start: u32,
        _: &mut World,
    ) {
        js! {
            const entries = @{Serde(entries)};
            const text = @{Serde(text)};
            if (window.cbReactApp.state.debug.logLastEntry == @{effective_last as u32}) {
                // append
                window.cbReactApp.boundSetState(oldState => update(oldState, {
                    debug: {
                        logLastEntry: {"$apply": n => n + @{entries.len() as u32}},
                        logEntries: {"$push": entries},
                        logText: {"$apply": t => t + text}
                    }
                }));
            } else {
                // replace, keep offset
                window.cbReactApp.boundSetState(oldState => update(oldState, {
                    debug: {
                        logLastEntry: {"$set": @{effective_last + entries.len() as u32}},
                        logTextStart: {"$set": @{effective_text_start}},
                        logFirstEntry: {"$set": @{effective_last}},
                        logEntries: {"$set": entries},
                        logText: {"$set": text}
                    }
                }));
            }
        };
    }
}

#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), js_export)]
pub fn get_newest_log_messages() {
    let system = unsafe { &mut *SYSTEM };
    let world = &mut system.world();

    use ::stdweb::unstable::TryInto;

    let last_log_entry: u32 = js! {
        return window.cbReactApp.state.debug.logLastEntry;
    }
    .try_into()
    .unwrap();

    // TODO: ugly
    LogID::global_broadcast(world).get_after(
        last_log_entry,
        500,
        LogUIID::local_first(world).into(),
        world,
    );
}

mod kay_auto;
pub use self::kay_auto::*;

pub fn setup(system: &mut ActorSystem) {
    system.register::<LogUI>();
    auto_setup(system);
}

pub fn spawn(world: &mut World) {
    LogUIID::spawn(world);
}
