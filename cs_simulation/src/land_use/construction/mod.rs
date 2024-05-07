use kay::World;
use compact::CVec;
use descartes::PointContainer;
use land_use::zone_planning::{LotPrototype, LotOccupancy};
use land_use::vacant_lots::VacantLotID;
use land_use::buildings::BuildingID;
use cb_planning::construction::ConstructableID;
use cb_planning::PrototypeID;
use planning::{CBConstructionID, CBPrototypeKind};

impl LotPrototype {
    pub fn construct(
        &self,
        self_id: PrototypeID,
        report_to: CBConstructionID,
        world: &mut World,
    ) -> CVec<ConstructableID<CBPrototypeKind>> {
        let id = match self.occupancy {
            LotOccupancy::Vacant => VacantLotID::spawn(self.lot.clone(), self_id, world).into(),
            LotOccupancy::Occupied(building_style) => {
                BuildingID::spawn(building_style, self.lot.clone(), world).into()
            }
        };
        report_to.action_done(id, world);
        vec![id].into()
    }

    pub fn morphable_from(&self, other: &LotPrototype) -> bool {
        // TODO: improve this
        (self.occupancy != LotOccupancy::Vacant)
            && (other.occupancy != LotOccupancy::Vacant)
            && other.lot.area.contains(self.lot.center_point())
    }
}
