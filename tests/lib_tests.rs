use boson::libsboson::generate::generate_command;
// find_config,
// read_config
// parse_config,
// get_config,
// };

#[test]
fn generate_command_no_options() {
    let result = generate_command("test");
    assert_eq!(String::from("test"), result.unwrap());
}
