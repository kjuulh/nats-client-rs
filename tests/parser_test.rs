use nats_client_rs::parser;

#[test]
fn parse_connect() {
    let connect_raw = r#"[CONNECT {"verbose":false,"pedantic":false,"tls_required":false,"name":"","lang":"go","version":"1.2.2","protocol":1}]"#;

    let res = parser::Parser::parse(connect_raw);

    assert_eq!(true, res.is_ok())
}

#[test]
fn parse_info() {
    let info_raw = r#"INFO {"server_id":"NBW5LX4Z6CAQR37FIRTZIK7247AVDBKQKHWQJTZBO2GZIIPYR433U26N","server_name":"NBW5LX4Z6CAQR37FIRTZIK7247AVDBKQKHWQJTZBO2GZIIPYR433U26N","version":"2.8.2","proto":1,"git_commit":"9e5d25b","go":"go1.17.9","host":"0.0.0.0","port":4222,"headers":true,"max_payload":1048576,"client_id":32,"client_ip":"172.17.0.1"}"#;

    let res = parser::Parser::parse(info_raw);

    assert_eq!(true, res.is_ok())
}
