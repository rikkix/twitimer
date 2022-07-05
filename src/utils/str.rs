pub fn str_join(s1: &str, s2: &str) -> String {
    let mut s = s1.to_string();
    s.push(' ');
    s.push_str(s2);
    s
}
