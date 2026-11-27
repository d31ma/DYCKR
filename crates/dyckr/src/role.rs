#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Role {
    pub marker: &'static str,
    pub name: &'static str,
    pub folder: &'static str,
}
impl Role {
    pub fn named(name: &str) -> Option<Self> {
        let name = name
            .strip_prefix("yon::")
            .or_else(|| name.strip_prefix("tac::"))
            .unwrap_or(name);
        let (marker, name, folder) = match name.replace('_', "").to_ascii_lowercase().as_str() {
            "requestroute" => ("RequestRoute", "Controller", "controllers"),
            "socketroute" => ("SocketRoute", "Controller", "controllers"),
            "errorhandler" => ("ErrorHandler", "ErrorHandler", "controllers"),
            "identityprovider" => ("IdentityProvider", "IdentityProvider", "controllers"),
            "service" => ("Service", "Service", "services"),
            "store" => ("Store", "Store", "stores"),
            "client" => ("Client", "Client", "remotes"),
            "httpclient" => ("HTTPClient", "Client", "remotes"),
            "delegate" => ("Delegate", "Delegate", "delegates"),
            "converter" => ("Converter", "Converter", "converters"),
            "setup" => ("Setup", "Setup", "setups"),
            "entity" => ("Entity", "Entity", "entities"),
            _ => return None,
        };
        Some(Self {
            marker,
            name,
            folder,
        })
    }

    pub fn is_controller(self) -> bool {
        self.folder == "controllers"
    }

    pub fn is_http_client(self) -> bool {
        self.marker == "HTTPClient"
    }

    pub fn has_suffix_in(self, type_name: &str) -> bool {
        type_name.len() > self.name.len() && type_name.ends_with(self.name)
    }

    pub fn expected_type_name(self, type_name: &str) -> String {
        if self.has_suffix_in(type_name) {
            return type_name.to_owned();
        }
        if type_name == self.name {
            return format!("Domain{}", self.name);
        }
        format!("{type_name}{}", self.name)
    }

    pub fn is_placed_in(self, parent: &str) -> bool {
        if self.is_controller() {
            return crate::role::Role::beneath(parent, "server/controllers");
        }
        if self.folder == "remotes"
            && (crate::role::Role::beneath(parent, "client/clients") || crate::role::Role::beneath(parent, "server/clients"))
        {
            return true;
        }
        crate::role::Role::beneath(parent, &format!("client/{}", self.folder))
            || crate::role::Role::beneath(parent, &format!("server/{}", self.folder))
    }
    pub fn beneath(parent: &str, root: &str) -> bool {
        parent == root
            || parent
                .strip_prefix(root)
                .is_some_and(|tail| tail.starts_with('/'))
    }
}
