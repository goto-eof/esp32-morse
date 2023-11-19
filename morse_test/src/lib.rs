extern crate morse_service;

#[cfg(test)]
mod tests {

    use morse_service::morse_service::retrieve_translation_map;
    use morse_service::morse_service::translate;
    use morse_service::morse_service::END;
    use morse_service::morse_service::PAUSE;
    #[test]
    fn translate_test() {
        let a = translate("A");
        let map = retrieve_translation_map();
        let mut correct_vec = map.get(&'A').unwrap().to_owned();
        let end_values = &mut vec![PAUSE, END];
        correct_vec.append(end_values);
        assert_eq!(vec_compare(&correct_vec, &a.unwrap()), true);
    }

    #[test]
    fn translate_test_fail() {
        let a = translate("A");
        println!("{:?}", a);
        assert_eq!(
            vec_compare(retrieve_translation_map().get(&'B').unwrap(), &a.unwrap()),
            false
        );
    }

    fn vec_compare(va: &[u32], vb: &[u32]) -> bool {
        if va.len() != vb.len() {
            return false;
        }

        for (index, a) in va.iter().enumerate() {
            if a.ne(vb.get(index).unwrap()) {
                return false;
            }
        }

        true
    }
}
