mod test_glue;

use indoc::indoc;
use roc_std::RocStr;
use test_glue::StrFingerTree;

extern "C" {
    #[link_name = "roc__mainForHost_1_exposed_generic"]
    fn roc_main(_: *mut StrFingerTree);
}

#[no_mangle]
pub extern "C" fn rust_main() -> i32 {
    use std::cmp::Ordering;
    use std::collections::hash_set::HashSet;

    let tag_union = test_glue::mainForHost(());

    // Eq
    assert!(StrFingerTree::Empty() == StrFingerTree::Empty());
    assert!(StrFingerTree::Empty() != tag_union);
    assert!(
        StrFingerTree::Single(RocStr::from("foo")) == StrFingerTree::Single(RocStr::from("foo"))
    );
    assert!(StrFingerTree::Single(RocStr::from("foo")) != StrFingerTree::Empty());

    // Verify that it has all the expected traits.
    assert!(tag_union == tag_union); // PartialEq
    assert!(tag_union.clone() == tag_union.clone()); // Clone
    assert!(StrFingerTree::Empty().clone() == StrFingerTree::Empty()); // Clone

    assert!(tag_union.partial_cmp(&tag_union) == Some(Ordering::Equal)); // PartialOrd
    assert!(tag_union.cmp(&tag_union) == Ordering::Equal); // Ord

    print!(
        indoc!(
            r#"
                tag_union was: {:?}
                `More "small str" (Single "other str")` is: {:?}
                `More "small str" Empty` is: {:?}
                `Single "small str"` is: {:?}
                `Empty` is: {:?}
            "#
        ),
        tag_union,
        StrFingerTree::More(
            "small str".into(),
            StrFingerTree::Single("other str".into()),
        ),
        StrFingerTree::More("small str".into(), StrFingerTree::Empty()),
        StrFingerTree::Single("small str".into()),
        StrFingerTree::Empty(),
    ); // Debug

    let mut set = HashSet::new();

    set.insert(tag_union.clone()); // Eq, Hash
    set.insert(tag_union);

    assert_eq!(set.len(), 1);

    // Exit code
    0
}

// Externs required by roc_std and by the Roc app

use core::ffi::c_void;
use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub unsafe extern "C" fn roc_alloc(size: usize, _alignment: u32) -> *mut c_void {
    return libc::malloc(size);
}

#[no_mangle]
pub unsafe extern "C" fn roc_realloc(
    c_ptr: *mut c_void,
    new_size: usize,
    _old_size: usize,
    _alignment: u32,
) -> *mut c_void {
    return libc::realloc(c_ptr, new_size);
}

#[no_mangle]
pub unsafe extern "C" fn roc_dealloc(c_ptr: *mut c_void, _alignment: u32) {
    return libc::free(c_ptr);
}

#[no_mangle]
pub unsafe extern "C" fn roc_panic(c_ptr: *mut c_void, tag_id: u32) {
    match tag_id {
        0 => {
            let slice = CStr::from_ptr(c_ptr as *const c_char);
            let string = slice.to_str().unwrap();
            eprintln!("Roc hit a panic: {}", string);
            std::process::exit(1);
        }
        _ => todo!(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn roc_memcpy(dst: *mut c_void, src: *mut c_void, n: usize) -> *mut c_void {
    libc::memcpy(dst, src, n)
}

#[no_mangle]
pub unsafe extern "C" fn roc_memset(dst: *mut c_void, c: i32, n: usize) -> *mut c_void {
    libc::memset(dst, c, n)
}
