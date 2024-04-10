use std::arch::x86_64::*;

use crate::math::Math;

pub const CHUNK_0: usize = 0;
pub const CHUNK_1: usize = 1;

#[inline(always)]
pub fn cosine<M: Math>(dot_product: f32, norm_x: f32, norm_y: f32) -> f32 {
    if norm_x == 0.0 && norm_y == 0.0 {
        0.0
    } else if norm_x == 0.0 || norm_y == 0.0 {
        1.0
    } else {
        M::sub(1.0, M::div(dot_product, M::mul(norm_x, norm_y).sqrt()))
    }
}

#[inline(always)]
/// Performs a sum of all packed values in the provided [__m256] register
/// returning the resulting f32 value.
pub(crate) unsafe fn sum_avx2(v: __m256) -> f32 {
    let left_half = _mm256_extractf128_ps::<1>(v);
    let right_half = _mm256_castps256_ps128(v);
    let sum_quad = _mm_add_ps(left_half, right_half);

    let left_half = sum_quad;
    let right_half = _mm_movehl_ps(sum_quad, sum_quad);
    let sum_dual = _mm_add_ps(left_half, right_half);

    let left_half = sum_dual;
    let right_half = _mm_shuffle_ps::<0x1>(sum_dual, sum_dual);
    let sum = _mm_add_ss(left_half, right_half);

    _mm_cvtss_f32(sum)
}

#[allow(clippy::too_many_arguments)]
#[inline(always)]
/// Rolls up 8 [__m256] registers into 1 summing them together.
pub(crate) unsafe fn rollup_x8(
    mut acc1: __m256,
    acc2: __m256,
    mut acc3: __m256,
    acc4: __m256,
    mut acc5: __m256,
    acc6: __m256,
    mut acc7: __m256,
    acc8: __m256,
) -> __m256 {
    acc1 = _mm256_add_ps(acc1, acc2);
    acc3 = _mm256_add_ps(acc3, acc4);
    acc5 = _mm256_add_ps(acc5, acc6);
    acc7 = _mm256_add_ps(acc7, acc8);

    acc1 = _mm256_add_ps(acc1, acc3);
    acc5 = _mm256_add_ps(acc5, acc7);

    _mm256_add_ps(acc1, acc5)
}

#[allow(clippy::too_many_arguments)]
#[inline(always)]
/// Rolls up 8 [__m256] registers into 1 summing them together.
pub(crate) unsafe fn sum_avx512_x8(
    mut acc1: __m512,
    acc2: __m512,
    mut acc3: __m512,
    acc4: __m512,
    mut acc5: __m512,
    acc6: __m512,
    mut acc7: __m512,
    acc8: __m512,
) -> f32 {
    acc1 = _mm512_add_ps(acc1, acc2);
    acc3 = _mm512_add_ps(acc3, acc4);
    acc5 = _mm512_add_ps(acc5, acc6);
    acc7 = _mm512_add_ps(acc7, acc8);

    acc1 = _mm512_add_ps(acc1, acc3);
    acc5 = _mm512_add_ps(acc5, acc7);

    acc1 = _mm512_add_ps(acc1, acc5);
    _mm512_reduce_add_ps(acc1)
}

#[inline(always)]
pub(crate) unsafe fn offsets_avx2<const CHUNK: usize>(
    ptr: *const f32,
) -> [*const f32; 4] {
    [
        ptr.add(CHUNK * 32),
        ptr.add((CHUNK * 32) + 8),
        ptr.add((CHUNK * 32) + 16),
        ptr.add((CHUNK * 32) + 24),
    ]
}

#[inline(always)]
pub(crate) unsafe fn offsets_avx512<const CHUNK: usize>(
    ptr: *const f32,
) -> [*const f32; 4] {
    [
        ptr.add(CHUNK * 64),
        ptr.add((CHUNK * 64) + 16),
        ptr.add((CHUNK * 64) + 32),
        ptr.add((CHUNK * 64) + 48),
    ]
}

