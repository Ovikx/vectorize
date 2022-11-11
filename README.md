# Vectorize
Vector and matrix operations in Rust.

# Examples
## Scalar multiplication (vectors)
#### Code
```rust
let v1 = vector![1.1, 0.0, -3.5];
let v2 = 2.0 * v1.clone();
println!("{:?}", &v2);
```
#### Output
```
[2.2, 0.0, -7.0]
```

## Scalar multiplication (matrices)
#### Code
```rust
let m1 = Matrix::ones(3,3); // Create a matrix filled with 1.0's
let m2 = 2.0 * m1.clone();
println!("{:?}", &m2);
```
#### Output
```
[
	[2.0, 2.0, 2.0]
	[2.0, 2.0, 2.0]
	[2.0, 2.0, 2.0]
]
```

## Dot product
#### Code
```rust
let v1 = vector![1.0, 0.0, -3.5];
let v2 = vector![3.0, 1.0, 2.0];
let dot = v1.dot(&v2);
println!("{:?}", &dot);
```
#### Output
```
-4.0
```

## Vector addition
#### Code
```rust
let v1 = vector![1.0, 0.0, -3.5];
let v2 = vector![3.0, 1.0, 2.0];
let v3 = v1.clone() + v2.clone();
println!("{:?}", &v3);
```
#### Output
```
[4.0, 1.0, -1.5]
```