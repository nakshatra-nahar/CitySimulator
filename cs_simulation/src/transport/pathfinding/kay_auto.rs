//! This is all auto-generated. Do not touch.
#![rustfmt::skip]
#[allow(unused_imports)]
use kay::{ActorSystem, TypedID, RawID, Fate, Actor, TraitIDFrom, ActorOrActorTrait};
#[allow(unused_imports)]
use super::*;

#[derive(Serialize, Deserialize)] #[serde(transparent)]
pub struct LinkID {
    _raw_id: RawID
}

impl Copy for LinkID {}
impl Clone for LinkID { fn clone(&self) -> Self { *self } }
impl ::std::fmt::Debug for LinkID {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "LinkID({:?})", self._raw_id)
    }
}
impl ::std::hash::Hash for LinkID {
    fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
        self._raw_id.hash(state);
    }
}
impl PartialEq for LinkID {
    fn eq(&self, other: &LinkID) -> bool {
        self._raw_id == other._raw_id
    }
}
impl Eq for LinkID {}

pub struct LinkRepresentative;

impl ActorOrActorTrait for LinkRepresentative {
    type ID = LinkID;
}

impl TypedID for LinkID {
    type Target = LinkRepresentative;

    fn from_raw(id: RawID) -> Self {
        LinkID { _raw_id: id }
    }

    fn as_raw(&self) -> RawID {
        self._raw_id
    }
}

impl<Act: Actor + Link> TraitIDFrom<Act> for LinkID {}

impl LinkID {
    pub fn after_route_forgotten(self, forgotten_route: Location, world: &mut World) {
        world.send(self.as_raw(), MSG_Link_after_route_forgotten(forgotten_route));
    }
    
    pub fn pathfinding_tick(self, world: &mut World) {
        world.send(self.as_raw(), MSG_Link_pathfinding_tick());
    }
    
    pub fn query_routes(self, requester: LinkID, connection_cost: f32, world: &mut World) {
        world.send(self.as_raw(), MSG_Link_query_routes(requester, connection_cost));
    }
    
    pub fn on_routes(self, new_routes: CDict < Location , CommunicatedRoutingEntry >, from: LinkID, world: &mut World) {
        world.send(self.as_raw(), MSG_Link_on_routes(new_routes, from));
    }
    
    pub fn forget_routes(self, forget: CVec < Location >, from: LinkID, world: &mut World) {
        world.send(self.as_raw(), MSG_Link_forget_routes(forget, from));
    }
    
    pub fn join_landmark(self, from: LinkID, join_as: Location, hops_from_landmark: u8, world: &mut World) {
        world.send(self.as_raw(), MSG_Link_join_landmark(from, join_as, hops_from_landmark));
    }
    
    pub fn get_distance_to(self, destination: Location, requester: DistanceRequesterID, world: &mut World) {
        world.send(self.as_raw(), MSG_Link_get_distance_to(destination, requester));
    }
    
    pub fn add_attachee(self, attachee: AttacheeID, world: &mut World) {
        world.send(self.as_raw(), MSG_Link_add_attachee(attachee));
    }
    
    pub fn remove_attachee(self, attachee: AttacheeID, world: &mut World) {
        world.send(self.as_raw(), MSG_Link_remove_attachee(attachee));
    }

    pub fn register_trait(system: &mut ActorSystem) {
        system.register_trait::<LinkRepresentative>();
        system.register_trait_message::<MSG_Link_after_route_forgotten>();
        system.register_trait_message::<MSG_Link_pathfinding_tick>();
        system.register_trait_message::<MSG_Link_query_routes>();
        system.register_trait_message::<MSG_Link_on_routes>();
        system.register_trait_message::<MSG_Link_forget_routes>();
        system.register_trait_message::<MSG_Link_join_landmark>();
        system.register_trait_message::<MSG_Link_get_distance_to>();
        system.register_trait_message::<MSG_Link_add_attachee>();
        system.register_trait_message::<MSG_Link_remove_attachee>();
    }

