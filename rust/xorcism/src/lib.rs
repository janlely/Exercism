// totally copied from whjpji's solution
use std::iter::Cycle;
use std::slice::Iter;
use std::borrow::Borrow;

/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    _key: Cycle<Iter<'a, u8>>,
}

/// For composability, it is important that `munge` returns an iterator compatible with its input.
///
/// However, `impl Trait` syntax can specify only a single non-auto trait.
/// Therefore, we define this output trait with generic implementations on all compatible types,
/// and return that instead.
pub trait MungeOutput<'a>: Iterator<Item = u8> + ExactSizeIterator {}
impl<'a, T> MungeOutput<'a> for T where T: Iterator<Item = u8> + ExactSizeIterator {}

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<Key: AsRef<[u8]> + ?Sized>(key: &'a Key) -> Xorcism<'a> {
        if key.as_ref().is_empty() {
            panic!("key should not be empty!");
        }
        Xorcism { _key : key.as_ref().iter().cycle() }
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        data.iter_mut().for_each(|b| *b ^= self.next_key() )
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    pub fn munge<'b, Data>(&'b mut self, data: Data) -> impl MungeOutput<'a> + 'b
        where
            Data : 'b + IntoIterator,
            Data::Item : Borrow<u8>,
            Data::IntoIter : ExactSizeIterator,
    {
        data.into_iter().map(move |b| b.borrow() ^ self.next_key())
    }

    fn next_key(&mut self) -> u8 {
        *self._key.next().unwrap()
    }
}