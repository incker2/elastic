use std::marker::PhantomData;
use serde::{ Serialize, Deserialize, Serializer, Deserializer };
use super::mapping::*;
use ::mapping::{ ElasticType, ElasticTypeMapping };

macro_rules! number_type {
    ($t:ident, $m:ident, $n:ident) => (
    	/// Number type with a given mapping.
    	#[derive(Debug, Default, Clone)]
		pub struct $t<M: ElasticTypeMapping<()> + $m> {
			value: $n,
			phantom: PhantomData<M>
		}
		impl <M: ElasticTypeMapping<()> + $m> $t<M> {
			/// Creates a new `$t` with the given mapping.
			pub fn new<I: Into<$n>>(num: I) -> $t<M> {
				$t {
					value: num.into(),
					phantom: PhantomData
				}
			}

			/// Get the value of the number.
			pub fn get(&self) -> $n {
				self.value
			}

			/// Set the value of the number.
			pub fn set<I: Into<$n>>(&mut self, num: I) {
				self.value = num.into()
			}

			/// Change the mapping of this number.
			pub fn into<MInto: ElasticTypeMapping<()> + $m>(self) -> $t<MInto> {
				$t::<MInto>::new(self.value)
			}
		}

		impl <M: ElasticTypeMapping<()> + $m> ElasticType<M, ()> for $t<M> { }

		impl <M: ElasticTypeMapping<()> + $m> From<$n> for $t<M> {
			fn from(num: $n) -> Self {
				$t::<M>::new(num)
			}
		}

		//Serialize elastic number
		impl <M: ElasticTypeMapping<()> + $m> Serialize for $t<M> {
			fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: Serializer {
				self.value.serialize(serializer)
			}
		}

		//Deserialize elastic number
		impl <M: ElasticTypeMapping<()> + $m> Deserialize for $t<M> {
			fn deserialize<D>(deserializer: &mut D) -> Result<$t<M>, D::Error> where D: Deserializer {
				let t = try!($n::deserialize(deserializer));

				Ok($t::<M>::new(t))
			}
		}
    )
}

number_type!(ElasticInteger, ElasticIntegerMapping, i32);

number_type!(ElasticLong, ElasticLongMapping, i64);

number_type!(ElasticShort, ElasticShortMapping, i16);

number_type!(ElasticByte, ElasticByteMapping, i8);

number_type!(ElasticFloat, ElasticFloatMapping, f32);

number_type!(ElasticDouble, ElasticDoubleMapping, f64);