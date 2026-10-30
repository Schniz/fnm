fn main() {
    // The manifest belongs to the `fnm` executable. A crate using fnm as a
    // library has its own, and must not be handed this one to link.
    #[cfg(feature = "cli")]
    embed_resource::compile("fnm-manifest.rc", embed_resource::NONE);
}
