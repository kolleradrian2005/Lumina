## Layers

Z-indices

```
Particles - 0.01 // Maybe variable later
Player - 0.0
Models - 0.0
Terrain - -0.0001 // Cannot lower ?
Background #1 - -0.25 // Cannot lower ?
```

## rusttype lib error

From

```
pub enum Font {
    Ref(Arc<owned_ttf_parser::Face>),
    Owned(Arc<owned_ttf_parser::OwnedFace>),
}
```

to

```
pub enum Font<'a> {
    Ref(Arc<owned_ttf_parser::Face<'a>>),
    Owned(Arc<owned_ttf_parser::OwnedFace>),
}
```
