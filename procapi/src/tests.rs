#[cfg(test)]
mod tests {
    use crate::ProcessInfo;

    #[test]
    fn test() {
        dbg!(ProcessInfo::processes());
    }
}

