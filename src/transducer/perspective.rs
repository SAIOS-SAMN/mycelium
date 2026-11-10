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
//! axis index.
//!
//! **A shift of perspective is a motion of the sensor** (track 26 S6). A
//! perspective holds its POSE, the signed permutation of the sample's
//! phases the sensor reads through, beside its roles; the roles are those
//! of the axes as read. A shift ([`SensorMotion`]) moves the pose and the
//! roles with it, or recasts one read axis to another role. The sample is
//! never rewritten: reading it under a perspective ([`Perspective::read`],
//! [`Perspective::frame`]) is a reading, and the posted sample, its content
//! and its address stay as the source gave them. Several perspectives of one
//! sample are a cover ([`cover`]).
//!
//! The planes, the compartment's cube and the frame's coordinate that the
//! readers of the base used to fix as constants (`GRID_AXES`, `GRID_PLANE`,
//! `FRAME_AXIS`, `PLANE_ROW`/`PLANE_COL`) are read here, from the rank.

pub mod cover;

use crate::transducer::family::transduce::{Enclosed, Enclosure, SignedPermutation, TransductionRefusal};
use crate::transducer::sample::Sample;

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

/// A SHIFT OF PERSPECTIVE: a motion of the sensor, never a rewrite of the
/// data (MU-SENSING §4).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SensorMotion {
    /// The sensor turns or mirrors: read axis `j` of the shifted perspective
    /// is read axis `σ(j)` of the perspective before, in its sense, and its
    /// role travels with it (one of the signed permutations of the phases).
    Pose(SignedPermutation),
    /// Read axis `axis` is read in `role`: a space axis read as the
    /// sequence a motion runs along, or as a value, or back.
    Recast { axis: usize, role: Role },
}

/// WHY A PERSPECTIVE IS NOT TAKEN OR NOT READ, named.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PerspectiveRefusal {
    /// The perspective reads `perspective` axes and the sample or motion
    /// presents `other`: a section is never padded or cut (MU-SENSING §5).
    Rank { perspective: usize, other: usize },
    /// A recast names an axis the perspective does not read.
    Axis { axis: usize, rank: usize },
    /// The pose's reading of the sample is refused by the transduction's
    /// own name.
    Read(TransductionRefusal),
}

/// AN ASSIGNMENT OF ROLES, one per axis of the sites it reads, in the order
/// the sensor reads them, and the POSE it reads them through: read axis `j`
/// is the sample's axis `pose(j)`, in its sense. The default pose is the
/// identity, so read order is canonical axis order, which writes addresses
/// alone and is never a role.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Perspective {
    roles: Vec<Role>,
    pose: SignedPermutation,
}

impl Perspective {
    /// A perspective from its roles, axis by axis, read in the sample's own
    /// pose.
    pub fn new(roles: Vec<Role>) -> Perspective {
        let pose = SignedPermutation::identity(roles.len());
        Perspective { roles, pose }
    }

    /// A PERSPECTIVE FROM ITS ROLES AND ITS POSE, or `Rank` where the pose
    /// reads another count of axes than the roles name.
    pub fn posed(roles: Vec<Role>, pose: SignedPermutation) -> Result<Perspective, PerspectiveRefusal> {
        match pose.rank() == roles.len() {
            true => Ok(Perspective { roles, pose }),
            false => Err(PerspectiveRefusal::Rank { perspective: roles.len(), other: pose.rank() }),
        }
    }

    /// THE DEFAULT PERSPECTIVE OF A SAMPLE OF RANK `rank`: every axis is
    /// space, read in the sample's own pose (ROADMAP track 26 S4b).
    pub fn of_sample(rank: usize) -> Perspective {
        Perspective::new(vec![Role::Space; rank])
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
        Perspective::new(roles)
    }

    /// The count of axes the perspective reads.
    pub fn rank(&self) -> usize {
        self.roles.len()
    }

    /// The roles, read axis by read axis.
    pub fn roles(&self) -> &[Role] {
        &self.roles
    }

    /// The pose: read axis `j` reads the sample's axis `pose(j)` in its
    /// sense.
    pub fn pose(&self) -> &SignedPermutation {
        &self.pose
    }

    /// The role of read axis `axis`, or `None` for an axis the perspective
    /// does not read: unread, never space by default (MU-SENSING §5).
    pub fn role(&self, axis: usize) -> Option<Role> {
        self.roles.get(axis).copied()
    }

    /// THE ROLE THE SAMPLE'S OWN AXIS `axis` IS READ IN: the role of the read
    /// axis the pose reads it as. `None` past the rank.
    pub fn role_of_sample_axis(&self, axis: usize) -> Option<Role> {
        let read = self.pose.images().iter().position(|&(k, _)| k == axis)?;
        self.role(read)
    }

