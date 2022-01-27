fn main() {
    let kv = steam_kit::vdf::KeyValues::parse(r#"
        test
        {
            "3"		"DMR ALİ"
            "4"		"무슨 일이"
            "5"		"發生了什麼"
            "6"		"Zażółć gęślą jaźń"
            "7"		"ماذا يحدث"
        }
        "UserLocalConfigStore"
        {
            "friends"
            {
                "9734658" "Grocel"
                "5318652" "/Hexalitos\\" //<-- this one
                "19358028" "put in"
            }
        }
    "#).unwrap();

    println!("{:?}", kv);
}
