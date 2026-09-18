match request {
    Some(value) if value > 10 => {
        let adjusted = value * 2 + compute(value)?;
        if adjusted > limit { adjusted } else { limit }
    }
    Some(value) => value,
    None => return fallback(),
}