    /// The read axes read as space, ascending.
    pub fn space_axes(&self) -> Vec<usize> {
        self.axes_of(Role::Space)
    }

    /// The read axes of one role, ascending.
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
    /// MU-ALPHABET §2): the first two space axes in read order, the plane
    /// the alphabet's letters are read on. `None` where fewer than two axes
    /// are space: a sample of one space axis has no plane to turn in, and
    /// none is invented for it.
    pub fn plane(&self) -> Option<(usize, usize)> {
        match self.space_axes().as_slice() {
            &[a, b, ..] => Some((a, b)),
            _ => None,
        }
    }

    /// THE PERSPECTIVE SHIFTED BY A MOTION OF THE SENSOR (MU-SENSING §4):
    /// under `Pose(σ)` the pose becomes `pose ∘ σ` and read axis `j` takes
    /// the role read axis `σ(j)` had; under `Recast` one read axis takes a
    /// role. Refused by name where the motion's rank or axis is not the
    /// perspective's. The sample is not touched: only the sensor moves.
    pub fn shifted(&self, motion: &SensorMotion) -> Result<Perspective, PerspectiveRefusal> {
        match motion {
            SensorMotion::Pose(sigma) => {
                if sigma.rank() != self.rank() {
                    return Err(PerspectiveRefusal::Rank { perspective: self.rank(), other: sigma.rank() });
                }
                let pose = self.pose.then(sigma).map_err(PerspectiveRefusal::Read)?;
                let roles = sigma.images().iter().filter_map(|&(k, _)| self.role(k)).collect();
                Perspective::posed(roles, pose)
            }
            SensorMotion::Recast { axis, role } => {
                let mut roles = self.roles.clone();
                let slot = roles.get_mut(*axis).ok_or(PerspectiveRefusal::Axis { axis: *axis, rank: self.rank() })?;
                *slot = *role;
                Perspective::posed(roles, self.pose.clone())
            }
        }
    }

    /// THE SENSOR'S TURN FROM THIS PERSPECTIVE TO `other`: the signed
    /// permutation `σ = pose⁻¹ ∘ pose′` with `self.pose ∘ σ = other.pose`.
    /// `Rank` where the two read another count of axes.
    pub fn turn_to(&self, other: &Perspective) -> Result<SignedPermutation, PerspectiveRefusal> {
        if other.rank() != self.rank() {
            return Err(PerspectiveRefusal::Rank { perspective: self.rank(), other: other.rank() });
        }
        self.pose.inverse().then(&other.pose).map_err(PerspectiveRefusal::Read)
    }

    /// THE SAMPLE AS THIS PERSPECTIVE READS IT: its frame moved by the pose
    /// and spoken onto the moved enclosure's own periods, which always
    /// land. A reading; the sample handed in is unchanged, its place kept.
    /// `Rank` where the sample's rank is not the perspective's.
    pub fn read(&self, sample: &Sample) -> Result<Sample, PerspectiveRefusal> {
        if sample.rank() != self.rank() {
            return Err(PerspectiveRefusal::Rank { perspective: self.rank(), other: sample.rank() });
        }
        let own = self.pose.enclosure(&Enclosure::of(sample)).map_err(PerspectiveRefusal::Read)?;
        self.pose.spoken_onto(&Enclosed::of(sample), &own).map_err(PerspectiveRefusal::Read)
    }

    /// THE FRAME THIS PERSPECTIVE READS: [`Perspective::read`] lifted, its
    /// bands and bodies read again in the moved enclosure.
    pub fn frame(&self, sample: &Sample) -> Result<Enclosed, PerspectiveRefusal> {
        Ok(Enclosed::of(&self.read(sample)?))
    }

    /// THE PERSPECTIVE WITH ITS AXES RELABELLED by `order`: output axis `j`
    /// takes the role of axis `order[j]`, the pose shifted by the
    /// permutation that keeps every sense. `None` where `order` is not a
    /// permutation of the axes. Relabelling the axes moves the roles with
    /// them, so a percept read under the relabelled perspective is the
    /// original permuted exactly (MU-SENSING §3.6).
    #[cfg(test)]
    pub fn relabelled(&self, order: &[usize]) -> Option<Perspective> {
        use crate::transducer::family::transduce::Sense;
        let sigma = SignedPermutation::new(order.iter().map(|&k| (k, Sense::Kept)).collect()).ok()?;
        self.shifted(&SensorMotion::Pose(sigma)).ok()
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
#[cfg(test)]
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
