use std::convert::From;

use binding::array;
use types::Value;

use super::any_object::AnyObject;
use super::string::RString;
use super::traits::Object;

/// `Array`
pub struct Array {
    value: Value,
}

impl Array {
    /// Creates a new instance of empty `Array`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Array, VM};
    /// # VM::init();
    ///
    /// Array::new();
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// []
    /// ```
    pub fn new() -> Self {
        Array { value: array::new() }
    }

    /// Retrieves an `AnyObject` from element at `index` position.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Array, Fixnum, VM};
    /// # VM::init();
    ///
    /// let array = Array::new().push(Fixnum::new(1));
    ///
    /// assert_eq!(array.at(0).to::<Fixnum>(), Fixnum::new(1));
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// array = [1]
    ///
    /// array[0] == 1
    /// ```
    pub fn at(&self, index: i64) -> AnyObject {
        let result = array::entry(self.value(), index);

        AnyObject::from(result)
    }

    /// Joins all elements of `Array` to Ruby `String`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Array, RString, VM};
    /// # VM::init();
    ///
    /// let array =
    ///     Array::new()
    ///         .push(RString::new("Hello"))
    ///         .push(RString::new("World!"));
    ///
    /// let joined_string = array.join(RString::new(", ")).to_string();
    ///
    /// assert_eq!(joined_string, "Hello, World!".to_string());
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// array = ['Hello', 'World!']
    ///
    /// array.join(', ') == 'Hello, World!'
    /// ```
    pub fn join(&self, separator: RString) -> RString {
        let result = array::join(self.value(), separator.value());

        RString::from(result)
    }

    /// Pushes an object to `Array`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Array, Fixnum, VM};
    /// # VM::init();
    ///
    /// let mut array = Array::new();
    ///
    /// array.push(Fixnum::new(1));
    ///
    /// assert_eq!(array.at(0).to::<Fixnum>(), Fixnum::new(1));
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// array = []
    /// array << 1
    ///
    /// array[0] == 1
    /// ```
    pub fn push<T: Object>(&mut self, item: T) -> Self {
        let result = array::push(self.value(), item.value());

        Array::from(result)
    }

    /// Stores an object at `index` position.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Array, Fixnum, VM};
    /// # VM::init();
    ///
    /// let mut array = Array::new().push(Fixnum::new(1));
    ///
    /// array.store(0, Fixnum::new(2));
    ///
    /// assert_eq!(array.at(0).to::<Fixnum>(), Fixnum::new(2));
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// array = [1]
    /// array[0] = 2
    ///
    /// array[0] == 2
    /// ```
    pub fn store<T: Object>(&mut self, index: i64, item: T) -> AnyObject {
        let result = array::store(self.value(), index, item.value());

        AnyObject::from(result)
    }
}

impl From<Value> for Array {
    fn from(value: Value) -> Self {
        Array { value: value }
    }
}

impl Object for Array {
    fn value(&self) -> Value {
        self.value
    }
}