    pub fn register_implementor<Act: Actor + Link>(system: &mut ActorSystem) {
        system.register_implementor::<Act, LinkRepresentative>();
        system.add_handler::<Act, _, _>(
            |&MSG_Link_after_route_forgotten(forgotten_route), instance, world| {
                instance.after_route_forgotten(forgotten_route, world); Fate::Live
            }, false
        );
        
        system.add_handler::<Act, _, _>(
            |&MSG_Link_pathfinding_tick(), instance, world| {
                instance.pathfinding_tick(world); Fate::Live
            }, false
        );
        
        system.add_handler::<Act, _, _>(
            |&MSG_Link_query_routes(requester, connection_cost), instance, world| {
                instance.query_routes(requester, connection_cost, world); Fate::Live
            }, false
        );
        
        system.add_handler::<Act, _, _>(
            |&MSG_Link_on_routes(ref new_routes, from), instance, world| {
                instance.on_routes(new_routes, from, world); Fate::Live
            }, false
        );
        
        system.add_handler::<Act, _, _>(
            |&MSG_Link_forget_routes(ref forget, from), instance, world| {
                instance.forget_routes(forget, from, world); Fate::Live
            }, false
        );
        
        system.add_handler::<Act, _, _>(
            |&MSG_Link_join_landmark(from, join_as, hops_from_landmark), instance, world| {
                instance.join_landmark(from, join_as, hops_from_landmark, world); Fate::Live
            }, false
        );
        
        system.add_handler::<Act, _, _>(
            |&MSG_Link_get_distance_to(destination, requester), instance, world| {
                instance.get_distance_to(destination, requester, world); Fate::Live
            }, false
        );
        
        system.add_handler::<Act, _, _>(
            |&MSG_Link_add_attachee(attachee), instance, world| {
                instance.add_attachee(attachee, world); Fate::Live
            }, false
        );
        
        system.add_handler::<Act, _, _>(
            |&MSG_Link_remove_attachee(attachee), instance, world| {
                instance.remove_attachee(attachee, world); Fate::Live
            }, false
        );
    }
}

#[derive(Compact, Clone)] #[allow(non_camel_case_types)]
struct MSG_Link_after_route_forgotten(pub Location);
#[derive(Copy, Clone)] #[allow(non_camel_case_types)]
struct MSG_Link_pathfinding_tick();
#[derive(Compact, Clone)] #[allow(non_camel_case_types)]
struct MSG_Link_query_routes(pub LinkID, pub f32);
#[derive(Compact, Clone)] #[allow(non_camel_case_types)]
struct MSG_Link_on_routes(pub CDict < Location , CommunicatedRoutingEntry >, pub LinkID);
#[derive(Compact, Clone)] #[allow(non_camel_case_types)]
struct MSG_Link_forget_routes(pub CVec < Location >, pub LinkID);
#[derive(Compact, Clone)] #[allow(non_camel_case_types)]
struct MSG_Link_join_landmark(pub LinkID, pub Location, pub u8);
#[derive(Compact, Clone)] #[allow(non_camel_case_types)]
struct MSG_Link_get_distance_to(pub Location, pub DistanceRequesterID);
#[derive(Compact, Clone)] #[allow(non_camel_case_types)]
struct MSG_Link_add_attachee(pub AttacheeID);
#[derive(Compact, Clone)] #[allow(non_camel_case_types)]
struct MSG_Link_remove_attachee(pub AttacheeID);
#[derive(Serialize, Deserialize)] #[serde(transparent)]
pub struct AttacheeID {
    _raw_id: RawID
}

impl Copy for AttacheeID {}
impl Clone for AttacheeID { fn clone(&self) -> Self { *self } }
impl ::std::fmt::Debug for AttacheeID {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "AttacheeID({:?})", self._raw_id)
    }
}
impl ::std::hash::Hash for AttacheeID {
    fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
        self._raw_id.hash(state);
    }
}
impl PartialEq for AttacheeID {
    fn eq(&self, other: &AttacheeID) -> bool {
        self._raw_id == other._raw_id
    }
}
impl Eq for AttacheeID {}

