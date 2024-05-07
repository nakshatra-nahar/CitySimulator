use kay::{World, Fate, ActorSystem};
use compact::CVec;
use descartes::{
    N, P2, V2, Area, WithUniqueOrthogonal, ClosedLinePath, LinePath, AreaError, AreaEmbedding,
    AreaFilter,
};
use ordered_float::OrderedFloat;

use land_use::zone_planning::{Lot, BuildingIntent};
use land_use::buildings::BuildingStyle;
use land_use::buildings::architecture::ideal_lot_shape;
use economy::immigration_and_development::DevelopmentManagerID;
use itertools::Itertools;

use cb_planning::construction::{Constructable, ConstructableID};
use cb_planning::{Prototype, PrototypeID};
use planning::{CBConstructionID, CBPrototypeKind};
use cb_util::log::{debug, error};
const LOG_T: &str = "Vacant Lots";

#[derive(Compact, Clone)]
pub struct VacantLot {
    pub id: VacantLotID,
    pub lot: Lot,
    based_on: PrototypeID,
}

impl Lot {
    pub fn width_depth_per_road_connection(&self) -> Vec<(P2, V2, f32, f32)> {
        let midpoints = self
            .area
            .primitives
            .iter()
            .flat_map(|primitive| {
                primitive
                    .boundary
                    .path()
                    .segments()
                    .map(|segment| segment.midpoint())
            })
            .collect::<Vec<_>>();

        self.all_road_connections()
            .into_iter()
            .map(|(point, direction)| {
                let depth_direction = direction;
                let width_direction = depth_direction.orthogonal_right();

                let depth = midpoints
                    .iter()
                    .map(|midpoint| OrderedFloat((*midpoint - point).dot(&depth_direction)))
                    .minmax()
                    .into_option()
                    .map_or(0.0, |(front, back)| *back - *front);

                let width = midpoints
                    .iter()
                    .map(|midpoint| OrderedFloat((*midpoint - point).dot(&width_direction)))
                    .minmax()
                    .into_option()
                    .map_or(0.0, |(left, right)| *right - *left);

                (point, direction, width, depth)
            })
            .collect()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn split_for(
        &self,
        building_style: BuildingStyle,
        allow_left_split: bool,
        allow_right_split: bool,
        max_width_after_splitting: N,
        recursion_depth: usize,
        log_as: VacantLotID,
        log_in: &mut World,
    ) -> Result<Option<Lot>, AreaError> {
        let (needed_width, needed_depth, needed_compactness) = ideal_lot_shape(building_style);

        let debug_padding: String = ::std::iter::repeat(" ").take(recursion_depth).collect();
        debug(
            LOG_T,
            format!(
                "{}Trying to suggest lot for {:?}. Road Boundaries {:?}",
                debug_padding,
                building_style,
                self.road_boundaries
                    .iter()
                    .map(LinePath::length)
                    .collect::<Vec<_>>(),
            ),
            log_as,
            log_in,
        );

        // TODO: fix fucked up orientation of areas compared to descartes!!
        let compactness = -self.area.primitives[0].area() * 4.0 * ::std::f32::consts::PI
            / self.area.primitives[0].boundary.path().length().powi(2);

        for (point, direction, width, depth) in self.width_depth_per_road_connection() {
            debug(
                LOG_T,
                format!(
                    "{}Is: {:?} Needed: {:?}",
                    debug_padding,
                    (width, depth, compactness),
                    (needed_width, needed_depth, needed_compactness)
                ),
                log_as,
                log_in,
            );

            // make sure that we're making progress after recursing
            if width < max_width_after_splitting {
                let width_ratio = width / needed_width;
                let depth_ratio = depth / needed_depth;

                if width_ratio > 0.5 && width_ratio < 2.0 && depth_ratio > 0.5 && depth_ratio < 2.0
                {
                    if compactness >= needed_compactness {
                        return Ok(Some(self.clone()));
                    } else {
                        return Ok(None);
                    }
                } else if width_ratio > 2.0 && depth_ratio > 0.5 {
                    let orthogonal = direction.orthogonal_right();

                    let corners = vec![
                        point + needed_depth * direction,
                        point + needed_depth * direction + 1_000.0 * orthogonal,
                        point - 500.0 * direction + 1_000.0 * orthogonal,
                        point - 500.0 * direction,
                        point + needed_depth * direction,
                    ];

                    let splitter_area = Area::new_simple(
                        ClosedLinePath::new(LinePath::new(corners.into()).unwrap()).unwrap(),
                    );

                    debug(
                        LOG_T,
                        format!("{}Attempting width split", debug_padding),
                        log_as,
                        log_in,
                    );

                    #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
                    enum SplittingLabel {
                        Lot,
                        Splitter,
                    };

                    let mut embedding = AreaEmbedding::new(width / 10.0);

                    embedding.insert(self.area.clone(), SplittingLabel::Lot);
                    embedding.insert(splitter_area, SplittingLabel::Splitter);

                    if allow_right_split {
                        if let Ok(right_splits) = embedding
                            .view(
                                AreaFilter::has(SplittingLabel::Lot)
                                    .and(AreaFilter::has(SplittingLabel::Splitter)),
                            )
                            .get_areas_with_pieces()
                        {
                            for (right_split, _) in right_splits {
                                let road_boundaries = new_road_boundaries(
                                    &self.road_boundaries,
                                    right_split.primitives[0].boundary.path(),
                                );

                                if !road_boundaries.is_empty() {
                                    let split_lot = Lot {
                                        road_boundaries,
                                        area: right_split.clone(),
                                        original_area: right_split,
                                        ..self.clone()
                                    };

                                    // recurse!
                                    debug(
                                        LOG_T,
                                        format!(
                                            "{}Got right split lot, checking suitability",
                                            debug_padding,
                                        ),
                                        log_as,
                                        log_in,
                                    );
                                    if let Some(ok_split_lot) = split_lot.split_for(
                                        building_style,
                                        true, //false,
                                        true,
                                        width * 0.66,
                                        recursion_depth + 1,
                                        log_as,
                                        log_in,
                                    )? {
                                        return Ok(Some(ok_split_lot));
                                    }
                                }
                            }
                        }
                    }
                    if allow_left_split {
                        if let Ok(left_splits) = embedding
                            .view(
                                AreaFilter::has(SplittingLabel::Lot)
                                    .and(AreaFilter::has(SplittingLabel::Splitter).not()),
                            )
                            .get_areas_with_pieces()
                        {
                            for (left_split, _) in left_splits {
                                let road_boundaries = new_road_boundaries(
                                    &self.road_boundaries,
                                    left_split.primitives[0].boundary.path(),
                                );

                                if !road_boundaries.is_empty() {
                                    let split_lot = Lot {
                                        road_boundaries,
                                        original_area: left_split.clone(),
                                        area: left_split,
                                        ..self.clone()
                                    };

                                    // recurse!
                                    debug(
                                        LOG_T,
                                        format!(
                                            "{}Got right split lot, checking suitability",
                                            debug_padding,
                                        ),
                                        log_as,
                                        log_in,
                                    );
                                    if let Some(ok_split_lot) = split_lot.split_for(
                                        building_style,
                                        true,
                                        true, //false,
                                        width * 0.66,
                                        recursion_depth + 1,
                                        log_as,
                                        log_in,
                                    )? {
                                        return Ok(Some(ok_split_lot));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(None)
    }
}

fn new_road_boundaries(old_road_bundaries: &[LinePath], new_boundary: &LinePath) -> CVec<LinePath> {
    new_boundary
        .points
        .iter()
        .batching(|points_it| {
            let mut consecutive_points = CVec::new();

            loop {
                if let Some(point) = points_it.next() {
                    if old_road_bundaries
                        .iter()
                        .any(|old_boundary| old_boundary.includes(*point))
                    {
                        consecutive_points.push(*point);
                    } else {
                        break;
                    }
                } else if consecutive_points.is_empty() {
                    return None;
                } else {
                    break;
                }
            }

            Some(LinePath::new(consecutive_points))
        })
        .filter_map(|maybe_new_boundary| maybe_new_boundary)
        .collect()
}

impl VacantLot {
    pub fn spawn(
        id: VacantLotID,
        lot: &Lot,
        based_on: PrototypeID,
        _world: &mut World,
    ) -> VacantLot {
        VacantLot {
            id,
            based_on,
            lot: lot.clone(),
        }
    }

    pub fn suggest_lot(
        &mut self,
        building_style: BuildingStyle,
        requester: DevelopmentManagerID,
        world: &mut World,
    ) {
        if self
            .lot
            .zone_configs
            .iter()
            .any(|intent| building_style.can_appear_in(intent.land_use))
        {
            debug(LOG_T, "Trying suggest", self.id, world);
            match self.lot.split_for(
                building_style,
                true,
                true,
                ::std::f32::INFINITY,
                0,
                self.id,
                world,
            ) {
                Ok(Some(suitable_lot)) => requester.on_suggested_lot(
                    BuildingIntent {
                        lot: suitable_lot,
                        building_style,
                    },
                    self.based_on,
                    world,
                ),
                Ok(None) => {}
                Err(_err) => error(LOG_T, "Geometry", self.id, world),
            }
        }
    }
}

impl Constructable<CBPrototypeKind> for VacantLot {
    fn morph(
        &mut self,
        _: &Prototype<CBPrototypeKind>,
        _report_to: CBConstructionID,
        _world: &mut World,
    ) {
        unreachable!()
    }

    fn destruct(&mut self, report_to: CBConstructionID, world: &mut World) -> Fate {
        report_to.action_done(self.id.into(), world);
        Fate::Die
    }
}

pub fn setup(system: &mut ActorSystem) {
    system.register::<VacantLot>();
    auto_setup(system);
}

mod kay_auto;
pub use self::kay_auto::*;
