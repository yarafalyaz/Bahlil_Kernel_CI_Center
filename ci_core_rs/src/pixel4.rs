/// Pixel 4 uses a pre-GKI build.
pub const BUILD_STYLE: &str = "pixel4-4.14";

pub fn is_pixel4(style: Option<&str>) -> bool {
    style == Some(BUILD_STYLE)
}

pub fn localversion(base: &str, sha: &str) -> anyhow::Result<String> {
    if sha.is_empty() || !sha.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(anyhow::anyhow!("Invalid kernel source commit: {sha}"));
    }
    Ok(format!(
        "-{}-g{}-openela",
        base.trim_start_matches('-').trim_end_matches("-openela"),
        sha
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn release_uses_kaltsit_and_preserves_the_ci_kernel_hash() {
        assert_eq!(
            localversion("-Kokuban-Kaltsit-BakaSU", "abcdef012345").unwrap(),
            "-Kokuban-Kaltsit-BakaSU-gabcdef012345-openela"
        );
        assert_eq!(
            localversion("-Custom-openela", "abcdef012345").unwrap(),
            "-Custom-gabcdef012345-openela"
        );
        assert!(localversion("-Kokuban-Kaltsit-BakaSU", "unknown").is_err());
        assert!(localversion("-Kokuban-Kaltsit-BakaSU", "").is_err());
    }

    #[test]
    fn selects_only_the_pixel4_pipeline() {
        assert!(is_pixel4(Some(BUILD_STYLE)));
        assert!(!is_pixel4(None));
        assert!(!is_pixel4(Some("gki-6.12")));
    }
}
