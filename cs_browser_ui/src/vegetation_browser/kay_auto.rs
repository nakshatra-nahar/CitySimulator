//! This is all auto-generated. Do not touch.
#![rustfmt::skip]
#[allow(unused_imports)]
use kay::{ActorSystem, TypedID, RawID, Fate, Actor, TraitIDFrom, ActorOrActorTrait};
#[allow(unused_imports)]
use super::*;



impl Actor for BrowserVegetationUI {
    type ID = BrowserVegetationUIID;

    fn id(&self) -> Self::ID {
        self.id
    }
    unsafe fn set_id(&mut self, id: RawID) {
        self.id = Self::ID::from_raw(id);
    }
}

#[derive(Serialize, Deserialize)] #[serde(transparent)]
pub struct BrowserVegetationUIID {
    _raw_id: RawID
}

impl Copy for BrowserVegetationUIID {}
impl Clone for BrowserVegetationUIID { fn clone(&self) -> Self { *self } }
impl ::std::fmt::Debug for BrowserVegetationUIID {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "BrowserVegetationUIID({:?})", self._raw_id)
    }
}
impl ::std::hash::Hash for BrowserVegetationUIID {
    fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
        self._raw_id.hash(state);
    }
}
impl PartialEq for BrowserVegetationUIID {
    fn eq(&self, other: &BrowserVegetationUIID) -> bool {
        self._raw_id == other._raw_id
    }
}
impl Eq for BrowserVegetationUIID {}

impl TypedID for BrowserVegetationUIID {
    type Target = BrowserVegetationUI;

    fn from_raw(id: RawID) -> Self {
        BrowserVegetationUIID { _raw_id: id }
    }

    fn as_raw(&self) -> RawID {
        self._raw_id
    }
}

impl BrowserVegetationUIID {
    pub fn spawn(world: &mut World) -> Self {
        let id = BrowserVegetationUIID::from_raw(world.allocate_instance_id::<BrowserVegetationUI>());
        let swarm = world.local_broadcast::<BrowserVegetationUI>();
        world.send(swarm, MSG_BrowserVegetationUI_spawn(id, ));
        id
    }
}

#[derive(Copy, Clone)] #[allow(non_camel_case_types)]
struct MSG_BrowserVegetationUI_spawn(pub BrowserVegetationUIID, );

impl Into<FrameListenerID> for BrowserVegetationUIID {
    fn into(self) -> FrameListenerID {
        FrameListenerID::from_raw(self.as_raw())
    }
}

impl Into<VegetationUIID> for BrowserVegetationUIID {
    fn into(self) -> VegetationUIID {
        VegetationUIID::from_raw(self.as_raw())
    }
}

#[allow(unused_variables)]
#[allow(unused_mut)]
pub fn auto_setup(system: &mut ActorSystem) {
    
    FrameListenerID::register_implementor::<BrowserVegetationUI>(system);
    VegetationUIID::register_implementor::<BrowserVegetationUI>(system);
    system.add_spawner::<BrowserVegetationUI, _, _>(
        |&MSG_BrowserVegetationUI_spawn(id, ), world| {
            BrowserVegetationUI::spawn(id, world)
        }, false
    );
}