/// Create a new `BDAddr` with the specified address values.
/// The macro expects exactly 6 values.
///
/// # Example
///
/// ```
/// let addr = bd_addr!(0, 0, 0, 0, 0, 0);
/// ```
#[macro_export]
macro_rules! bd_addr {
    ($($e:expr),*) => {
        {
            let arr = [$($e),*];
            assert_eq!(arr.len(), 6, "BDAddr requires exactly 6 values");
            BDAddr {
                address: arr,
            }
        }
    };
}

/// Create a new `Uuid` from a 128-bit integer.
///
/// # Example
///
/// ```
/// let id = uuid!(12345678901234567890123456789012);
/// ```
#[macro_export]
macro_rules! uuid {
    ($val:expr) => {
        {
            assert!($val <= u128::MAX, "Uuid value cannot be larger than a 128-bit integer");
            Uuid::from_u128($val)
        }
    };
}

/// Create a new `Characteristic` with the specified UUID, service UUID, and properties.
///
/// # Example
///
/// ```
/// let flags = char_prop_flags!(BROADCAST, READ, NOTIFY);
/// let characteristic = characteristic!(12345678901234567890123456789012, 12345678901234567890123456789012, flags);
/// ```
#[macro_export]
macro_rules! characteristic {
    ($uuid:expr, $service_uuid:expr, $properties:expr) => {
        Characteristic {
            uuid: uuid!($uuid),
            service_uuid: uuid!($service_uuid),
            properties: char_prop_flags!($properties),
        }
    };
}

/// Create a new `CharPropFlags` with the specified flags.
/// Flags can be separated by commas.
/// Available flags are:
/// BROADCAST
/// READ
/// WRITE_WITHOUT_RESPONSE
/// WRITE
/// NOTIFY
/// INDICATE
/// AUTHENTICATED_SIGNED_WRITES
/// EXTENDED_PROPERTIES
/// 
/// # Example
/// 
/// ```
/// let flags = char_prop_flags!(BROADCAST, READ, NOTIFY);
/// ```
#[macro_export]
macro_rules! char_prop_flags {
    ($($flag:ident),*) => {
        $($crate::CharPropFlags::$flag)|*
    };
}