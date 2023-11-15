# rtnw

It is a Rust implementation of ["Ray Tracing: The Next Week" by Peter Shirley](https://raytracing.github.io/books/RayTracingTheNextWeek.html)

i found it helpful [cbiffle's implementation](https://github.com/cbiffle/rtiow-rust) when i encountered some problems in optimization.

## Some improvements

even though it still has some *tiny* problems , i do make some changes:

1. the main render loop runs parallelly by using the [rayon](https://crates.io/crates/rayon) crates.

2. using an improved version of Bvh which you may find in the [aabb-rewrite branch](https://github.com/RayTracing/raytracing.github.io/tree/aabb-rewrite).



![](./final_scene.png)

*5000 times oversampled final scene*
