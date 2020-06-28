use std::any::TypeId;
use std::mem::size_of;

pub fn equals<U: 'static, V: 'static>() -> bool {
    TypeId::of::<U>() == TypeId::of::<V>() && size_of::<U>() == size_of::<V>()
}

pub fn cast_ref<U: 'static, V: 'static>(u: &U) -> Option<&V> {
    if equals::<U, V>() {
        Some(unsafe { std::mem::transmute::<&U, &V>(u) })
    } else {
        None
    }
}

pub fn cast_mut<U: 'static, V: 'static>(u: &mut U) -> Option<&mut V> {
    if equals::<U, V>() {
        Some(unsafe { std::mem::transmute::<&mut U, &mut V>(u) })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equals() {
        assert!(equals::<u32, u32>());
        type SameAsU32 = u32;
        assert!(equals::<u32, SameAsU32>());
        assert!(!equals::<u32, u64>());
        assert!(!equals::<Option<u64>, u64>());
        type OptionU64 = Option<u64>;
        assert!(equals::<Option<u64>, OptionU64>());
    }

    #[test]
    fn test_cast() {
        let a: u32 = 10;
        assert!(cast_ref::<_, u32>(&a).is_some());
        assert_eq!(*cast_ref::<_, u32>(&a).unwrap(), 10);
        assert!(cast_ref::<_, u64>(&a).is_none());
    }
}