#[allow(clippy::too_many_arguments)]
#[inline(always)]
/// Sums 8 scalar accumulators into one f32 value.
pub fn rollup_scalar_x8<M: Math>(
    mut acc1: f32,
    acc2: f32,
    mut acc3: f32,
    acc4: f32,
    mut acc5: f32,
    acc6: f32,
    mut acc7: f32,
    acc8: f32,
) -> f32 {
    acc1 = M::add(acc1, acc2);
    acc3 = M::add(acc3, acc4);
    acc5 = M::add(acc5, acc6);
    acc7 = M::add(acc7, acc8);

    acc1 = M::add(acc1, acc3);
    acc5 = M::add(acc5, acc7);

    M::add(acc1, acc5)
}

#[cfg(test)]
mod tests {
    use std::array;

    use super::*;
    use crate::math::StdMath;

    #[test]
    fn test_avx2_offsets() {
        let x: [f32; 32] = array::from_fn(|i| i as f32);
        let [p1, p2, p3, p4] = unsafe { offsets_avx2::<CHUNK_0>(x.as_ptr()) };
        assert_eq!(x[0..].as_ptr(), p1);
        assert_eq!(x[8..].as_ptr(), p2);
        assert_eq!(x[16..].as_ptr(), p3);
        assert_eq!(x[24..].as_ptr(), p4);
    }

    #[test]
    fn test_avx512_offsets() {
        let x: [f32; 64] = array::from_fn(|i| i as f32);
        let [p1, p2, p3, p4] = unsafe { offsets_avx512::<CHUNK_0>(x.as_ptr()) };
        assert_eq!(x[0..].as_ptr(), p1);
        assert_eq!(x[16..].as_ptr(), p2);
        assert_eq!(x[32..].as_ptr(), p3);
        assert_eq!(x[48..].as_ptr(), p4);
    }

    #[test]
    fn test_rollup_scalar_x8() {
        let res = rollup_scalar_x8::<StdMath>(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(res, 0.0);

        let res = rollup_scalar_x8::<StdMath>(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0);
        assert_eq!(res, 8.0);
    }

    #[test]
    fn test_sum_avx2() {
        unsafe {
            let acc = _mm256_setzero_ps();
            let res = sum_avx2(acc);
            assert_eq!(res, 0.0);

            let acc = _mm256_set1_ps(1.0);
            let res = sum_avx2(acc);
            assert_eq!(res, 8.0);
        }
    }

    #[test]
    fn test_rollup_avx2_x8() {
        unsafe {
            let acc1 = _mm256_setzero_ps();
            let acc2 = _mm256_setzero_ps();
            let acc3 = _mm256_setzero_ps();
            let acc4 = _mm256_setzero_ps();
            let acc5 = _mm256_setzero_ps();
            let acc6 = _mm256_setzero_ps();
            let acc7 = _mm256_setzero_ps();
            let acc8 = _mm256_setzero_ps();
            let res = rollup_x8(acc1, acc2, acc3, acc4, acc5, acc6, acc7, acc8);
            let res = sum_avx2(res);
            assert_eq!(res, 0.0);

            let acc1 = _mm256_set1_ps(1.0);
            let acc2 = _mm256_set1_ps(1.0);
            let acc3 = _mm256_set1_ps(1.0);
            let acc4 = _mm256_set1_ps(1.0);
            let acc5 = _mm256_set1_ps(1.0);
            let acc6 = _mm256_set1_ps(1.0);
            let acc7 = _mm256_set1_ps(1.0);
            let acc8 = _mm256_set1_ps(1.0);
            let res = rollup_x8(acc1, acc2, acc3, acc4, acc5, acc6, acc7, acc8);
            let res = sum_avx2(res);
            assert_eq!(res, 64.0);
        }
    }
}
