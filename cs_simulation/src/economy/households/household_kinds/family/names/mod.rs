use super::FamilyID;
use economy::households::MemberIdx;
use cb_util::random::{seed, Rng};

// names taken from https://github.com/icebob/fakerator

const FIRST_NAMES: [&str; 3007] = include!("first_names.txt");

#[rustfmt::skip]
const LAST_NAMES: [&str; 474] = include!("last_names.txt");

const PATREON_NAMES: [(&str, &str); 37] = include!("patron_names.txt");

pub fn family_name(id: FamilyID) -> &'static str {
    let mut rng = seed(id);
    if rng.gen_bool(0.1) {
        rng.choose(&PATREON_NAMES).unwrap().1
    } else {
        rng.choose(&LAST_NAMES).unwrap()
    }
}

pub fn member_name(id: FamilyID, member: MemberIdx) -> String {
    let mut family_rng = seed(id);
    let (first_name, last_name) = if family_rng.gen_bool(0.1) {
        let entry = family_rng.choose(&PATREON_NAMES).unwrap();
        if member.0 == 0 {
            *entry
        } else {
            let mut rng = seed((id, member.0));
            (*rng.choose(&FIRST_NAMES).unwrap(), entry.1)
        }
    } else {
        let mut rng = seed((id, member.0));
        (*rng.choose(&FIRST_NAMES).unwrap(), family_name(id))
    };
    format!("{} {}", first_name, last_name)
}
