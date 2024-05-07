//! This is all auto-generated. Do not touch.
#![rustfmt::skip]
#[allow(unused_imports)]
use kay::{ActorSystem, TypedID, RawID, Fate, Actor, TraitIDFrom, ActorOrActorTrait};
#[allow(unused_imports)]
use super::*;

#[derive(Serialize, Deserialize)] #[serde(transparent)]
pub struct LogRecipientID {
    _raw_id: RawID
}

impl Copy for LogRecipientID {}
impl Clone for LogRecipientID { fn clone(&self) -> Self { *self } }
impl ::std::fmt::Debug for LogRecipientID {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "LogRecipientID({:?})", self._raw_id)
    }
}
impl ::std::hash::Hash for LogRecipientID {
    fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
        self._raw_id.hash(state);
    }
}
impl PartialEq for LogRecipientID {
    fn eq(&self, other: &LogRecipientID) -> bool {
        self._raw_id == other._raw_id
    }
}
impl Eq for LogRecipientID {}

pub struct LogRecipientRepresentative;

impl ActorOrActorTrait for LogRecipientRepresentative {
    type ID = LogRecipientID;
}

impl TypedID for LogRecipientID {
    type Target = LogRecipientRepresentative;

    fn from_raw(id: RawID) -> Self {
        LogRecipientID { _raw_id: id }
    }

    fn as_raw(&self) -> RawID {
        self._raw_id
    }
}

impl<Act: Actor + LogRecipient> TraitIDFrom<Act> for LogRecipientID {}

impl LogRecipientID {
    pub fn receive_newest_logs(self, entries: CVec < Entry >, text: CString, effective_last: u32, effective_text_start: u32, world: &mut World) {
        world.send(self.as_raw(), MSG_LogRecipient_receive_newest_logs(entries, text, effective_last, effective_text_start));
    }

    pub fn register_trait(system: &mut ActorSystem) {
        system.register_trait::<LogRecipientRepresentative>();
        system.register_trait_message::<MSG_LogRecipient_receive_newest_logs>();
    }

    pub fn register_implementor<Act: Actor + LogRecipient>(system: &mut ActorSystem) {
        system.register_implementor::<Act, LogRecipientRepresentative>();
        system.add_handler::<Act, _, _>(
            |&MSG_LogRecipient_receive_newest_logs(ref entries, ref text, effective_last, effective_text_start), instance, world| {
                instance.receive_newest_logs(entries, text, effective_last, effective_text_start, world); Fate::Live
            }, false
        );
    }
}

#[derive(Compact, Clone)] #[allow(non_camel_case_types)]
struct MSG_LogRecipient_receive_newest_logs(pub CVec < Entry >, pub CString, pub u32, pub u32);

impl Actor for Log {
    type ID = LogID;

    fn id(&self) -> Self::ID {
        self.id
    }
    unsafe fn set_id(&mut self, id: RawID) {
        self.id = Self::ID::from_raw(id);
    }
}

#[derive(Serialize, Deserialize)] #[serde(transparent)]
pub struct LogID {
    _raw_id: RawID
}

impl Copy for LogID {}
impl Clone for LogID { fn clone(&self) -> Self { *self } }
impl ::std::fmt::Debug for LogID {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "LogID({:?})", self._raw_id)
    }
}
impl ::std::hash::Hash for LogID {
    fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
        self._raw_id.hash(state);
    }
}
impl PartialEq for LogID {
    fn eq(&self, other: &LogID) -> bool {
        self._raw_id == other._raw_id
    }
}
impl Eq for LogID {}

impl TypedID for LogID {
    type Target = Log;

    fn from_raw(id: RawID) -> Self {
        LogID { _raw_id: id }
    }

    fn as_raw(&self) -> RawID {
        self._raw_id
    }
}

impl LogID {
    pub fn spawn(world: &mut World) -> Self {
        let id = LogID::from_raw(world.allocate_instance_id::<Log>());
        let swarm = world.local_broadcast::<Log>();
        world.send(swarm, MSG_Log_spawn(id, ));
        id
    }
    
    pub fn log(self, topic: CString, message: CString, from: Option < RawID >, level: LogLevel, world: &mut World) {
        world.send(self.as_raw(), MSG_Log_log(topic, message, from, level));
    }
    
    pub fn get_after(self, last_known: u32, max_diff: u32, recipient: LogRecipientID, world: &mut World) {
        world.send(self.as_raw(), MSG_Log_get_after(last_known, max_diff, recipient));
    }
}

#[derive(Copy, Clone)] #[allow(non_camel_case_types)]
struct MSG_Log_spawn(pub LogID, );
#[derive(Compact, Clone)] #[allow(non_camel_case_types)]
struct MSG_Log_log(pub CString, pub CString, pub Option < RawID >, pub LogLevel);
#[derive(Compact, Clone)] #[allow(non_camel_case_types)]
struct MSG_Log_get_after(pub u32, pub u32, pub LogRecipientID);


#[allow(unused_variables)]
#[allow(unused_mut)]
pub fn auto_setup(system: &mut ActorSystem) {
    LogRecipientID::register_trait(system);
    
    system.add_spawner::<Log, _, _>(
        |&MSG_Log_spawn(id, ), world| {
            Log::spawn(id, world)
        }, false
    );
    
    system.add_handler::<Log, _, _>(
        |&MSG_Log_log(ref topic, ref message, from, level), instance, world| {
            instance.log(topic, message, from, level, world); Fate::Live
        }, false
    );
    
    system.add_handler::<Log, _, _>(
        |&MSG_Log_get_after(last_known, max_diff, recipient), instance, world| {
            instance.get_after(last_known, max_diff, recipient, world); Fate::Live
        }, false
    );
}