#[cfg(test)]
mod obj {
    use crate::formats::obj;

    #[test]
    fn can_load() {
        let content = "v 0.0 0.0 0.0 1.0\n\
        v 0.1 0.2 0.3 1.0\n\
        v 0.4 0.5 0.6 1.0\n\
        f 1 2 3".to_string();

        let _obj = obj::load(content);
    }

    #[test]
    #[should_panic]
    fn cannot_load() {
        let content = "v 0.0 0.0 0.0 1.0\n\
        v 0.1 0.2 0.3 1.0\n\
        f 1 2 3".to_string();

        let _obj = obj::load(content);
    }
}
