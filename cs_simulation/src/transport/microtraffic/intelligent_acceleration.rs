use super::Obstacle;

pub const COMFORTABLE_BREAKING_DECELERATION: f32 = 0.4;

pub fn intelligent_acceleration(
    car: &Obstacle,
    obstacle: &Obstacle,
    safe_time_headway: f32,
) -> f32 {
    // http://en.wikipedia.org/wiki/Intelligent_driver_model

    let car_length = 4.0;
    let acceleration = 0.4;
    let max_deceleration: f32 = 5.0;
    let desired_velocity = car.max_velocity;
    let acceleration_exponent = 4.0;
    let minimum_spacing = 4.0;

    let net_distance = *obstacle.position - *car.position - car_length;
    let velocity_difference = car.velocity - obstacle.velocity;

    let s_star = minimum_spacing
        + 0.0f32.max(
            car.velocity * safe_time_headway
                + (car.velocity * velocity_difference
                    / (2.0 * (acceleration * COMFORTABLE_BREAKING_DECELERATION).sqrt())),
        );

    (-max_deceleration).max(
        acceleration
            * (1.0
                - (car.velocity / desired_velocity).powf(acceleration_exponent)
                - (s_star / net_distance).powf(2.0)),
    )
}
