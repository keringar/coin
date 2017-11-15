error_chain!{
    errors {
        ConfigLoad(path: String) {
            description("Config file not found")
            display("Unable to open file '{}'", path)
        }

        ConfigParse(path: String) {
            description("Configuration file could not be parsed")
            display("Unable to parse file '{}'", path)
        }
    }
}