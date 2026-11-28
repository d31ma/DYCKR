use crate::annotation_target::{Target, Arguments};
use crate::realm::{Realm, TAC, YON};

pub struct Rule {
    pub name: &'static str,
    pub meaning: &'static str,
    pub targets: &'static [Target],
    pub arguments: Arguments,
    pub tac: &'static [&'static str],
    pub yon: &'static [&'static str],
    pub instance: bool,
    pub with: &'static [&'static str],
    pub clashes: &'static [&'static str],
    pub class_needs: &'static [&'static str],
    pub one_per_method: bool,
    pub refuses_constructor: bool,
    pub refuses_nullable: bool,
    pub argument_advice: &'static str,
    pub replaced_by: &'static str,
    pub advice: &'static str,
    pub code: u16,
}

impl Rule {
    pub const fn new(name: &'static str, targets: &'static [Target], meaning: &'static str) -> Self {
        Self {
            name,
            meaning,
            targets,
            arguments: Arguments::Optional,
            tac: AnnotationRules::TAC_BEHAVIOR,
            yon: AnnotationRules::YON_BEHAVIOR,
            instance: false,
            with: &[],
            clashes: &[],
            class_needs: &[],
            one_per_method: false,
            refuses_constructor: false,
            refuses_nullable: false,
            argument_advice: "",
            replaced_by: "",
            advice: "",
            code: 1308,
        }
    }

    pub const fn retired(name: &'static str, replaced_by: &'static str, advice: &'static str) -> Self {
        Self {
            replaced_by,
            advice,
            targets: &[],
            tac: &[],
            yon: &[],
            ..Self::new(name, &[], "")
        }
    }

    pub const fn bare(self) -> Self {
        Self {
            arguments: Arguments::None,
            ..self
        }
    }

    pub const fn needs_arguments(self) -> Self {
        Self {
            arguments: Arguments::Required,
            ..self
        }
    }

    pub const fn reach(self, tac: &'static [&'static str], yon: &'static [&'static str]) -> Self {
        Self { tac, yon, ..self }
    }

    pub const fn instance(self) -> Self {
        Self {
            instance: true,
            ..self
        }
    }

    pub const fn with(self, with: &'static [&'static str]) -> Self {
        Self { with, ..self }
    }

    pub const fn clashes(self, clashes: &'static [&'static str]) -> Self {
        Self { clashes, ..self }
    }

    pub const fn class_needs(self, class_needs: &'static [&'static str]) -> Self {
        Self {
            class_needs,
            ..self
        }
    }

    pub const fn one_per_method(self) -> Self {
        Self {
            one_per_method: true,
            ..self
        }
    }

    pub const fn refuses_constructor(self) -> Self {
        Self {
            refuses_constructor: true,
            ..self
        }
    }

    pub const fn refuses_nullable(self) -> Self {
        Self {
            refuses_nullable: true,
            ..self
        }
    }

    pub const fn argument_advice(self, argument_advice: &'static str) -> Self {
        Self {
            argument_advice,
            ..self
        }
    }

    pub const fn advice(self, advice: &'static str) -> Self {
        Self { advice, ..self }
    }

    pub const fn code(self, code: u16) -> Self {
        Self { code, ..self }
    }

    pub fn python_annotated(&self) -> bool {
        self.targets.contains(&Target::Field) || self.targets.contains(&Target::Parameter)
    }

    pub fn is_retired(&self) -> bool {
        self.targets.is_empty()
    }

    pub fn languages(&self, realm: Realm) -> &'static [&'static str] {
        match realm {
            Realm::Tac => self.tac,
            Realm::Yon => self.yon,
        }
    }

    pub fn allows(&self, language: &str, realm: Realm) -> bool {
        self.languages(realm).contains(&language)
    }

    pub fn allowed_targets(&self) -> String {
        let words: Vec<String> = self
            .targets
            .iter()
            .map(|target| format!("a {}", target.word()))
            .collect();
        words.join(" or ")
    }

    pub fn explained(&self) -> String {
        format!("{} {}.", self.name, self.meaning)
    }
}

