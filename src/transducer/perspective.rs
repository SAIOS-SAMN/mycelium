// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! MU-SENSING §4 — THE PERSPECTIVE: the sensor's assignment of a sample's
//! axes, and of the sample's place in its family, to roles. No axis is
//! privileged by the code (MU-SENSING §3.6): which axes are space, which
//! one a motion runs along and which are values is a perspective's, read
//! after intake and never before addressing (MU-SENSING §6.1).
//!
//! The default perspective (ROADMAP track 26 S4b) reads every axis of a
//! sample as space. A site of a family's base carries the sample's axes and
//! then the sample's place in its family, the sequence the pair's motion
//! runs along: the place is a coordinate the family carries, never a fixed
//! axis index. S6 makes perspectives general (a sensor motion, a cover of
//! several perspectives); every reader here already takes its roles from a
//! perspective rather than from an axis number.
//!
//! The planes, the compartment's cube and the frame's coordinate that the
//! readers of the base used to fix as constants (`GRID_AXES`, `GRID_PLANE`,
//! `FRAME_AXIS`, `PLANE_ROW`/`PLANE_COL`) are read here, from the rank.

/// THE ROLE A PERSPECTIVE GIVES ONE AXIS (MU-SENSING §4).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    /// A phase the bodies extend along.
    Space,
    /// The axis a motion runs along: a sample's place in its family, or an
    /// axis a perspective reads as a sequence.
    Sequence,
    /// A fibre: the axis's coordinate is read as a value, never a place.
    Value,
}

/// AN AXIS OF A PERSPECTIVE'S PLANE (MU-SENSING §3.6, §4): the first or the
/// second of the two space axes [`Perspective::plane`] names, in the plane's
/// order. It is named by its place in the plane, never as a row or a column:
/// which axis of the sample it is, is the perspective's.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaneAxis {
    /// The plane's first space axis.
    First,
    /// The plane's second space axis.
    Second,
}

impl PlaneAxis {
    /// THE AXIS'S PLACE IN THE PLANE: the index of its coordinate in a site
    /// of the section, which is written in the order [`Perspective::plane`]
    /// names the plane's axes.
    pub fn index(self) -> usize {
        match self {
            PlaneAxis::First => 0,
            PlaneAxis::Second => 1,
        }
    }
}

/// AN ASSIGNMENT OF ROLES, one per axis of the sites it reads, in canonical
/// axis order. The order writes addresses alone and is never a role.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Perspective {
    roles: Vec<Role>,
}

impl Perspective {
    /// A perspective from its roles, axis by axis.
    pub fn new(roles: Vec<Role>) -> Perspective {
        Perspective { roles }
    }

    /// THE DEFAULT PERSPECTIVE OF A SAMPLE OF RANK `rank`: every axis is
    /// space (ROADMAP track 26 S4b).
    pub fn of_sample(rank: usize) -> Perspective {
        Perspective { roles: vec![Role::Space; rank] }
    }

    /// THE DEFAULT PERSPECTIVE OF A SITE OF A FAMILY'S BASE, of `n`
    /// coordinates: the sample's `n − 1` axes, each space, and then the
    /// sample's place in its family, the sequence. A base of no coordinate
    /// holds no role.
    pub fn of_family_site(n: usize) -> Perspective {
        let roles = match n.checked_sub(1) {
            Some(space) => {
                let mut roles = vec![Role::Space; space];
                roles.push(Role::Sequence);
                roles
            }
            None => Vec::new(),
        };
        Perspective { roles }
    }

    /// The count of axes the perspective reads.
    pub fn rank(&self) -> usize {
        self.roles.len()
    }

    /// The roles, axis by axis.
    pub fn roles(&self) -> &[Role] {
        &self.roles
    }

    /// The role of `axis`, or `None` for an axis the perspective does not
    /// read: unread, never space by default (MU-SENSING §5).
    pub fn role(&self, axis: usize) -> Option<Role> {
        self.roles.get(axis).copied()
    }

    /// The axes read as space, ascending.
    pub fn space_axes(&self) -> Vec<usize> {
        self.axes_of(Role::Space)
    }

    /// The axes of one role, ascending.
    pub fn axes_of(&self, role: Role) -> Vec<usize> {
        self.roles.iter().enumerate().filter(|(_, r)| **r == role).map(|(i, _)| i).collect()
    }

    /// THE SAMPLE'S PLACE IN ITS FAMILY, as an axis of the family's site:
    /// the one axis read as sequence. `None` where the perspective reads
    /// none, or reads several (no one of them is the place).
    pub fn place_axis(&self) -> Option<usize> {
        match self.axes_of(Role::Sequence).as_slice() {
            &[k] => Some(k),
            _ => None,
        }
    }

    /// THE SENSOR'S PLANE UNDER THIS PERSPECTIVE (MU-SENSING §9 on
    /// MU-ALPHABET §2): the first two space axes in canonical order, the
    /// plane the alphabet's letters are read on. `None` where fewer than two
    /// axes are space: a sample of one space axis has no plane to turn in,
    /// and none is invented for it.
    pub fn plane(&self) -> Option<(usize, usize)> {
        match self.space_axes().as_slice() {
            &[a, b, ..] => Some((a, b)),
            _ => None,
        }
    }

    /// THE PERSPECTIVE WITH ITS AXES RELABELLED by `order`: output axis `j`
    /// takes the role of axis `order[j]`. `None` where `order` is not a
    /// permutation of the axes. Relabelling the axes moves the roles with
    /// them, so a percept read under the relabelled perspective is the
    /// original permuted exactly (MU-SENSING §3.6).
    pub fn relabelled(&self, order: &[usize]) -> Option<Perspective> {
        let mut seen = vec![false; self.roles.len()];
        let mut roles = Vec::with_capacity(order.len());
        for &k in order {
            let slot = seen.get_mut(k)?;
            if *slot {
                return None;
            }
            *slot = true;
            roles.push(*self.roles.get(k)?);
        }
        (roles.len() == self.roles.len()).then_some(Perspective { roles })
    }
}

/// THE SENSOR PLANE OF A SITE OF THE FAMILY'S BASE OF `n` COORDINATES,
/// under the default perspective ([`Perspective::of_family_site`]).
pub fn family_plane(n: usize) -> Option<(usize, usize)> {
    Perspective::of_family_site(n).plane()
}

/// THE SENSOR PLANE OF THE ALPHABET'S BASE as two base axes (a base of
/// `alphabet::ALPHABET_BASE_N` coordinates under the default perspective):
/// the plane the letters' sensor slots stand for.
pub fn alphabet_base_plane() -> Option<(usize, usize)> {
    family_plane(crate::transducer::alphabet::ALPHABET_BASE_N)
}

/// THE PLACE AXIS OF A SITE OF THE FAMILY'S BASE OF `n` COORDINATES,
/// under the default perspective: the coordinate after the sample's axes.
pub fn family_place_axis(n: usize) -> Option<usize> {
    Perspective::of_family_site(n).place_axis()
}

/// THE ROLE OF `axis` ON A SITE OF THE FAMILY'S BASE OF `n` COORDINATES,
/// under the default perspective.
pub fn family_role(n: usize, axis: usize) -> Option<Role> {
    Perspective::of_family_site(n).role(axis)
}