pub struct AttacheeRepresentative;

impl ActorOrActorTrait for AttacheeRepresentative {
    type ID = AttacheeID;
}

impl TypedID for AttacheeID {
    type Target = AttacheeRepresentative;

    fn from_raw(id: RawID) -> Self {
        AttacheeID { _raw_id: id }
    }

    fn as_raw(&self) -> RawID {
        self._raw_id
    }
}

impl<Act: Actor + Attachee> TraitIDFrom<Act> for AttacheeID {}

impl AttacheeID {
    pub fn location_changed(self, old: Option < Location >, new: Option < Location >, world: &mut World) {
        world.send(self.as_raw(), MSG_Attachee_location_changed(old, new));
    }

    pub fn register_trait(system: &mut ActorSystem) {
        system.register_trait::<AttacheeRepresentative>();
        system.register_trait_message::<MSG_Attachee_location_changed>();
    }

    pub fn register_implementor<Act: Actor + Attachee>(system: &mut ActorSystem) {
        system.register_implementor::<Act, AttacheeRepresentative>();
        system.add_handler::<Act, _, _>(
            |&MSG_Attachee_location_changed(old, new), instance, world| {
                instance.location_changed(old, new, world); Fate::Live
            }, false
        );
    }
}

#[derive(Compact, Clone)] #[allow(non_camel_case_types)]
struct MSG_Attachee_location_changed(pub Option < Location >, pub Option < Location >);
#[derive(Serialize, Deserialize)] #[serde(transparent)]
pub struct RoughLocationID {
    _raw_id: RawID
}

impl Copy for RoughLocationID {}
impl Clone for RoughLocationID { fn clone(&self) -> Self { *self } }
impl ::std::fmt::Debug for RoughLocationID {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "RoughLocationID({:?})", self._raw_id)
    }
}
impl ::std::hash::Hash for RoughLocationID {
    fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
        self._raw_id.hash(state);
    }
}
impl PartialEq for RoughLocationID {
    fn eq(&self, other: &RoughLocationID) -> bool {
        self._raw_id == other._raw_id
    }
}
impl Eq for RoughLocationID {}

pub struct RoughLocationRepresentative;

impl ActorOrActorTrait for RoughLocationRepresentative {
    type ID = RoughLocationID;
}

impl TypedID for RoughLocationID {
    type Target = RoughLocationRepresentative;

    fn from_raw(id: RawID) -> Self {
        RoughLocationID { _raw_id: id }
    }

    fn as_raw(&self) -> RawID {
        self._raw_id
    }
}

impl<Act: Actor + RoughLocation> TraitIDFrom<Act> for RoughLocationID {}

impl RoughLocationID {
    pub fn resolve_as_location(self, requester: LocationRequesterID, rough_location: RoughLocationID, instant: Instant, world: &mut World) {
        world.send(self.as_raw(), MSG_RoughLocation_resolve_as_location(requester, rough_location, instant));
    }
    
    pub fn resolve_as_position(self, requester: PositionRequesterID, rough_location: RoughLocationID, world: &mut World) {
        world.send(self.as_raw(), MSG_RoughLocation_resolve_as_position(requester, rough_location));
    }

    pub fn register_trait(system: &mut ActorSystem) {
        system.register_trait::<RoughLocationRepresentative>();
        system.register_trait_message::<MSG_RoughLocation_resolve_as_location>();
        system.register_trait_message::<MSG_RoughLocation_resolve_as_position>();
    }

    pub fn register_implementor<Act: Actor + RoughLocation>(system: &mut ActorSystem) {
        system.register_implementor::<Act, RoughLocationRepresentative>();
        system.add_handler::<Act, _, _>(
            |&MSG_RoughLocation_resolve_as_location(requester, rough_location, instant), instance, world| {
                instance.resolve_as_location(requester, rough_location, instant, world); Fate::Live
            }, false
        );
        
        system.add_handler::<Act, _, _>(
            |&MSG_RoughLocation_resolve_as_position(requester, rough_location), instance, world| {
                instance.resolve_as_position(requester, rough_location, world); Fate::Live
            }, false
        );
    }
}

