pub fn transmute_vec<S, T>(mut vec: Vec<S>) -> Vec<T> {
    let ptr = vec.as_mut_ptr() as *mut T;
    let len = vec.len() * std::mem::size_of::<S>() / std::mem::size_of::<T>();
    let capacity = vec.capacity() * std::mem::size_of::<S>() / std::mem::size_of::<T>();
    std::mem::forget(vec);
    unsafe { Vec::from_raw_parts(ptr, len, capacity) }
}

pub fn transmute_slice<S, T>(slice: &[S]) -> &[T] {
    let ptr = slice.as_ptr() as *const T;
    let len = slice.len() * std::mem::size_of::<S>() / std::mem::size_of::<T>();
    unsafe { std::slice::from_raw_parts(ptr, len) }
}

pub fn transmute_slice_mut<S, T>(slice: &mut [S]) -> &mut [T] {
    let ptr = slice.as_mut_ptr() as *mut T;
    let len = slice.len() * std::mem::size_of::<S>() / std::mem::size_of::<T>();
    unsafe { std::slice::from_raw_parts_mut(ptr, len) }
}

pub fn signed_triangle_area(t: &[glam::Vec2]) -> f32 {
    glam::Mat2::from_cols(t[0] - t[2], t[1] - t[2]).determinant()
}