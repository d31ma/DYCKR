pub fn dotnet_model_languages(extension: &str) -> Vec<&str> {
    if extension == "cs" {
        return vec!["cs", "fs"];
    }
    vec![extension]
}
