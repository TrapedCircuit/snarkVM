// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use crate::Vec;
use snarkvm_fields::PrimeField;

use core::fmt::Debug;

/// Trait for an algebraic sponge.
pub trait AlgebraicSponge<BaseField: PrimeField>: Clone + Debug {
    type Parameters;

    fn sample_params() -> Self::Parameters;

    /// Initializes an algebraic sponge.
    fn new() -> Self;

    /// Initializes an algebraic sponge.
    fn with_parameters(params: &Self::Parameters) -> Self;

    /// Takes in field elements.
    fn absorb(&mut self, elems: &[BaseField]);
    /// Takes out field elements.
    fn squeeze(&mut self, num: usize) -> Vec<BaseField>;
}
