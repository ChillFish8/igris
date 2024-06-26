#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[derive(Debug, Copy, Clone)]
/// AVX2 enabled architectures.
pub struct Avx2(());

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl Default for Avx2 {
    fn default() -> Self {
        assert!(
            is_x86_feature_detected!("avx2"),
            "AVX2 support is not available on the current platform"
        );
        Self(())
    }
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "nightly"))]
#[derive(Debug, Copy, Clone)]
/// AVX512 enabled architectures.
pub struct Avx512(());

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "nightly"))]
impl Default for Avx512 {
    fn default() -> Self {
        assert!(
            is_x86_feature_detected!("avx512f"),
            "AVX512f support is not available on the current platform"
        );
        Self(())
    }
}

#[derive(Debug, Copy, Clone, Default)]
/// No specialised features detected, fallback impls.
pub struct Fallback(());

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub struct Avx2Fma(());
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl Default for Avx2Fma {
    fn default() -> Self {
        Self(())
    }
}

#[derive(Debug, Copy, Clone, Default)]
/// A dynamically selectable arch.
///
/// This will automatically select the best set of features to use at runtime.
pub struct Auto(pub(crate) SelectedArch);

#[derive(Debug, Copy, Clone)]
pub(crate) enum SelectedArch {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    Avx2,
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    Avx2Fma,
    #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "nightly"))]
    Avx512,
    Fallback,
}

impl Default for SelectedArch {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn default() -> Self {
        #[cfg(all(
            any(target_arch = "x86", target_arch = "x86_64"),
            feature = "nightly"
        ))]
        if is_x86_feature_detected!("avx512f") {
            return Self::Avx512;
        }

        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        if is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma") {
            return Self::Avx2Fma;
        }

        #[cfg(all(
            any(target_arch = "x86", target_arch = "x86_64"),
            feature = "nightly"
        ))]
        if is_x86_feature_detected!("avx512f") {
            return Self::Avx512;
        }

        if is_x86_feature_detected!("avx2") {
            return Self::Avx2;
        }

        Self::Fallback
    }

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    fn default() -> Self {
        Self::Fallback
    }
}

pub trait Arch: Default {}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl Arch for Avx2 {}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl Arch for Avx2Fma {}
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "nightly"))]
impl Arch for Avx512 {}
impl Arch for Auto {}
impl Arch for Fallback {}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    #[test]
    fn test_if_avx2_enabled() {
        if is_x86_feature_detected!("avx2") {
            Avx2::default();
        } else {
            assert!(
                std::panic::catch_unwind(Avx2::default).is_err(),
                "Type should panic due to missing cpu flags",
            );
        }
    }

    #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "nightly"))]
    #[test]
    fn test_if_avx512_enabled() {
        if is_x86_feature_detected!("avx512f") {
            Avx512::default();
        } else {
            assert!(
                std::panic::catch_unwind(Avx512::default).is_err(),
                "Type should panic due to missing cpu flags",
            );
        }
    }
}
