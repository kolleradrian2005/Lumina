# Fixing libs for android compilation

## rusttype lib error

From

```rs
pub enum Font {
    Ref(Arc<owned_ttf_parser::Face>),
    Owned(Arc<owned_ttf_parser::OwnedFace>),
}
```

to

```rs
pub enum Font<'a> {
    Ref(Arc<owned_ttf_parser::Face<'a>>),
    Owned(Arc<owned_ttf_parser::OwnedFace>),
}
```

## glutin-winit lib error

Add the following lines to C:\users\xyz\.cargo\registry\src\index.crates.io-...\glutin-winit-x.x.x\Cargo.toml

```toml
[target.'cfg(target_os = "android")'.dependencies.winit]
version = "0.29.2"
features = ["android-native-activity, rwh_05"]
default-features = false

```
