use core::cmp::*;
use core::{mem, ptr};

use ::u2size::u2size;

// k ≥ 2 ⇒ leo[k] = leo[k - 1] + leo[k - 2] + 1
const leo: [isize; 92] = [
    0x0000000000000001,
    0x0000000000000001,
    0x0000000000000003,
    0x0000000000000005,
    0x0000000000000009,
    0x000000000000000F,
    0x0000000000000019,
    0x0000000000000029,
    0x0000000000000043,
    0x000000000000006D,
    0x00000000000000B1,
    0x000000000000011F,
    0x00000000000001D1,
    0x00000000000002F1,
    0x00000000000004C3,
    0x00000000000007B5,
    0x0000000000000C79,
    0x000000000000142F,
    0x00000000000020A9,
    0x00000000000034D9,
    0x0000000000005583,
    0x0000000000008A5D,
    0x000000000000DFE1,
    0x0000000000016A3F,
    0x0000000000024A21,
    0x000000000003B461,
    0x000000000005FE83,
    0x000000000009B2E5,
    0x00000000000FB169,
    0x000000000019644F,
    0x00000000002915B9,
    0x0000000000427A09,
    0x00000000006B8FC3,
    0x0000000000AE09CD,
    0x0000000001199991,
    0x0000000001C7A35F,
    0x0000000002E13CF1,
    0x0000000004A8E051,
    0x00000000078A1D43,
    0x000000000C32FD95,
    0x0000000013BD1AD9,
    0x000000001FF0186F,
    0x0000000033AD3349,
    0x00000000539D4BB9,
    0x00000000874A7F03,
    0x00000000DAE7CABD,
    0x00000001623249C1,
    0x000000023D1A147F,
    0x000000039F4C5E41,
    0x00000005DC6672C1,
    0x000000097BB2D103,
    0x0000000F581943C5,
    0x00000018D3CC14C9,
    0x000000282BE5588F,
    0x00000040FFB16D59,
    0x000000692B96C5E9,
    0x000000AA2B483343,
    0x0000011356DEF92D,
    0x000001BD82272C71,
    0x000002D0D906259F,
    0x0000048E5B2D5211,
    0x0000075F343377B1,
    0x00000BED8F60C9C3,
    0x0000134CC3944175,
    0x00001F3A52F50B39,
    0x0000328716894CAF,
    0x000051C1697E57E9,
    0x000084488007A499,
    0x0000D609E985FC83,
    0x00015A52698DA11D,
    0x0002305C53139DA1,
    0x00038AAEBCA13EBF,
    0x0005BB0B0FB4DC61,
    0x000945B9CC561B21,
    0x000F00C4DC0AF783,
    0x0018467EA86112A5,
    0x00274743846C0A29,
    0x003F8DC22CCD1CCF,
    0x0066D505B13926F9,
    0x00A662C7DE0643C9,
    0x010D37CD8F3F6AC3,
    0x01B39A956D45AE8D,
    0x02C0D262FC851951,
    0x04746CF869CAC7DF,
    0x07353F5B664FE131,
    0x0BA9AC53D01AA911,
    0x12DEEBAF366A8A43,
    0x1E88980306853355,
    0x316783B23CEFBD99,
    0x4FF01BB54374F0EF,
    0x81579F678064AE89,
    0xD147BB1CC3D99F79,
];

struct LeoHeap<T, Less: Fn(&T, &T) -> bool> {
    ptr: *mut T,
    sizes: u2size,
    less: Less,
}

impl<T, Less: Fn(&T, &T) -> bool> LeoHeap<T, Less> {
    #[inline]
    unsafe fn push(&mut self) {
        let order = self.sizes.trailing_zeros();
        if self.sizes > u2size::from(0) &&
           (self.sizes >> order).lsw & 7 == 3  /* last 2 trees of order k and k+1 */  {
            self.sizes ^= u2size::from(7) << order;  // Merge 2 trees
        } else if self.sizes.lsw & 3 == 2  /* last tree of order 1 */  {
            self.sizes.lsw |= 1;  // Add a tree of order 0
        } else {
            self.sizes.lsw |= 2;  // Add a tree of order 1
        }
        self.ptr = self.ptr.offset(1);
        self.insert_root();
    }

