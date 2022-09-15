use winsafe::co;
use winsafe::co::{DMDO};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum RotationType {
    ROT0,
    ROT90,
    ROT180,
    ROT270,
}

impl From<co::DMDO> for RotationType {
    fn from(win_rot: co::DMDO) -> Self {
        match win_rot {
            DMDO::DEFAULT => RotationType::ROT0,
            DMDO::D90 => RotationType::ROT90,
            DMDO::D180 => RotationType::ROT180,
            DMDO::D270 => RotationType::ROT270,
            _ => RotationType::ROT0
        }
    }
}

impl Into<co::DMDO> for RotationType {
    fn into(self) -> co::DMDO {
        match self {
            RotationType::ROT0 => DMDO::DEFAULT,
            RotationType::ROT90 => DMDO::D90,
            RotationType::ROT180 => DMDO::D180,
            RotationType::ROT270 => DMDO::D270,
        }
    }
}
