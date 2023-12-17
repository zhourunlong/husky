use dataset::MNIST_DATASET;
use husky_ml_task_prelude::input_id;
use husky_standard_value::{FromValue as __FromValue, IntoValue as __IntoValue, Value as __Value};

mod dataset;

#[husky_standard_value::value_conversion]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MnistLabel {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl From<u8> for MnistLabel {
    fn from(value: u8) -> Self {
        todo!()
    }
}

#[husky_standard_value::value_conversion]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BinaryImage28([u32; 30]);

impl BinaryImage28 {
    pub fn new_zeros() -> Self {
        Self::default()
    }
}

impl std::ops::Index<usize> for BinaryImage28 {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for BinaryImage28 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[husky_standard_value::value_conversion]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BinaryGrid28([u32; 31]);

impl BinaryGrid28 {
    pub fn new_zeros() -> Self {
        Self::default()
    }
}

impl std::ops::Index<usize> for BinaryGrid28 {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for BinaryGrid28 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl BinaryGrid28 {}

pub fn input() -> &'static BinaryImage28 {
    MNIST_DATASET.input(input_id())
}