#[derive(Compact, Clone)] #[allow(non_camel_case_types)]
struct MSG_RoughLocation_resolve_as_location(pub LocationRequesterID, pub RoughLocationID, pub Instant);
#[derive(Compact, Clone)] #[allow(non_camel_case_types)]
struct MSG_RoughLocation_resolve_as_position(pub PositionRequesterID, pub RoughLocationID);
#[derive(Serialize, Deserialize)] #[serde(transparent)]
pub struct LocationRequesterID {
    _raw_id: RawID
}

impl Copy for LocationRequesterID {}
impl Clone for LocationRequesterID { fn clone(&self) -> Self { *self } }
impl ::std::fmt::Debug for LocationRequesterID {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "LocationRequesterID({:?})", self._raw_id)
    }
}
impl ::std::hash::Hash for LocationRequesterID {
    fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
        self._raw_id.hash(state);
    }
}
impl PartialEq for LocationRequesterID {
    fn eq(&self, other: &LocationRequesterID) -> bool {
        self._raw_id == other._raw_id
    }
}
impl Eq for LocationRequesterID {}

pub struct LocationRequesterRepresentative;

impl ActorOrActorTrait for LocationRequesterRepresentative {
    type ID = LocationRequesterID;
}

impl TypedID for LocationRequesterID {
    type Target = LocationRequesterRepresentative;

    fn from_raw(id: RawID) -> Self {
        LocationRequesterID { _raw_id: id }
    }

    fn as_raw(&self) -> RawID {
        self._raw_id
    }
}

impl<Act: Actor + LocationRequester> TraitIDFrom<Act> for LocationRequesterID {}

impl LocationRequesterID {
    pub fn location_resolved(self, rough_location: RoughLocationID, location: Option < PreciseLocation >, instant: Instant, world: &mut World) {
        world.send(self.as_raw(), MSG_LocationRequester_location_resolved(rough_location, location, instant));
    }

    pub fn register_trait(system: &mut ActorSystem) {
        system.register_trait::<LocationRequesterRepresentative>();
        system.register_trait_message::<MSG_LocationRequester_location_resolved>();
    }

    pub fn register_implementor<Act: Actor + LocationRequester>(system: &mut ActorSystem) {
        system.register_implementor::<Act, LocationRequesterRepresentative>();
        system.add_handler::<Act, _, _>(
            |&MSG_LocationRequester_location_resolved(rough_location, location, instant), instance, world| {
                instance.location_resolved(rough_location, location, instant, world); Fate::Live
            }, false
        );
    }
}

#[derive(Compact, Clone)] #[allow(non_camel_case_types)]
struct MSG_LocationRequester_location_resolved(pub RoughLocationID, pub Option < PreciseLocation >, pub Instant);
#[derive(Serialize, Deserialize)] #[serde(transparent)]
pub struct PositionRequesterID {
    _raw_id: RawID
}

impl Copy for PositionRequesterID {}
impl Clone for PositionRequesterID { fn clone(&self) -> Self { *self } }
impl ::std::fmt::Debug for PositionRequesterID {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "PositionRequesterID({:?})", self._raw_id)
    }
}
impl ::std::hash::Hash for PositionRequesterID {
    fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
        self._raw_id.hash(state);
    }
}
impl PartialEq for PositionRequesterID {
    fn eq(&self, other: &PositionRequesterID) -> bool {
        self._raw_id == other._raw_id
    }
}
impl Eq for PositionRequesterID {}

pub struct PositionRequesterRepresentative;

impl ActorOrActorTrait for PositionRequesterRepresentative {
    type ID = PositionRequesterID;
}

impl TypedID for PositionRequesterID {
    type Target = PositionRequesterRepresentative;

