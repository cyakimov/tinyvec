/// A trait for types that can be the backing store of an
/// [`ArrayVec`](ArrayVec::<A>).
///
/// An "array", for our purposes, has the following basic properties:
/// * Owns some number of elements.
/// * The element type can be generic, but must implement [`Default`].
/// * The capacity is fixed based on the array type.
/// * You can get a shared or mutable slice to the elements.
///
/// You are generally note expected to need to implement this yourself. It is
/// already implemented for all the major array lengths. Additional lengths can
/// probably be added upon request.
///
/// ## Safety Reminder
///
/// As a reminder, this trait is 100% safe, which means that `unsafe` code
/// **must not** rely on an instance of the trait being correct to avoid UB.
pub trait Array {
  /// The type of the items in the thing.
  type Item: Default;

  /// The number of slots in the thing.
  const CAPACITY: usize;

  /// Gives a shared slice over the whole thing.
  ///
  /// A correct implementation will return a slice with a length equal to the
  /// `CAPACITY` value.
  fn slice(&self) -> &[Self::Item];

  /// Gives a unique slice over the whole thing.
  /// 
  /// A correct implementation will return a slice with a length equal to the
  /// `CAPACITY` value.
  fn slice_mut(&mut self) -> &mut [Self::Item];
}

macro_rules! impl_array_for_len {
  ($($len:expr),+ $(,)?) => {
    $(impl<T: Default> Array for [T; $len] {
      type Item = T;
      const CAPACITY: usize = $len;
      #[inline(always)]
      fn slice(&self) -> &[T] {
        &*self
      }
      #[inline(always)]
      fn slice_mut(&mut self) -> &mut [T] {
        &mut *self
      }
    })+
  }
}

impl_array_for_len! {
  0, /* The oft-forgotten 0-length array! */
  1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
  17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
  33, /* for luck */
  64, 128, 256, 512, 1024, 2048, 4096,
}
