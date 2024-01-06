use std::ops::{BitAnd, BitAndAssign, BitOr};

#[derive(Copy, Clone)]
pub struct EffBool {
    actual_boolean: bool,
    is_changed: bool,
}

impl Default for EffBool {
    fn default() -> Self {
        Self {
            actual_boolean: false,
            is_changed: false,
        }
    }
}

impl EffBool {
    /// Construct EffBool with given boolean.
    pub fn new(b: bool) -> Self {
        EffBool {
            actual_boolean: b,
            is_changed: false,
        }
    }

    /// Get boolean.
    pub fn get(&self) -> bool {
        self.actual_boolean
    }

    /// Set boolean.
    pub fn set(&mut self, b: bool) {
        if b == self.actual_boolean {
            self.is_changed = false;
            return;
        }
        self.actual_boolean = b;
        self.is_changed = true;
    }

    /// Set boolean and returns ( is_changed ,  actualbool ) .
    /// Will set is_changed flag to false.
    /// If you want to just peek the is_changed flag and not modifying it, use peek_change_flag() function instead.
    pub fn set_and_is_changed(&mut self, b: bool) -> (bool, bool) {
        self.set(b);
        (self.is_changed(), self.actual_boolean)
    }

    /// If boolean value has changed with last set() or set_and_is_changed() call, returns true.
    /// Will set is_changed flag to false.
    /// If you want to just peek the is_changed flag and not modifying it, use peek_change_flag() function instead.
    pub fn is_changed(&mut self) -> bool {
        let ret = self.is_changed;
        self.is_changed = false; //clear
        ret
    }

    /// Clear the changed flag manually.
    pub fn clear_change_flag(&mut self) {
        self.is_changed = false;
    }

    /// Just returns is_changed flag.
    /// Will not clear the flag.
    /// You can clear the flag by calling clear_change_flag()
    pub fn peek_change_flag(&self) -> bool {
        self.is_changed
    }
}

impl BitAnd for EffBool {
    type Output = bool;
    fn bitand(self, rhs: Self) -> bool {
        self.actual_boolean & rhs.actual_boolean
    }
}

impl BitOr for EffBool {
    type Output = bool;
    fn bitor(self, rhs: Self) -> bool {
        self.actual_boolean | rhs.actual_boolean
    }
}
/*
impl BitAndAssign for EffBool {
    // `rhs` is the "right-hand side" of the expression `a &= b`.
    fn bitand_assign(&mut self, rhs: Self) {
        let rhs = rhs.get();
        let lhs = self.get();
        self.set(rhs & lhs);
    }
}
*/
#[cfg(test)]
mod tests {
    use super::EffBool;

    #[test]
    fn test() {
        //init test
        let mut a = EffBool::default();
        let b = EffBool::new(false);
        let c = EffBool::new(true);
        assert_eq!(a.get(), false);
        assert_eq!(b.get(), false);
        assert_eq!(c.get(), true);

        //modification test
        a.set(true);
        assert_eq!(a.get(), true);
        assert_eq!(a.is_changed(), true);
        assert_eq!(a.set_and_is_changed(false), (true, false));
        // a is false now, test is_changed
        assert_eq!(a.is_changed(), true);
        assert_eq!(a.set_and_is_changed(true), (true, true));
    }

    #[test]
    fn ops() {
        //test operations
        let mut tru = EffBool::new(true);
        let fal = EffBool::new(false);
        assert_eq!(tru & fal, false);
        assert_eq!(fal & fal, false);

        assert_eq!(tru | fal, true);
        assert_eq!(fal | fal, false);
    }
}