    fn from_raw(id: RawID) -> Self {
        PositionRequesterID { _raw_id: id }
    }

    fn as_raw(&self) -> RawID {
        self._raw_id
    }
}

impl<Act: Actor + PositionRequester> TraitIDFrom<Act> for PositionRequesterID {}

impl PositionRequesterID {
    pub fn position_resolved(self, rough_location: RoughLocationID, position: P2, world: &mut World) {
        world.send(self.as_raw(), MSG_PositionRequester_position_resolved(rough_location, position));
    }

    pub fn register_trait(system: &mut ActorSystem) {
        system.register_trait::<PositionRequesterRepresentative>();
        system.register_trait_message::<MSG_PositionRequester_position_resolved>();
    }

    pub fn register_implementor<Act: Actor + PositionRequester>(system: &mut ActorSystem) {
        system.register_implementor::<Act, PositionRequesterRepresentative>();
        system.add_handler::<Act, _, _>(
            |&MSG_PositionRequester_position_resolved(rough_location, position), instance, world| {
                instance.position_resolved(rough_location, position, world); Fate::Live
            }, false
        );
    }
}

#[derive(Compact, Clone)] #[allow(non_camel_case_types)]
struct MSG_PositionRequester_position_resolved(pub RoughLocationID, pub P2);
#[derive(Serialize, Deserialize)] #[serde(transparent)]
pub struct DistanceRequesterID {
    _raw_id: RawID
}

impl Copy for DistanceRequesterID {}
impl Clone for DistanceRequesterID { fn clone(&self) -> Self { *self } }
impl ::std::fmt::Debug for DistanceRequesterID {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "DistanceRequesterID({:?})", self._raw_id)
    }
}
impl ::std::hash::Hash for DistanceRequesterID {
    fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
        self._raw_id.hash(state);
    }
}
impl PartialEq for DistanceRequesterID {
    fn eq(&self, other: &DistanceRequesterID) -> bool {
        self._raw_id == other._raw_id
    }
}
impl Eq for DistanceRequesterID {}

pub struct DistanceRequesterRepresentative;

impl ActorOrActorTrait for DistanceRequesterRepresentative {
    type ID = DistanceRequesterID;
}

impl TypedID for DistanceRequesterID {
    type Target = DistanceRequesterRepresentative;

    fn from_raw(id: RawID) -> Self {
        DistanceRequesterID { _raw_id: id }
    }

    fn as_raw(&self) -> RawID {
        self._raw_id
    }
}

impl<Act: Actor + DistanceRequester> TraitIDFrom<Act> for DistanceRequesterID {}

impl DistanceRequesterID {
    pub fn on_distance(self, maybe_distance: Option < f32 >, world: &mut World) {
        world.send(self.as_raw(), MSG_DistanceRequester_on_distance(maybe_distance));
    }

    pub fn register_trait(system: &mut ActorSystem) {
        system.register_trait::<DistanceRequesterRepresentative>();
        system.register_trait_message::<MSG_DistanceRequester_on_distance>();
    }

    pub fn register_implementor<Act: Actor + DistanceRequester>(system: &mut ActorSystem) {
        system.register_implementor::<Act, DistanceRequesterRepresentative>();
        system.add_handler::<Act, _, _>(
            |&MSG_DistanceRequester_on_distance(maybe_distance), instance, world| {
                instance.on_distance(maybe_distance, world); Fate::Live
            }, false
        );
    }
}

#[derive(Compact, Clone)] #[allow(non_camel_case_types)]
struct MSG_DistanceRequester_on_distance(pub Option < f32 >);



#[allow(unused_variables)]
#[allow(unused_mut)]
pub fn auto_setup(system: &mut ActorSystem) {
    LinkID::register_trait(system);
    AttacheeID::register_trait(system);
    RoughLocationID::register_trait(system);
    LocationRequesterID::register_trait(system);
    PositionRequesterID::register_trait(system);
    DistanceRequesterID::register_trait(system);
    
}