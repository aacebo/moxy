fn shorthand(value: Point) {
    let Point { x } = value;
    let Point { x, y } = value;
    let Point { x, y: renamed, r#type, .. } = value;
    let Point { ref x, mut y, ref mut z, } = value;
}
