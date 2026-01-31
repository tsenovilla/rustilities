// SPDX-License-Identifier: GPL-3.0

#[cfg(test)]
mod tests;

use core::marker::PhantomData;

pub struct UniversalTestBuilder<State> {
	_state_mask: u128,
	_marker: PhantomData<State>,
}

pub struct UniversalTestBuilderInit;
impl Default for UniversalTestBuilder<UniversalTestBuilderInit> {
	fn default() -> Self {
		Self { _state_mask: 0, _marker: PhantomData }
	}
}
