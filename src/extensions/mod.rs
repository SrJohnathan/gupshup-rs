

pub fn remove_first_nine_from_brazilian_phone(phone: &str) -> String {
    if phone.starts_with("55") {
        let ddi = &phone[0..2];
        let ddd = &phone[2..4];
        let local_number = &phone[4..];
        let mut found_first_nine = false;
        let new_local_number: String = local_number.chars()
            .filter(|&c| {
                if c == '9' && !found_first_nine {
                    found_first_nine = true;
                    false
                } else {
                    true
                }
            })
            .collect();
        format!("{}{}{}", ddi, ddd, new_local_number)
    } else {
        phone.to_string()
    }
}