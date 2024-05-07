use kay::{ActorSystem, World, Actor};
use cb_util::random::{seed, Rng};

use cb_time::actors::{Temporal, TemporalID, TimeID};
use cb_time::units::{TimeOfDay, TimeOfDayRange, Instant, Duration, Ticks};
use economy::resources::Resource;
use economy::resources::Resource::*;
use economy::market::{Deal, EvaluationRequester, EvaluationRequesterID, EvaluatedSearchResult};
use land_use::buildings::BuildingID;
use transport::pathfinding::trip::{TripResult, TripListenerID};
use transport::pathfinding::RoughLocationID;

pub mod names;
use self::names::{family_name, member_name};

use economy::households::{Household, HouseholdID, HouseholdCore,
MemberIdx, Offer, OfferID, OfferIdx};

#[derive(Compact, Clone)]
pub struct Family {
    id: FamilyID,
    home: BuildingID,
    core: HouseholdCore,
}

impl Family {
    pub fn move_into(
        id: FamilyID,
        n_members: u32,
        home: BuildingID,
        time: TimeID,
        world: &mut World,
    ) -> Family {
        time.wake_up_in(Ticks(0), id.into(), world);

        let mut core = HouseholdCore::new(
            id.into(),
            world,
            n_members as usize,
            home.into(),
            vec![Offer::new(
                MemberIdx(0),
                TimeOfDayRange::new(16, 0, 11, 0),
                Deal::new(Some((Wakefulness, 3.0)), Duration::from_hours(1)),
                1,
                true,
            )]
            .into(),
        );

        core.used_offers.insert(
            Wakefulness,
            OfferID {
                household: id.into(),
                idx: OfferIdx(0),
            },
        );

        Family { id, home, core }
    }
}

use cb_time::actors::{Sleeper, SleeperID};

impl Sleeper for Family {
    fn wake(&mut self, current_instant: Instant, world: &mut World) {
        self.update_core(current_instant, world);
    }
}

use economy::households::ResultAspect;

impl EvaluationRequester for Family {
    fn expect_n_results(&mut self, resource: Resource, n: u32, world: &mut World) {
        self.update_results(resource, &ResultAspect::SetTarget(n), world);
    }

    fn on_result(&mut self, result: &EvaluatedSearchResult, world: &mut World) {
        let &EvaluatedSearchResult {
            resource,
            ref evaluated_deals,
            ..
        } = result;
        self.update_results(
            resource,
            &ResultAspect::AddDeals(evaluated_deals.clone()),
            world,
        );
    }
}

use transport::pathfinding::trip::{TripListener, TripID};

impl TripListener for Family {
    fn trip_created(&mut self, trip: TripID, world: &mut World) {
        self.on_trip_created(trip, world);
    }

    fn trip_result(
        &mut self,
        trip: TripID,
        result: TripResult,
        rough_source: RoughLocationID,
        rough_destination: RoughLocationID,
        world: &mut World,
    ) {
        self.on_trip_result(trip, result, rough_source, rough_destination, world);
    }
}

impl Household for Family {
    fn core(&self) -> &HouseholdCore {
        &self.core
    }

    fn core_mut(&mut self) -> &mut HouseholdCore {
        &mut self.core
    }

    fn site(&self) -> RoughLocationID {
        self.home.into()
    }

    fn is_shared(resource: Resource) -> bool {
        match resource {
            Wakefulness | Satiety => false,
            Money | Groceries => true,
            _ => unimplemented!(),
        }
    }

    fn supplier_shared(resource: Resource) -> bool {
        match resource {
            Money => false,
            Wakefulness | Satiety | Groceries => true,
            _ => unimplemented!(),
        }
    }

    fn importance(resource: Resource, time: TimeOfDay) -> f32 {
        let hour = time.hours_minutes().0;

        let bihourly_importance = match resource {
            Wakefulness => Some([7, 7, 7, 7, 5, 5, 5, 5, 5, 5, 7, 7]),
            Satiety => Some([0, 0, 5, 5, 1, 5, 5, 1, 5, 5, 1, 1]),
            Money => Some([0, 0, 3, 3, 5, 5, 5, 3, 3, 1, 1, 1]),
            Groceries => Some([0, 0, 4, 4, 1, 4, 4, 4, 4, 4, 0, 0]),
            _ => None,
        };

        bihourly_importance
            .map(|lookup| lookup[hour / 2] as f32)
            .unwrap_or(0.0)
    }

    fn interesting_resources() -> &'static [Resource] {
        &[
            Wakefulness,
            Satiety, //Entertainment,
            Money,
            Groceries,
            /* Furniture,
             *Clothes,
             *Devices,
             *Services, */
        ]
    }

    fn decay(&mut self, dt: Duration, _: &mut World) {
        for (i, member_resources) in self.core.member_resources.iter_mut().enumerate() {
            {
                let individuality = seed((self.id, i)).gen_range(0.8, 1.2);
                let wakefulness = member_resources.mut_entry_or(Wakefulness, 0.0);
                *wakefulness -= 1.0 * individuality * dt.as_hours();
            }
            {
                let individuality = seed((self.id, i, 1u8)).gen_range(0.8, 1.2);
                let satiety = member_resources.mut_entry_or(Satiety, 0.0);
                if *satiety < 0.0 {
                    let groceries = self.core.resources.mut_entry_or(Groceries, 0.0);
                    *groceries -= 1.0;
                    *satiety += 1.0;
                }
                *satiety -= 3.0 * individuality * dt.as_days();
            }
            // {
            //     let individuality = seed((self.id, i)).gen_range(0.8, 1.2);
            //     let entertainment = member_resources.mut_entry_or(Entertainment, 0.0);
            //     *entertainment -= 0.2 * individuality * dt.as_hours();
            // }
        }
        // {
        //     let individuality = seed(self.id).gen_range(0.8, 1.2);
        //     let furniture = self.core.resources.mut_entry_or(Furniture, 0.0);
        //     *furniture -= 0.005 * individuality * dt.as_hours();
        // }
        // {
        //     let individuality = seed(self.id).gen_range(0.8, 1.2);
        //     let devices = self.core.resources.mut_entry_or(Devices, 0.0);
        //     *devices -= 0.005 * individuality * dt.as_hours();
        // }
        // {
        //     let individuality = seed(self.id).gen_range(0.8, 1.2);
        //     let services = self.core.resources.mut_entry_or(Services, 0.0);
        //     *services -= 0.01 * individuality * dt.as_hours();
        // }
    }

    fn on_destroy(&mut self, world: &mut World) {
        self.home.remove_household(self.id_as(), world);
    }

    fn household_name(&self) -> String {
        format!("The {} Family", family_name(self.id))
    }

    fn member_name(&self, member: MemberIdx) -> String {
        member_name(self.id, member)
    }
}

impl Temporal for Family {
    fn tick(&mut self, _dt: f32, current_instant: Instant, world: &mut World) {
        self.on_tick(current_instant, world);
    }
}

use transport::pathfinding::{RoughLocation, RoughLocationResolve};

impl RoughLocation for Family {
    fn resolve(&self) -> RoughLocationResolve {
        RoughLocationResolve::SameAs(self.site())
    }
}

pub fn setup(system: &mut ActorSystem) {
    system.register::<Family>();
    auto_setup(system);
}

mod kay_auto;
pub use self::kay_auto::*;
