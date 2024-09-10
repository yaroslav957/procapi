#[cfg(test)]
mod tests {
    use crate::process::ProcessInfo;

    #[test]
    fn test() {
        dbg!(ProcessInfo::init().processes);
    }
}