pub struct AnnotationRules;
impl AnnotationRules {
    pub const YON_BEHAVIOR: &'static [&'static str] = &[
        "js", "ts", "py", "php", "java", "groovy", "kt", "cs", "fs", "rs", "dart", "go", "rb",
        "lua",
    ];
    pub const YON_ROLES: &'static [&'static str] = YON;

    pub const YON_CONTEXT: &'static [&'static str] = YON;

    pub const YON_ERRORS: &'static [&'static str] = YON;

    pub const YON_CHECKS: &'static [&'static str] = YON;

    pub const YON_POLICIES: &'static [&'static str] = YON;

    pub const YON_OUTBOUND: &'static [&'static str] = YON;

    pub const YON_SOCKET: &'static [&'static str] = YON;

    pub const YON_TEST: &'static [&'static str] = YON;

    pub const TAC_ROLE: &'static [&'static str] = TAC;
    pub const TAC_REMOTE: &'static [&'static str] = TAC;
    pub const TAC_BEHAVIOR: &'static [&'static str] = &[
        "js", "ts", "rs", "cs", "dart", "go", "swift", "kt", "rb", "py", "lua", "java", "fs",
    ];
    pub const TAC_CONTEXT: &'static [&'static str] = Self::TAC_BEHAVIOR_AND_CPP;
    pub const TAC_FAKE: &'static [&'static str] = &[
        "js", "ts", "rs", "cs", "dart", "go", "swift", "kt", "rb", "py", "hpp", "lua", "java", "fs",
    ];
    pub const TAC_BEHAVIOR_AND_CPP: &'static [&'static str] = Self::TAC_FAKE;
    pub const TAC_ADAPTER_AND_CPP: &'static [&'static str] = TAC;
    pub const TAC_STORED_STATE: &'static [&'static str] = Self::TAC_ADAPTER_AND_CPP;
    pub const TAC_HELPER: &'static [&'static str] = TAC;
    pub const TAC_NEEDS_ALL: &'static [&'static str] = &[
        "js", "ts", "rs", "cs", "dart", "go", "swift", "kt", "rb", "java", "lua", "py", "hpp", "fs",
    ];
    pub const YON_BACKGROUND: &'static [&'static str] = Self::YON_ROLES;
    pub const TAC_STREAM: &'static [&'static str] = &[
        "js", "ts", "go", "rb", "rs", "dart", "swift", "kt", "cs", "lua", "py", "hpp", "fs", "java",
    ];
    pub const TAC_VALUE_SOURCE: &'static [&'static str] = &[
        "js", "ts", "rs", "kt", "cs", "go", "rb", "dart", "swift", "lua", "hpp", "py", "java", "fs",
    ];
    pub const NOWHERE: &'static [&'static str] = &[];
    pub const TYPE: &'static [Target] = &[Target::Type];
    pub const METHOD: &'static [Target] = &[Target::Method];
    pub const FIELD: &'static [Target] = &[Target::Field];
    pub const PARAMETER: &'static [Target] = &[Target::Parameter];
    pub const FIELD_OR_TYPE: &'static [Target] = &[Target::Field, Target::Type];
    pub const TYPE_OR_METHOD: &'static [Target] = &[Target::Type, Target::Method];
    pub const ACCESS: &'static [&'static str] = &["Public", "SignedIn", "Allow"];
    pub const GUARDED_CLASSES: &'static [&'static str] = &["RequestRoute", "SocketRoute", "Topic"];
    pub const ACCESS_ADVICE: &'static str = "Every route and every server topic says who may use it (ADR 0068, ADR 0061). Write @Public, @SignedIn or @Allow(Role) on the controller class or on a route method, where the method wins, or next to @Topic on a server model";
    pub const TEST_CLASSES: &'static [&'static str] = &["Test", "TestSetup"];
    pub const SETUP_CLASSES: &'static [&'static str] = &["Setup", "TestSetup"];
    pub const ANSWERS: &'static [&'static str] = &[
        "GetRoute",
        "PostRoute",
        "PutRoute",
        "PatchRoute",
        "DeleteRoute",
        "StreamRoute",
        "HandlesError",
    ];
    pub const HTTP_METHODS: &'static [&'static str] = &[
        "GetRoute",
        "PostRoute",
        "PutRoute",
        "PatchRoute",
        "DeleteRoute",
        "StreamRoute",
        "GetRequest",
        "PostRequest",
        "PutRequest",
        "PatchRequest",
        "DeleteRequest",
        "StreamRequest",
        "SocketRequest",
    ];
    pub const SOCKET_EVENTS: &'static [&'static str] = &["OnOpen", "OnMessage", "OnClose", "OnError"];
    pub const SOCKET_ADVICE: &'static str = "A socket class is a Yon controller in server/controllers/, written in any Yon language. In the browser, receive its messages with @SocketRequest on an @HTTPClient method";
    pub const ROUTE_SOURCES: &'static [&'static str] = &[
        "FromPath",
        "FromQuery",
        "FromBody",
        "FromHeader",
        "FromCookie",
        "FromIdentity",
    ];
    pub const REQUEST_PARTS: &'static [&'static str] = &[
        "PathVariable",
        "RequestParam",
        "RequestBody",
        "RequestHeader",
    ];
    pub const CACHE_CHANGES: &'static [&'static str] = &["Cached", "ClearCache", "UpdateCache"];
    pub const LIFECYCLE: &'static [&'static str] = &["OnStart", "OnStop", "OnEvent", "Every"];
    pub const HELPER_ADVICE: &'static str = "Tac Delegate supports JavaScript, TypeScript, Dart, compiled Rust, C++, Swift, the synchronous Python ownership adapter, Go Delegates owned by the service that needs or creates them, Ruby Delegates that their service creates on first use, Kotlin, Java, C# and F# Needs fields (each needing class makes its own Delegate), Lua Delegates that their service builds with itself, and caller-owned C#/F# composition with new. Ruby, Kotlin, Java, C#, F# and Lua Delegates have no OnStart, OnStop or OnEvent. A Swift Delegate is built by the field that needs it, so it has an init() without arguments. C# and F# Delegates have no NeedsAll or Lifetime. F# Delegates require an ordinary primary constructor, and their Needs properties name other Delegates or the page service that the Delegate takes on.";
    pub const RUBY_NEEDS_ADVICE: &'static str = "A Ruby NeedsAll class lists each field as `attr_reader :order_store` and writes no `initialize`; a Ruby class that needs default values writes a plain `initialize` and no NeedsAll";

        pub const ALL: &'static [Rule] = &[
        Rule::new(
            "Test",
            Self::TYPE,
            "marks a test class and names the TestSetup class it runs with",
        )
        .needs_arguments()
        .reach(TAC, Self::YON_TEST),
        Rule::new(
            "TestSetup",
            Self::TYPE,
            "marks the class that prepares the objects a test uses",
        )
        .bare()
        .reach(TAC, Self::YON_TEST),
        Rule::new(
            "Fake",
            Self::FIELD,
            "puts a stand-in object into this field while a test runs",
        )
        .bare()
        .class_needs(Self::TEST_CLASSES)
        .reach(Self::TAC_FAKE, Self::YON_TEST),
        Rule::new(
            "ValueSource",
            Self::METHOD,
            "runs this test case once for each input in its list",
        )
        .needs_arguments()
        .instance()
        .reach(Self::TAC_VALUE_SOURCE, Self::YON_TEST)
        .class_needs(Self::TEST_CLASSES),
        Rule::new(
            "UseProfiles",
            Self::TYPE,
            "turns on the named profiles while this test runs",
        )
        .needs_arguments()
        .reach(Self::TAC_CONTEXT, Self::YON_TEST)
        .class_needs(Self::TEST_CLASSES),
        Rule::new(
            "TestSettings",
            Self::TYPE,
            "sets setting values while this test runs",
        )
        .reach(Self::TAC_CONTEXT, Self::YON_TEST)
        .needs_arguments()
        .class_needs(Self::TEST_CLASSES),
        Rule::new(
            "FreshStart",
            Self::TYPE,
            "builds new objects for every test instead of sharing them",
        )
        .bare()
        .reach(Self::TAC_BEHAVIOR_AND_CPP, Self::YON_TEST)
        .class_needs(Self::TEST_CLASSES),
        Rule::retired(
            "Client",
            "HTTPClient",
            "HTTPClient marks a class that sends HTTP requests and gives the path they start with",
        ),
        Rule::new(
            "Id",
            Self::FIELD,
            "marks the field that identifies each saved record",
        )
        .bare()
        .reach(Self::TAC_BEHAVIOR_AND_CPP, Self::YON_POLICIES)
        .class_needs(&["Entity"]),
        Rule::new(
            "Service",
            Self::TYPE,
            "marks a class that other classes can use",
        )
        .bare()
        .reach(Self::TAC_ROLE, Self::YON_ROLES),
        Rule::new(
            "Lifetime",
            Self::TYPE,
            "says how long one object of this class lives",
        )
        .needs_arguments()
        .reach(TAC, Self::YON_ROLES),
        Rule::new(
            "Delegate",
            Self::TYPE,
            "marks a class that services use for part of their work",
        )
        .bare()
        .reach(Self::TAC_HELPER, Self::YON_ROLES)
        .advice(Self::HELPER_ADVICE),
        Rule::new(
            "Converter",
            Self::TYPE,
            "marks a class whose methods copy one model into another",
        )
        .bare()
        .reach(TAC, Self::YON_CHECKS),
        Rule::new(
            "Convert",
            Self::METHOD,
            "copies the fields of one model into a new model",
        )
        .reach(TAC, Self::YON_CHECKS),
        Rule::new(
            "Needs",
            Self::FIELD_OR_TYPE,
            "asks Tachyon to supply an object that this class uses",
        )
        .reach(TAC, Self::YON_ROLES)
        .refuses_nullable(),
        Rule::new(
            "Named",
            Self::FIELD,
            "chooses which object fills a Needs field when there are several",
        )
        .needs_arguments()
        .reach(TAC, Self::YON_ROLES)
        .with(&["Needs"]),
        Rule::new(
            "Setup",
            Self::TYPE,
            "marks a class that builds and configures shared objects",
        )
        .reach(Self::TAC_CONTEXT, Self::YON_CONTEXT)
        .bare(),
        Rule::new(
            "Description",
            Self::TYPE,
            "gives the class a short description for tools and documents",
        )
        .needs_arguments()
        .reach(Self::TAC_BEHAVIOR_AND_CPP, Self::YON_ROLES),
        Rule::new(
            "Schedule",
            Self::TYPE,
            "runs this worker again and again after a fixed delay",
        )
        .needs_arguments()
        .reach(Self::NOWHERE, Self::YON_ROLES)
        .code(1502),
        Rule::new(
            "NeedsAll",
            Self::TYPE,
            "requires every field that has no initial value: collaborators are injected and the other fields come from the component's props",
        )
        .bare()
        .reach(Self::TAC_NEEDS_ALL, Self::YON_ROLES)
        .advice(Self::RUBY_NEEDS_ADVICE)
        .refuses_constructor()
        .refuses_nullable()
        .argument_advice("NeedsAll takes no field list; every field can be filled"),
        Rule::new(
            "LocalState",
            Self::FIELD,
            "keeps this field in the browser's localStorage, so its value comes back after a reload and in later visits on this device",
        )
        .bare()
        .reach(Self::TAC_STORED_STATE, Self::NOWHERE)
        .clashes(&["SessionState", "Needs"]),
        Rule::new(
            "SessionState",
            Self::FIELD,
            "keeps this field in the browser's sessionStorage, so its value comes back after a reload in the same tab",
        )
        .bare()
        .reach(Self::TAC_STORED_STATE, Self::NOWHERE)
        .clashes(&["LocalState", "Needs"]),
        Rule::new(
            "CacheSettings",
            Self::TYPE,
            "names the cache that this class's Cached methods use, and sets its limits",
        )
        .needs_arguments()
        .reach(Self::TAC_ADAPTER_AND_CPP, Self::YON_POLICIES),
        Rule::new(
            "Cached",
            Self::METHOD,
            "keeps a method's answer and returns it again for the same arguments",
        )
        .bare()
        .instance()
        .reach(Self::TAC_ADAPTER_AND_CPP, Self::YON_POLICIES)
        .clashes(Self::CACHE_CHANGES)
        .class_needs(&["CacheSettings"]),
        Rule::new(
            "CacheKey",
            Self::PARAMETER,
            "makes this argument part of the key of a cached answer",
        )
        .bare()
        .reach(Self::TAC_ADAPTER_AND_CPP, Self::YON_POLICIES),
        Rule::new(
            "ClearCache",
            Self::METHOD,
            "removes the saved answers of a Cached method after this method runs",
        )
        .needs_arguments()
        .instance()
        .reach(Self::TAC_ADAPTER_AND_CPP, Self::YON_POLICIES)
        .clashes(Self::CACHE_CHANGES),
        Rule::new(
            "UpdateCache",
            Self::METHOD,
            "saves this method's answer as the new answer of a Cached method",
        )
        .needs_arguments()
        .instance()
        .reach(Self::TAC_ADAPTER_AND_CPP, Self::YON_POLICIES)
        .clashes(Self::CACHE_CHANGES),
        Rule::new(
            "PageDefaults",
            Self::PARAMETER,
            "gives the page size and sort order that a page argument uses when the caller leaves them out",
        )
        .reach(Self::TAC_ADAPTER_AND_CPP, Self::YON_POLICIES),
        Rule::new("Retry", Self::METHOD, "runs the method again when it fails")
            .instance()
            .reach(Self::TAC_ADAPTER_AND_CPP, Self::YON_POLICIES),
        Rule::new(
            "MaxAtOnce",
            Self::METHOD,
            "limits how many calls of this method run at the same time",
        )
        .instance()
        .reach(Self::TAC_ADAPTER_AND_CPP, Self::YON_POLICIES),
        Rule::new(
            "AllOrNothing",
            Self::METHOD,
            "saves all of the method's changes, or none of them when it fails",
        )
        .instance()
        .reach(Self::TAC_ADAPTER_AND_CPP, Self::YON_POLICIES),
        Rule::new(
            "Entity",
            Self::TYPE,
            "marks a model that is saved as a record",
        )
        .reach(Self::TAC_BEHAVIOR_AND_CPP, Self::YON_POLICIES),
        Rule::new(
            "Topic",
            Self::TYPE,
            "sends each published event of this model to the listeners in every Yon language and in open pages",
        )
        .bare()
        .reach(TAC, Self::YON_OUTBOUND),
        Rule::new(
            "Table",
            Self::TYPE,
            "names the table that stores this Entity",
        )
        .needs_arguments()
        .reach(Self::TAC_BEHAVIOR_AND_CPP, Self::YON_POLICIES)
        .class_needs(&["Entity"]),
        Rule::retired(
            "WebCall",
            "GetRequest",
            "write @GetRequest, @PostRequest, @PutRequest, @PatchRequest or @DeleteRequest with the path inside: calling the method then sends that HTTP request",
        ),
        Rule::new(
            "Gateway",
            Self::METHOD,
            "calls a method of a class that is written in another language",
        )
        .needs_arguments()
        .instance()
        .reach(Self::NOWHERE, Self::YON_OUTBOUND),
        Rule::new(
            "Store",
            Self::TYPE,
            "marks a class that saves and finds records",
        )
        .reach(Self::TAC_ADAPTER_AND_CPP, Self::YON_POLICIES),
        Rule::new(
            "Query",
            Self::METHOD,
            "turns an empty method into a query of the Store's records",
        )
        .reach(Self::TAC_ADAPTER_AND_CPP, Self::YON_POLICIES)
        .instance()
        .class_needs(&["Store"]),
        Rule::new(
            "Settings",
            Self::TYPE,
            "fills the class's fields from the settings whose names share a prefix",
        )
        .needs_arguments()
        .reach(Self::TAC_BEHAVIOR_AND_CPP, Self::YON_CONTEXT),
        Rule::new(
            "WhenProfile",
            Self::TYPE,
            "uses this Setup only when one of the named profiles is on",
        )
        .reach(Self::TAC_CONTEXT, Self::YON_CONTEXT)
        .needs_arguments()
        .class_needs(Self::SETUP_CLASSES),
        Rule::new(
            "WhenSetting",
            Self::TYPE,
            "uses this Setup only when a setting has the given value",
        )
        .reach(Self::TAC_CONTEXT, Self::YON_CONTEXT)
        .needs_arguments()
        .class_needs(Self::SETUP_CLASSES),
        Rule::new(
            "Setting",
            Self::FIELD,
            "fills this field with one setting value",
        )
        .needs_arguments()
        .reach(Self::TAC_BEHAVIOR_AND_CPP, Self::YON_CONTEXT),
        Rule::new(
            "Checked",
            Self::TYPE,
            "marks a model whose fields are checked before a method uses it",
        )
        .reach(Self::TAC_BEHAVIOR_AND_CPP, Self::YON_CHECKS)
        .bare(),
        Rule::new(
            "Check",
            Self::PARAMETER,
            "checks the fields of this argument before the method runs",
        )
        .reach(Self::TAC_BEHAVIOR_AND_CPP, Self::YON_CHECKS),
        Rule::new(
            "Required",
            Self::FIELD,
            "refuses a value when this field is missing",
        )
        .reach(Self::TAC_BEHAVIOR_AND_CPP, Self::YON_CHECKS)
        .bare(),
        Rule::new(
            "NotEmpty",
            Self::FIELD,
            "refuses a value when this field is empty",
        )
        .reach(Self::TAC_BEHAVIOR_AND_CPP, Self::YON_CHECKS)
        .bare(),
        Rule::new(
            "Length",
            Self::FIELD,
            "limits how long the text or list in this field may be",
        )
        .reach(Self::TAC_BEHAVIOR_AND_CPP, Self::YON_CHECKS)
        .needs_arguments(),
        Rule::new(
            "Min",
            Self::FIELD,
            "refuses a number in this field that is below the given value",
        )
        .reach(Self::TAC_BEHAVIOR_AND_CPP, Self::YON_CHECKS)
        .needs_arguments(),
        Rule::new(
            "Max",
            Self::FIELD,
            "refuses a number in this field that is above the given value",
        )
        .reach(Self::TAC_BEHAVIOR_AND_CPP, Self::YON_CHECKS)
        .needs_arguments(),
        Rule::new(
            "OnEvent",
            Self::METHOD,
            "runs the method when an event of its parameter's type is published",
        )
        .instance()
        .reach(TAC, Self::YON_ROLES)
        .clashes(Self::LIFECYCLE),
        Rule::new(
            "OnStart",
            Self::METHOD,
            "runs the method once when the object starts",
        )
        .bare()
        .instance()
        .reach(TAC, Self::YON_ROLES)
        .clashes(Self::LIFECYCLE),
        Rule::new(
            "OnStop",
            Self::METHOD,
            "runs the method once when the object stops",
        )
        .bare()
        .instance()
        .reach(TAC, Self::YON_ROLES)
        .clashes(Self::LIFECYCLE),
        Rule::retired(
            "Resource",
            "Needs",
            "write @Needs on a field whose name says which object it receives (ADR 0038)",
        ),
        Rule::new(
            "Order",
            Self::METHOD,
            "sets when this listener runs compared with the other listeners of the event",
        )
        .needs_arguments()
        .reach(TAC, Self::YON_ROLES)
        .with(&["OnEvent"]),
        Rule::new(
            "InBackground",
            Self::METHOD,
            "runs this listener without making the publisher wait",
        )
        .bare()
        .reach(TAC, Self::YON_BACKGROUND)
        .with(&["OnEvent"])
        .advice("InBackground listeners require a Yon language that defers them."),
        Rule::retired(
            "Behaviors",
            "",
            "Behaviors is a name that Tachyon generates for its own code; it is not an annotation",
        ),
        Rule::new(
            "RequestRoute",
            Self::TYPE,
            "marks a controller and gives the path that its routes start with",
        )
        .reach(Self::NOWHERE, YON)
        .code(2015),
        Rule::new(
            "ErrorHandler",
            Self::TYPE,
            "marks a class whose methods answer the errors of controllers",
        )
        .reach(Self::NOWHERE, Self::YON_ERRORS)
        .code(2015),
        Rule::new(
            "HandlesError",
            Self::METHOD,
            "answers one kind of error with an HTTP response",
        )
        .bare()
        .reach(Self::NOWHERE, Self::YON_ERRORS)
        .class_needs(&["ErrorHandler"])
        .code(2015),
        Rule::route_verb("GetRoute", "means this method answers GET requests"),
        Rule::route_verb("PostRoute", "means this method answers POST requests"),
        Rule::route_verb("PutRoute", "means this method answers PUT requests"),
        Rule::route_verb("PatchRoute", "means this method answers PATCH requests"),
        Rule::route_verb("DeleteRoute", "means this method answers DELETE requests"),
        Rule::new(
            "ResponseStatus",
            Self::METHOD,
            "sets the HTTP status of a normal answer",
        )
        .needs_arguments()
        .reach(Self::NOWHERE, Self::YON_ERRORS)
        .with(Self::ANSWERS)
        .code(2015),
        Rule::route_parameter("FromPath", "fills this argument from a path variable, such as {id}"),
        Rule::route_parameter("FromQuery", "fills this argument from the query string, after ?"),
        Rule::route_parameter("FromBody", "fills this argument from the request body, read as JSON")
        .one_per_method(),
        Rule::route_parameter("FromHeader", "fills this argument from a request header"),
        Rule::route_parameter("FromCookie", "fills this argument from a cookie"),
        Rule::route_parameter(
            "FromIdentity",
            "fills this argument with the identity of the signed-in caller, as the identity provider returned it",
        )
        .class_needs(&["RequestRoute"])
        .one_per_method(),
        Rule::new(
            "HTTPClient",
            Self::TYPE,
            "marks a class that sends HTTP requests and gives the path they start with",
        )
        .reach(Self::TAC_REMOTE, Self::YON_OUTBOUND),
        Rule::request_verb("GetRequest", "means calling this method sends a GET request"),
        Rule::request_verb("PostRequest", "means calling this method sends a POST request"),
        Rule::request_verb("PutRequest", "means calling this method sends a PUT request"),
        Rule::request_verb("PatchRequest", "means calling this method sends a PATCH request"),
        Rule::request_verb("DeleteRequest", "means calling this method sends a DELETE request"),
        Rule::request_parameter("PathVariable", "puts this argument into the request path"),
        Rule::request_parameter("RequestParam", "adds this argument to the query string of the request"),
        Rule::request_parameter("RequestBody", "sends this argument as the request body, written as JSON")
        .one_per_method(),
        Rule::request_parameter("RequestHeader", "sends this argument as a request header"),
        Rule::new(
            "StreamRequest",
            Self::METHOD,
            "means Tachyon keeps a stream of server-sent events open and passes each message to this method",
        )
        .request()
        .instance()
        .reach(Self::TAC_STREAM, Self::NOWHERE)
        .advice("The browser's JavaScript holds the stream, so write the @StreamRequest client in JavaScript, TypeScript, Go, Ruby, Rust, Dart, Swift, Kotlin, Java, C#, Lua, Python, F# or C++. Each message is also published as a typed event, so a service in any Tac language receives it with @OnEvent on a method whose parameter is the message model"),
        Rule::new(
            "StreamRoute",
            Self::METHOD,
            "means this method answers GET requests with a stream of server-sent events",
        )
        .route()
        .reach(Self::NOWHERE, Self::YON_SOCKET),
        Rule::new(
            "SocketRequest",
            Self::METHOD,
            "means Tachyon keeps a WebSocket open and passes each incoming message to this method",
        )
        .request()
        .instance()
        .reach(Self::TAC_STREAM, Self::NOWHERE)
        .advice("The browser's JavaScript holds the socket, so write the @SocketRequest client in JavaScript, TypeScript, Go, Ruby, Rust, Dart, Swift, Kotlin, Java, C#, Lua, Python, F# or C++. Each message is also published as a typed event, so a service in any Tac language receives it with @OnEvent on a method whose parameter is the message model"),
        Rule::new(
            "SocketRoute",
            Self::TYPE,
            "marks a controller that keeps a WebSocket connection open and gives its path",
        )
        .reach(Self::NOWHERE, Self::YON_SOCKET)
        .clashes(&["RequestRoute"])
        .advice(Self::SOCKET_ADVICE)
        .code(2015),
        Rule::new(
            "OnOpen",
            Self::METHOD,
            "runs the method when a browser opens the socket",
        )
        .socket_event(),
        Rule::new(
            "OnMessage",
            Self::METHOD,
            "runs the method for each message that the browser sends on the socket",
        )
        .socket_event(),
        Rule::new("OnClose", Self::METHOD, "runs the method when the socket closes")
            .socket_event(),
        Rule::new(
            "OnError",
            Self::METHOD,
            "runs the method when opening the socket or handling a message fails",
        )
        .socket_event(),
        Rule::retired(
            "Stream",
            "StreamRoute",
            "write @StreamRoute alone instead of @GetRoute and @Stream, and start the method name with 'stream'. `ty migrate fix --write` converts it",
        ),
    ];

    pub const ACCESS_RULES: &'static [Rule] = &[
        Rule::new(
            "Public",
            Self::TYPE_OR_METHOD,
            "means anyone may call this route, also a caller who is not signed in",
        )
        .access(),
        Rule::new(
            "SignedIn",
            Self::TYPE_OR_METHOD,
            "means any signed-in user may call this route",
        )
        .access(),
        Rule::new(
            "Allow",
            Self::TYPE_OR_METHOD,
            "means only a signed-in user with one of these roles may call this route",
        )
        .access()
        .needs_arguments()
        .argument_advice("@Allow takes one or more role types from server/roles/, for example @Allow(Admin)"),
        Rule::new(
            "IdentityProvider",
            Self::TYPE,
            "marks the one class that reads the request and says who is signed in and which roles they have",
        )
        .bare()
        .reach(Self::NOWHERE, Self::YON_ERRORS)
        .clashes(&["RequestRoute", "SocketRoute", "ErrorHandler"])
        .advice("Put the one @IdentityProvider class in server/controllers/<Name>IdentityProvider.<ext>, with one method identify(request)")
        .code(2015),
    ];

    pub const TIMER_RULES: &'static [Rule] = &[Rule::new(
        "Every",
        Self::METHOD,
        "runs the method again and again in the page after a fixed time, while the page is shown",
    )
    .needs_arguments()
    .instance()
    .reach(TAC, Self::NOWHERE)
    .clashes(Self::LIFECYCLE)
    .advice(
        "@Every runs in the page, in a Tac service. On the server, use @Schedule on a Yon worker",
    )];

    pub fn rules() -> impl Iterator<Item = &'static Rule> {
        Self::ALL
            .iter()
            .chain(Self::ACCESS_RULES)
            .chain(Self::TIMER_RULES)
    }

    pub fn find(written: &str) -> Option<&'static Rule> {
        let plain = written.rsplit("::").next().unwrap_or(written);
        if let Some(rule) = Self::rules().find(|rule| rule.name == plain) {
            return Some(rule);
        }
        let normalized = plain.replace('_', "").to_ascii_lowercase();
        Self::rules().find(|rule| rule.name.to_ascii_lowercase() == normalized)
    }

}
impl Rule {
    pub const fn route_verb(name: &'static str, meaning: &'static str) -> Self {
        Self::new(name, AnnotationRules::METHOD, meaning).route()
    }

