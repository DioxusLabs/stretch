//! Style types for Flexbox layout
use crate::geometry::AbsoluteAxis;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

/// Controls whether flex items are forced onto one line or can wrap onto multiple lines.
///
/// Defaults to [`FlexWrap::NoWrap`]
///
/// [Specification](https://www.w3.org/TR/css-flexbox-1/#flex-wrap-property)
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub enum FlexWrap {
    /// Items will not wrap and stay on a single line
    NoWrap,
    /// Items will wrap according to this item's [`FlexDirection`]
    Wrap,
    /// Items will wrap in the opposite direction to this item's [`FlexDirection`]
    WrapReverse,
}

impl Default for FlexWrap {
    fn default() -> Self {
        Self::NoWrap
    }
}

impl TryFrom<i32> for FlexWrap {
    type Error = ();
    fn try_from(n: i32) -> Result<Self, ()> {
        match n {
            0 => Ok(FlexWrap::NoWrap),
            1 => Ok(FlexWrap::Wrap),
            2 => Ok(FlexWrap::WrapReverse),
            _ => Err(()),
        }
    }
}

/// The direction of the flexbox layout main axis.
///
/// There are always two perpendicular layout axes: main (or primary) and cross (or secondary).
/// Adding items will cause them to be positioned adjacent to each other along the main axis.
/// By varying this value throughout your tree, you can create complex axis-aligned layouts.
///
/// Items are always aligned relative to the cross axis, and justified relative to the main axis.
///
/// The default behavior is [`FlexDirection::Row`].
///
/// [Specification](https://www.w3.org/TR/css-flexbox-1/#flex-direction-property)
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub enum FlexDirection {
    /// Defines +x as the main axis
    ///
    /// Items will be added from left to right in a row.
    Row,
    /// Defines +y as the main axis
    ///
    /// Items will be added from top to bottom in a column.
    Column,
    /// Defines -x as the main axis
    ///
    /// Items will be added from right to left in a row.
    RowReverse,
    /// Defines -y as the main axis
    ///
    /// Items will be added from bottom to top in a column.
    ColumnReverse,
}

impl Default for FlexDirection {
    fn default() -> Self {
        Self::Row
    }
}

impl TryFrom<i32> for FlexDirection {
    type Error = ();
    fn try_from(n: i32) -> Result<Self, ()> {
        match n {
            0 => Ok(FlexDirection::Row),
            1 => Ok(FlexDirection::Column),
            2 => Ok(FlexDirection::RowReverse),
            3 => Ok(FlexDirection::ColumnReverse),
            _ => Err(()),
        }
    }
}

impl FlexDirection {
    #[inline]
    /// Is the direction [`FlexDirection::Row`] or [`FlexDirection::RowReverse`]?
    pub(crate) fn is_row(self) -> bool {
        matches!(self, Self::Row | Self::RowReverse)
    }

    #[inline]
    /// Is the direction [`FlexDirection::Column`] or [`FlexDirection::ColumnReverse`]?
    pub(crate) fn is_column(self) -> bool {
        matches!(self, Self::Column | Self::ColumnReverse)
    }

    #[inline]
    /// Is the direction [`FlexDirection::RowReverse`] or [`FlexDirection::ColumnReverse`]?
    pub(crate) fn is_reverse(self) -> bool {
        matches!(self, Self::RowReverse | Self::ColumnReverse)
    }

    #[inline]
    /// The `AbsoluteAxis` that corresponds to the main axis
    pub(crate) fn main_axis(self) -> AbsoluteAxis {
        match self {
            Self::Row | Self::RowReverse => AbsoluteAxis::Horizontal,
            Self::Column | Self::ColumnReverse => AbsoluteAxis::Vertical,
        }
    }

    #[inline]
    /// The `AbsoluteAxis` that corresponds to the cross axis
    pub(crate) fn cross_axis(self) -> AbsoluteAxis {
        match self {
            Self::Row | Self::RowReverse => AbsoluteAxis::Vertical,
            Self::Column | Self::ColumnReverse => AbsoluteAxis::Horizontal,
        }
    }
}

#[cfg(test)]
mod tests {
    mod test_flex_direction {
        use crate::style::*;

        #[test]
        fn flex_direction_is_row() {
            assert_eq!(FlexDirection::Row.is_row(), true);
            assert_eq!(FlexDirection::RowReverse.is_row(), true);
            assert_eq!(FlexDirection::Column.is_row(), false);
            assert_eq!(FlexDirection::ColumnReverse.is_row(), false);
        }

        #[test]
        fn flex_direction_is_column() {
            assert_eq!(FlexDirection::Row.is_column(), false);
            assert_eq!(FlexDirection::RowReverse.is_column(), false);
            assert_eq!(FlexDirection::Column.is_column(), true);
            assert_eq!(FlexDirection::ColumnReverse.is_column(), true);
        }

        #[test]
        fn flex_direction_is_reverse() {
            assert_eq!(FlexDirection::Row.is_reverse(), false);
            assert_eq!(FlexDirection::RowReverse.is_reverse(), true);
            assert_eq!(FlexDirection::Column.is_reverse(), false);
            assert_eq!(FlexDirection::ColumnReverse.is_reverse(), true);
        }
    }
}
