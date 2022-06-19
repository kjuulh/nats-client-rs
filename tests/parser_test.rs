use nats_client_rs::parser;

#[test]
fn parse_connect() {
    let connect_raw = r#"[CONNECT {"verbose":false,"pedantic":false,"tls_required":false,"name":"","lang":"go","version":"1.2.2","protocol":1}]"#;

    let res = parser::Parser::parse(connect_raw);

    assert_eq!(true, res.is_ok())
}