    pub const fn request_verb(name: &'static str, meaning: &'static str) -> Self {
        Self::new(name, AnnotationRules::METHOD, meaning).request()
    }

    pub const fn route_parameter(name: &'static str, meaning: &'static str) -> Self {
        Self::new(name, AnnotationRules::PARAMETER, meaning).route_source()
    }

    pub const fn request_parameter(name: &'static str, meaning: &'static str) -> Self {
        Self::new(name, AnnotationRules::PARAMETER, meaning).request_part()
    }

    pub const fn route(self) -> Self {
        Self {
            tac: AnnotationRules::NOWHERE,
            yon: YON,
            instance: true,
            clashes: AnnotationRules::HTTP_METHODS,
            class_needs: &["RequestRoute"],
            code: 2015,
            ..self
        }
    }

    pub const fn access(self) -> Self {
        Self {
            arguments: Arguments::None,
            tac: AnnotationRules::NOWHERE,
            yon: YON,
            clashes: AnnotationRules::ACCESS,
            class_needs: AnnotationRules::GUARDED_CLASSES,
            advice: AnnotationRules::ACCESS_ADVICE,
            code: 2015,
            ..self
        }
    }

    pub const fn socket_event(self) -> Self {
        Self {
            arguments: Arguments::None,
            tac: AnnotationRules::NOWHERE,
            yon: AnnotationRules::YON_SOCKET,
            instance: true,
            clashes: AnnotationRules::SOCKET_EVENTS,
            class_needs: &["SocketRoute"],
            advice: AnnotationRules::SOCKET_ADVICE,
            code: 2015,
            ..self
        }
    }

    pub const fn request(self) -> Self {
        Self {
            tac: AnnotationRules::TAC_REMOTE,
            yon: AnnotationRules::YON_OUTBOUND,
            clashes: AnnotationRules::HTTP_METHODS,
            class_needs: &["HTTPClient"],
            ..self
        }
    }

    pub const fn route_source(self) -> Self {
        Self {
            tac: AnnotationRules::NOWHERE,
            yon: YON,
            clashes: AnnotationRules::ROUTE_SOURCES,
            class_needs: &["RequestRoute", "IdentityProvider"],
            code: 2015,
            ..self
        }
    }

    pub const fn request_part(self) -> Self {
        Self {
            tac: AnnotationRules::TAC_REMOTE,
            yon: AnnotationRules::YON_OUTBOUND,
            clashes: AnnotationRules::REQUEST_PARTS,
            class_needs: &["HTTPClient"],
            ..self
        }
    }
}