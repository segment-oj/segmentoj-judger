#[cfg(test)]
mod tests {
    use crate::task::*;

    #[test]
    fn test_queue() {
        let mut que = queue::Queue::new();

        que.add(content::Task::new(
            2,
            10054511,
            content::Code::new("Example Code".to_string(), content::Lang::Js),
            "http://172.32.5.72:3000".to_string(),
        ));

        assert_eq!(que.is_idle(), false);
        assert_eq!(que.get().unwrap().sid, 10054511);
        assert!(que.get().is_none());
        assert_eq!(que.is_idle(), true);
    }
}