    #[inline]
    unsafe fn pop(&mut self) {
        debug_assert!(self.sizes > u2size::from(0));
        self.ptr = self.ptr.offset(-1);
        if self.sizes.lsw & 3 == 3 /* last trees of order 0 and 1 */ {
            self.sizes.lsw ^= 1;
        } else if self.sizes.lsw & 3 == 2 /* last tree of order 1 */ {
            self.sizes.lsw ^= 2;
        } else {
            let order = self.sizes.trailing_zeros();
            debug_assert!(order > 1);

            let p = self.ptr.offset(-leo[order - 2]);
            if self.sizes ^ u2size::from(1) << order > u2size::from(0) {
                let q = p.offset(-leo[order - 1]);
                if (self.less)(&*p, &*q) {
                    ptr::swap(p, q);
                    LeoHeap { ptr: q, sizes: self.sizes ^ u2size::from(1) << order,
                              less: &self.less }.insert_root();
                }
            }
            if (self.less)(&*self.ptr, &*p) {
                ptr::swap(self.ptr, p);
                LeoHeap { ptr: p, sizes: self.sizes ^ u2size::from(3) << (order - 1),
                          less: &self.less }.insert_root();
            }

            self.sizes ^= u2size::from(7) << (order - 2); // Split 2 trees
        }
    }

    // Moves the given root to its appropriate location in the sequence of roots
    // and sifts that tree.
    #[inline]
    unsafe fn insert_root(&mut self) {
        let mut r = self.ptr;
        let mut sizes = self.sizes;
        while sizes.count_ones() > 1 {
            let order = sizes.trailing_zeros();
            let p = near_heap_ultimate_root_ptr(r, order, &self.less);
            let q = r.offset(-leo[order]);
            if !(self.less)(&*p, &*q) { break }
            sizes ^= u2size::from(1) << order;
            ptr::swap(q, r);
            r = q;
        }
        sift(r, sizes.trailing_zeros(), &self.less);
    }
}

#[inline]
unsafe fn sift<T, F: Fn(&T, &T) -> bool>(mut root: *mut T, mut order: usize, f: F) {
    while order > 1 {
        let new_root = near_heap_ultimate_root_ptr(root, order, &f);
        order = match ((root as usize) - (new_root as usize))/mem::size_of::<T>() {
            0 => return,
            1 => order - 2,
            _ => order - 1,
        };
        ptr::swap(root, new_root);
        root = new_root;
    }
}

#[inline]
unsafe fn near_heap_ultimate_root_ptr<T, F: Fn(&T, &T) -> bool>(mut root: *mut T, order: usize, f: F) -> *mut T {
    if order > 1 { for &child_root in &[root.offset(-(leo[order - 2] + 1)), root.offset(-1)] {
        if f(&*root, &*child_root) { root = child_root; }
    } }
    root
}

#[inline]
pub fn sort_by<T, Cmp: Fn(&T, &T) -> Ordering>(xs: &mut [T], cmp: Cmp) { unsafe {
    let mut h = LeoHeap { ptr: xs.as_mut_ptr().offset(-1), sizes: u2size::from(0),
                          less: |a, b| Ordering::Less == cmp(a, b) };
    for _ in 0..xs.len() { h.push() }
    for _ in 0..xs.len() { h.pop() }
} }

#[cfg(test)] mod tests {
    use quickcheck::TestResult;
    use std::vec::*;

    use super::*;
    use util::is_sorted;

    #[quickcheck] fn test_sort(mut xv: Vec<isize>) -> TestResult {
        sort_by(&mut xv, Ord::cmp);
        TestResult::from_bool(is_sorted(&xv))
    }
}
