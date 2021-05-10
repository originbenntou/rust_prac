#[argio::argio]
fn main(n: usize, children: [u32; n]) -> u32 {
    children.into_iter().sum()
}
