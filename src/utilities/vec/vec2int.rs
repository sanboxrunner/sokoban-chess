use super::{vec_iter::*, Axis, Vec2};
use std::fmt::{self, Display};

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct Vec2Int {
    pub x: i32,
    pub y: i32,
}

impl Vec2Int {
    pub fn new(x: i32, y: i32) -> Self {
        Vec2Int { x, y }
    }

    pub fn clamp_components(&mut self, min_vec: &Vec2Int, max_vec: &Vec2Int) {
        self.x = self.x.max(min_vec.x).min(max_vec.x);
        self.y = self.y.max(min_vec.y).min(max_vec.y);
    }

    pub fn cwise_product(&self, other_vec: Vec2Int) -> Vec2Int {
        let mut me: Vec2Int = self.clone();
        me.x *= other_vec.x;
        me.y *= other_vec.y;
        me
    }

    pub fn cwise_div(&self, other_vec: Vec2Int) -> Vec2Int {
        let mut me: Vec2Int = self.clone();
        me.x /= other_vec.x;
        me.y /= other_vec.y;
        me
    }

    pub fn has_zero_dimension(&self) -> bool {
        self.x == 0 || self.y == 0
    }

    /// 0 counts as being positive. The purpose of this
    /// function is normally to check if the components can be
    /// cast into unsigned ints safely. A true means they can,
    /// assuming precision allows for it.
    pub fn is_positive(&self) -> bool {
        self.x >= 0 && self.y >= 0
    }

    pub fn get_axis(&self, axis: Axis) -> i32 {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
        }
    }
}

impl Vec2Int {
    #[allow(dead_code)]
    pub const ZERO: Vec2Int = Vec2Int { x: 0, y: 0 };

    #[allow(dead_code)]
    pub const ONE: Vec2Int = Vec2Int { x: 1, y: 1 };

    #[allow(dead_code)]
    pub const UP: Vec2Int = Vec2Int { x: 0, y: 1 };

    #[allow(dead_code)]
    pub const RIGHT: Vec2Int = Vec2Int { x: 1, y: 0 };
}

impl Display for Vec2Int {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl std::ops::Add<Vec2Int> for Vec2Int {
    type Output = Vec2Int;

    fn add(self, rhs: Vec2Int) -> Vec2Int {
        Vec2Int {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign<Vec2Int> for Vec2Int {
    fn add_assign(&mut self, rhs: Vec2Int) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Sub<Vec2Int> for Vec2Int {
    type Output = Vec2Int;

    fn sub(self, rhs: Vec2Int) -> Vec2Int {
        Vec2Int {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::SubAssign<Vec2Int> for Vec2Int {
    fn sub_assign(&mut self, rhs: Vec2Int) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl std::ops::Div<i32> for Vec2Int {
    type Output = Vec2Int;

    fn div(self, rhs: i32) -> Vec2Int {
        Vec2Int {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl std::ops::DivAssign<i32> for Vec2Int {
    fn div_assign(&mut self, rhs: i32) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
    }
}

impl std::ops::Mul<i32> for Vec2Int {
    type Output = Vec2Int;

    fn mul(self, rhs: i32) -> Vec2Int {
        Vec2Int {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::MulAssign<i32> for Vec2Int {
    fn mul_assign(&mut self, rhs: i32) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
    }
}

impl From<[i32; 2]> for Vec2Int {
    fn from(w: [i32; 2]) -> Vec2Int {
        Vec2Int { x: w[0], y: w[1] }
    }
}

impl From<(usize, usize)> for Vec2Int {
    fn from(w: (usize, usize)) -> Vec2Int {
        Vec2Int {
            x: w.0 as i32,
            y: w.1 as i32,
        }
    }
}

impl From<Vec2Int> for [i32; 2] {
    fn from(w: Vec2Int) -> [i32; 2] {
        [w.x, w.y]
    }
}

impl From<Vec2> for Vec2Int {
    fn from(other: Vec2) -> Vec2Int {
        Vec2Int {
            x: other.x as i32,
            y: other.y as i32,
        }
    }
}

impl GameVec<i32> for Vec2Int {
    fn x(&self) -> &i32 {
        &self.x
    }

    fn y(&self) -> &i32 {
        &self.y
    }
}

impl Vec2Int {
    pub fn iter(&self) -> VecIter<'_, i32> {
        VecIter::new(self)
    }
}

use imgui;
impl Vec2Int {
    pub fn vec2int_inspector(&mut self, ui: &imgui::Ui<'_>, label: &imgui::ImStr) -> bool {
        let mut vec2_deconstructed = self.clone().into();

        if ui.input_int2(label, &mut vec2_deconstructed).build() {
            self.x = vec2_deconstructed[0];
            self.y = vec2_deconstructed[1];
            true
        } else {
            false
        }
    }

    pub fn vec2int_inspector_like_ints(
        &mut self,
        ui: &imgui::Ui<'_>,
        x_label: &imgui::ImStr,
        y_label: &imgui::ImStr,
    ) -> bool {
        let mut dirty = false;

        if ui.input_int(x_label, &mut self.x).build() {
            dirty = true;
        }

        if ui.input_int(y_label, &mut self.y).build() {
            dirty = true;
        }

        dirty
    }
}
