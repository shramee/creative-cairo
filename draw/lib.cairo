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

// Draws an arc
// start and end angles map 0 - 10000 to 0 - 2*PI
fn arc( x: felt, y: felt, r: felt, start: felt, end: felt ) -> Arc {
    Arc{method: 'arc', args: 5,x: x, y: y, r: r, start: start, end: end}
}

// Draws an arc from 0 to 10000 (2*PI)
fn circle(x: felt, y: felt, r: felt) -> Arc {
    arc( x, y, r, 0, 10000 )
}
