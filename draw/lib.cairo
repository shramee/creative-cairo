// use examples::fib_array::fib;

#[derive(Copy, Drop)]
struct Arc {
    method: felt,
    args: felt,
    x: felt,
    y: felt,
    r: felt,
    start: felt,
    end: felt,
}

// fn main() -> (Array::<felt>, felt, u128) {
// fn main() -> Array::<felt> {
fn arc( x: felt, y: felt, r: felt, start: felt, end: felt ) -> Arc {
    Arc{method: 'arc', args: 5,x: x, y: y, r: r, start: start, end: end}
}

// fn main() -> (Array::<felt>, felt, u128) {
// fn main() -> Array::<felt> {
fn circle(x: felt, y: felt, r: felt) -> Arc {
    arc( x, y, r, 0, 10000 )
}
