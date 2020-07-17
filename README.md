I have installed the latest version of geos available (3.8.1) from
[GEOS](https://trac.osgeo.org/geos/)

When I execute this program (`cargo run`) I have the following:

```
ordinate: 2
line: 0
test-geos: geos_ts_c.cpp:3928: int GEOSCoordSeq_getOrdinate_r(GEOSContextHandle_t, const geos::geom::CoordinateSequence*, unsigned int, unsigned int, double*): Assertion `0 != val' failed.
fish: “cargo run 2>&1 > /tmp/error.txt” terminated by signal SIGABRT (Abort)
```

and indeed, in the original geos library, in `geom/geos_ts_c.cpp`, we have the following:

```cpp
int
GEOSCoordSeq_getOrdinate_r(GEOSContextHandle_t extHandle, const CoordinateSequence* cs,
                           unsigned int idx, unsigned int dim, double* val)
{
  assert(0 != cs);
  assert(0 != val);
  [...]
}
```

So it's the `val` which is 0...

Now my question, and I don't have much experience in FFI, is that the geos rust package has a
slightly different API for that function: in `src/functions.rs`, I have

```rust
pub fn GEOSCoordSeq_getOrdinate(
  s: *const GEOSCoordSequence,
  idx: c_uint,
  ordinate: size_t,
) -> c_double;
```

So, shouldn't there be a pointer at the end of this function?, and then, in the rust geos crate,
we would go from

```rust
unsafe {
  GEOSCoordSeq_getOrdinate_r(self.get_raw_context(), self.as_raw(), line as _, ordinate)
}
```

to something like

```rust
unsafe {
  let mut ord = 0_f64;
  GEOSCoordSeq_getOrdinate_r(self.get_raw_context(), self.as_raw(), line as _, ordinate, &mut ord);
  ord
}
```

