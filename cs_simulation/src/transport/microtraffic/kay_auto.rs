//! This is all auto-generated. Do not touch.
#![rustfmt::skip]
#[allow(unused_imports)]
use kay::{ActorSystem, TypedID, RawID, Fate, Actor, TraitIDFrom, ActorOrActorTrait};
#[allow(unused_imports)]
use super::*;

#[derive(Serialize, Deserialize)] #[serde(transparent)]
pub struct LaneLikeID {
    _raw_id: RawID
}

impl Copy for LaneLikeID {}
impl Clone for LaneLikeID { fn clone(&self) -> Self { *self } }
impl ::std::fmt::Debug for LaneLikeID {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "LaneLikeID({:?})", self._raw_id)
    }
}
impl ::std::hash::Hash for LaneLikeID {
    fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
        self._raw_id.hash(state);
    }
}
impl PartialEq for LaneLikeID {
    fn eq(&self, other: &LaneLikeID) -> bool {
        self._raw_id == other._raw_id
    }
}
impl Eq for LaneLikeID {}

pub struct LaneLikeRepresentative;

impl ActorOrActorTrait for LaneLikeRepresentative {
    type ID = LaneLikeID;
}

impl TypedID for LaneLikeID {
    type Target = LaneLikeRepresentative;

    fn from_raw(id: RawID) -> Self {
        LaneLikeID { _raw_id: id }
    }

    fn as_raw(&self) -> RawID {
        self._raw_id
    }
}

impl<Act: Actor + LaneLike> TraitIDFrom<Act> for LaneLikeID {}

impl LaneLikeID {
    pub fn add_car(self, car: LaneCar, from: Option < LaneLikeID >, instant: Instant, world: &mut World) {
        world.send(self.as_raw(), MSG_LaneLike_add_car(car, from, instant));
    }
    
    pub fn add_obstacles(self, obstacles: CVec < Obstacle >, from: LaneLikeID, world: &mut World) {
        world.send(self.as_raw(), MSG_LaneLike_add_obstacles(obstacles, from));
    }

    pub fn register_trait(system: &mut ActorSystem) {
        system.register_trait::<LaneLikeRepresentative>();
        system.register_trait_message::<MSG_LaneLike_add_car>();
        system.register_trait_message::<MSG_LaneLike_add_obstacles>();
    }

    pub fn register_implementor<Act: Actor + LaneLike>(system: &mut ActorSystem) {
        system.register_implementor::<Act, LaneLikeRepresentative>();
        system.add_handler::<Act, _, _>(
            |&MSG_LaneLike_add_car(car, from, instant), instance, world| {
                instance.add_car(car, from, instant, world); Fate::Live
            }, false
        );
        
        system.add_handler::<Act, _, _>(
            |&MSG_LaneLike_add_obstacles(ref obstacles, from), instance, world| {
                instance.add_obstacles(obstacles, from, world); Fate::Live
            }, false
        );
    }
}

#[derive(Compact, Clone)] #[allow(non_camel_case_types)]
struct MSG_LaneLike_add_car(pub LaneCar, pub Option < LaneLikeID >, pub Instant);
#[derive(Compact, Clone)] #[allow(non_camel_case_types)]
struct MSG_LaneLike_add_obstacles(pub CVec < Obstacle >, pub LaneLikeID);



impl LaneID {
    pub fn on_signal_changed(self, from: LaneID, new_green: bool, world: &mut World) {
        world.send(self.as_raw(), MSG_Lane_on_signal_changed(from, new_green));
    }
}

#[derive(Compact, Clone)] #[allow(non_camel_case_types)]
struct MSG_Lane_on_signal_changed(pub LaneID, pub bool);

impl Into<LaneLikeID> for LaneID {
    fn into(self) -> LaneLikeID {
        LaneLikeID::from_raw(self.as_raw())
    }
}

impl Into<TemporalID> for LaneID {
    fn into(self) -> TemporalID {
        TemporalID::from_raw(self.as_raw())
    }
}


impl SwitchLaneID {
    
}



impl Into<LaneLikeID> for SwitchLaneID {
    fn into(self) -> LaneLikeID {
        LaneLikeID::from_raw(self.as_raw())
    }
}

impl Into<TemporalID> for SwitchLaneID {
    fn into(self) -> TemporalID {
        TemporalID::from_raw(self.as_raw())
    }
}

#[allow(unused_variables)]
#[allow(unused_mut)]
pub fn auto_setup(system: &mut ActorSystem) {
    LaneLikeID::register_trait(system);
    LaneLikeID::register_implementor::<Lane>(system);
    TemporalID::register_implementor::<Lane>(system);
    system.add_handler::<Lane, _, _>(
        |&MSG_Lane_on_signal_changed(from, new_green), instance, world| {
            instance.on_signal_changed(from, new_green, world); Fate::Live
        }, false
    );
    LaneLikeID::register_implementor::<SwitchLane>(system);
    TemporalID::register_implementor::<SwitchLane>(system);
}