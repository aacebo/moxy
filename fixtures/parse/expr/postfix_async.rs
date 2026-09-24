async move {
    let value = client.fetch(index as usize).await?;
    let mapped = value.entries()[0].clone().into_result()?;
    (|item| item.normalize().finish())(mapped)
}
